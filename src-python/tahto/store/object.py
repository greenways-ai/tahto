"""Object negotiation, verification, ranges, and namespace references."""

from __future__ import annotations

from typing import Iterable, Iterator

from .model import (
    _now,
    _validate_digest,
    DigestMismatch,
    ObjectInfo,
    ObjectNotFound,
    RangeNotSatisfiable,
)


class ObjectMixin:
    def has_object(self, digest: str) -> bool:
        digest = _validate_digest(digest)
        row = self.db.execute(
            "SELECT 1 FROM objects WHERE digest = ?", (digest,)
        ).fetchone()
        return row is not None and self._object_path(digest).is_file()

    def missing(self, digests: Iterable[str]) -> list[str]:
        output: list[str] = []
        seen: set[str] = set()
        for digest in digests:
            digest = _validate_digest(digest)
            if digest in seen:
                continue
            seen.add(digest)
            if not self.has_object(digest):
                output.append(digest)
        return output

    def object_info(self, digest: str) -> ObjectInfo:
        digest = _validate_digest(digest)
        row = self.db.execute(
            "SELECT * FROM objects WHERE digest = ?", (digest,)
        ).fetchone()
        if row is None or not self._object_path(digest).is_file():
            raise ObjectNotFound(digest)
        return ObjectInfo(
            digest=row["digest"],
            size=int(row["size"]),
            media_type=row["media_type"],
            created_at=int(row["created_at"]),
            verified_at=int(row["verified_at"]),
        )

    def verify_object(self, digest: str) -> ObjectInfo:
        info = self.object_info(digest)
        observed_digest, observed_size = self._hash_file(self._object_path(digest))
        if observed_digest != digest or observed_size != info.size:
            raise DigestMismatch(
                f"object verification failed for {digest}: "
                f"observed {observed_digest} with {observed_size} bytes"
            )
        with self._transaction():
            self.db.execute(
                "UPDATE objects SET verified_at = ? WHERE digest = ?",
                (_now(), digest),
            )
        return self.object_info(digest)

    def iter_range(
        self,
        digest: str,
        *,
        start: int = 0,
        end_exclusive: int | None = None,
        chunk_bytes: int | None = None,
    ) -> Iterator[bytes]:
        info = self.object_info(digest)
        end = info.size if end_exclusive is None else end_exclusive
        if start < 0 or end < start or start > info.size or end > info.size:
            raise RangeNotSatisfiable(
                f"range [{start}, {end}) is invalid for object length {info.size}"
            )
        chunk_bytes = chunk_bytes or self.config.range_chunk_bytes
        if chunk_bytes <= 0 or chunk_bytes > self.config.max_upload_chunk_bytes:
            raise ValueError("invalid range chunk size")
        remaining = end - start
        with self._object_path(digest).open("rb", buffering=0) as handle:
            handle.seek(start)
            while remaining:
                chunk = handle.read(min(chunk_bytes, remaining))
                if not chunk:
                    raise DigestMismatch(f"object ended early while reading: {digest}")
                remaining -= len(chunk)
                yield chunk

    def reference_object(
        self,
        application: str,
        namespace: str,
        digest: str,
        *,
        role: str = "root",
    ) -> None:
        if role not in {"root", "staging"}:
            raise ValueError("role must be 'root' or 'staging'")
        info = self.object_info(digest)
        with self._transaction():
            self._check_quota_for_reference(
                application, namespace, digest, info.size
            )
            self.db.execute(
                """
                INSERT INTO object_refs(application, namespace, digest, role, created_at)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(application, namespace, digest)
                DO UPDATE SET role = excluded.role
                """,
                (application, namespace, digest, role, _now()),
            )

    def release_object(self, application: str, namespace: str, digest: str) -> None:
        digest = _validate_digest(digest)
        with self._transaction():
            self.db.execute(
                """
                DELETE FROM object_refs
                WHERE application = ? AND namespace = ? AND digest = ?
                """,
                (application, namespace, digest),
            )
