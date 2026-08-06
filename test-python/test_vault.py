from __future__ import annotations

import io
import tempfile
import unittest
from pathlib import Path

from tahto.store import (
    ClosureIncomplete,
    DigestMismatch,
    InvalidDigest,
    QuotaExceeded,
    RangeNotSatisfiable,
    UploadConflict,
    Vault,
    VaultConfig,
    sha256_digest,
)


class VaultTest(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.vault = Vault(
            self.root,
            VaultConfig(
                max_object_bytes=1024,
                max_upload_chunk_bytes=4,
                max_manifest_chunks=8,
                max_closure_objects=32,
                range_chunk_bytes=3,
            ),
        )

    def tearDown(self) -> None:
        self.vault.close()
        self.temp.cleanup()

    def test_resumable_stream_is_verified_and_atomically_installed(self) -> None:
        data = b"hello tahto"
        digest = sha256_digest(data)
        upload = self.vault.begin_upload("app.test", "profile.a", digest, len(data))
        partial = self.vault.append_upload(
            upload.upload_id, io.BytesIO(data[:5]), offset=0
        )
        self.assertEqual(5, partial.offset)
        with self.assertRaises(UploadConflict):
            self.vault.append_upload(upload.upload_id, io.BytesIO(b"x"), offset=0)
        complete = self.vault.append_upload(
            upload.upload_id, io.BytesIO(data[5:]), offset=5
        )
        self.assertEqual(len(data), complete.offset)
        info = self.vault.finish_upload(upload.upload_id)
        self.assertEqual(digest, info.digest)
        missing = sha256_digest(b"missing")
        self.assertEqual([missing], self.vault.missing([missing, digest, missing]))
        self.assertTrue(self.vault.has_object(digest))
        self.assertEqual(data, b"".join(self.vault.iter_range(digest)))
        self.assertFalse(any((self.root / "tmp" / "uploads").iterdir()))

    def test_active_uploads_reserve_namespace_quota(self) -> None:
        self.vault.set_quota("app.test", "profile.a", 6)
        first_data = b"1234"
        first = self.vault.begin_upload(
            "app.test", "profile.a", sha256_digest(first_data), len(first_data)
        )
        self.assertEqual(4, self.vault.reserved_upload_bytes("app.test", "profile.a"))
        with self.assertRaises(QuotaExceeded):
            self.vault.begin_upload(
                "app.test", "profile.a", sha256_digest(b"abc"), 3
            )
        self.vault.abort_upload(first.upload_id)
        self.assertEqual(0, self.vault.reserved_upload_bytes("app.test", "profile.a"))

    def test_put_file_streams_through_the_upload_protocol(self) -> None:
        path = self.root / "source.bin"
        path.write_bytes(b"streamed-file")
        info = self.vault.put_file("app.test", "profile.a", path)
        self.assertEqual(sha256_digest(b"streamed-file"), info.digest)
        self.assertEqual(b"streamed-file", b"".join(self.vault.iter_range(info.digest)))

    def test_digest_mismatch_never_installs_object(self) -> None:
        data = b"tampered"
        expected = sha256_digest(b"expected")
        upload = self.vault.begin_upload("app.test", "profile.a", expected, len(data))
        self.vault.append_upload(upload.upload_id, io.BytesIO(data), offset=0)
        with self.assertRaises(DigestMismatch):
            self.vault.finish_upload(upload.upload_id)
        self.assertFalse(self.vault.has_object(expected))

    def test_quota_counts_logical_namespace_references_once(self) -> None:
        self.vault.set_quota("app.test", "profile.a", 6)
        first = self.vault.put_bytes("app.test", "profile.a", b"1234")
        self.vault.reference_object("app.test", "profile.a", first.digest)
        self.assertEqual(4, self.vault.usage("app.test", "profile.a"))
        with self.assertRaises(QuotaExceeded):
            self.vault.put_bytes("app.test", "profile.a", b"abc")
        # Global object deduplication does not bypass another namespace's quota.
        self.vault.set_quota("app.test", "profile.b", 4)
        self.vault.reference_object("app.test", "profile.b", first.digest)
        self.assertEqual(4, self.vault.usage("app.test", "profile.b"))

    def test_explicit_verification_rejects_tampered_installed_bytes(self) -> None:
        info = self.vault.put_bytes("app.test", "profile.a", b"verified")
        path = self.vault._object_path(info.digest)
        path.write_bytes(b"tampered")
        with self.assertRaises(DigestMismatch):
            self.vault.verify_object(info.digest)

    def test_range_reads_are_bounded(self) -> None:
        info = self.vault.put_bytes("app.test", "profile.a", b"0123456789")
        self.assertEqual(
            b"34567",
            b"".join(
                self.vault.iter_range(info.digest, start=3, end_exclusive=8)
            ),
        )
        with self.assertRaises(RangeNotSatisfiable):
            list(self.vault.iter_range(info.digest, start=8, end_exclusive=11))

    def test_chunk_manifest_registers_and_verifies_complete_closure(self) -> None:
        chunks = [
            self.vault.put_bytes(
                "app.test", "profile.a", value, role="staging"
            ).digest
            for value in (b"abc", b"def", b"ghi")
        ]
        manifest = self.vault.create_chunk_manifest(
            "app.test", "profile.a", chunks
        )
        self.assertEqual(
            set([manifest.digest, *chunks]),
            set(self.vault.verify_closure([manifest.digest])),
        )
        self.vault.pin_root(
            "backup", "app.test", "profile.a", "daily", manifest.digest
        )
        repeated = self.vault.create_chunk_manifest(
            "app.test", "profile.a", [chunks[0], chunks[0]]
        )
        self.assertEqual(
            {repeated.digest, chunks[0]},
            set(self.vault.verify_closure([repeated.digest])),
        )
        with self.assertRaises(ClosureIncomplete):
            self.vault.register_edges(
                manifest.digest,
                ["sha256:" + "f" * 64],
            )

    def test_garbage_collection_preserves_pinned_closure(self) -> None:
        child = self.vault.put_bytes(
            "app.test", "profile.a", b"child", role="staging"
        )
        parent = self.vault.create_chunk_manifest(
            "app.test", "profile.a", [child.digest]
        )
        orphan = self.vault.put_bytes("app.test", "profile.a", b"orphan")
        self.vault.pin_root(
            "backup", "app.test", "profile.a", "daily", parent.digest
        )
        for digest in (child.digest, parent.digest, orphan.digest):
            self.vault.release_object("app.test", "profile.a", digest)
        self.assertEqual((orphan.digest,), self.vault.collect_garbage())
        self.assertEqual((orphan.digest,), self.vault.collect_garbage(dry_run=False))
        self.assertFalse(self.vault.has_object(orphan.digest))
        self.assertTrue(self.vault.has_object(parent.digest))
        self.assertTrue(self.vault.has_object(child.digest))

    def test_callers_cannot_select_object_paths(self) -> None:
        with self.assertRaises(InvalidDigest):
            self.vault.object_info("../../etc/passwd")
        with self.assertRaises(InvalidDigest):
            self.vault.missing(["sha256:" + "A" * 64])


if __name__ == "__main__":
    unittest.main()
