"""Resumable uploads and atomic object installation."""

from __future__ import annotations

import fcntl
import io
import os
import secrets
from pathlib import Path
from typing import BinaryIO

from .model import (
    _now,
    _validate_digest,
    DigestMismatch,
    ObjectInfo,
    QuotaExceeded,
    UploadConflict,
    UploadInfo,
    UploadNotFound,
    sha256_digest,
)


class UploadMixin:
    def begin_upload(
        self,
        application: str,
        namespace: str,
        expected_digest: str,
        expected_size: int,
        media_type: str | None = None,
        *,
        upload_id: str | None = None,
    ) -> UploadInfo:
        expected_digest = _validate_digest(expected_digest)
        if expected_size < 0 or expected_size > self.config.max_object_bytes:
            raise QuotaExceeded(
                f"object size {expected_size} exceeds maximum {self.config.max_object_bytes}"
            )
        self._check_quota_for_upload(
            application, namespace, expected_digest, expected_size
        )
        upload_id = upload_id or secrets.token_hex(16)
        path = self._upload_path(upload_id)
        now = _now()
        try:
            descriptor = os.open(path, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        except FileExistsError as error:
            raise UploadConflict(f"upload {upload_id} already exists") from error
        else:
            os.close(descriptor)
        try:
            with self._transaction():
                self._check_quota_for_upload(
                    application, namespace, expected_digest, expected_size
                )
                self.db.execute(
                    """
                    INSERT INTO uploads(
                      upload_id, application, namespace, expected_digest,
                      expected_size, media_type, offset, temp_name,
                      created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?, ?)
                    """,
                    (
                        upload_id,
                        application,
                        namespace,
                        expected_digest,
                        expected_size,
                        media_type,
                        path.name,
                        now,
                        now,
                    ),
                )
        except Exception:
            path.unlink(missing_ok=True)
            raise
        return self.upload_info(upload_id)

    def upload_info(self, upload_id: str) -> UploadInfo:
        self._upload_path(upload_id)
        row = self.db.execute(
            "SELECT * FROM uploads WHERE upload_id = ?", (upload_id,)
        ).fetchone()
        if row is None:
            raise UploadNotFound(f"upload not found: {upload_id}")
        return UploadInfo(
            upload_id=row["upload_id"],
            application=row["application"],
            namespace=row["namespace"],
            expected_digest=row["expected_digest"],
            expected_size=int(row["expected_size"]),
            media_type=row["media_type"],
            offset=int(row["offset"]),
            created_at=int(row["created_at"]),
            updated_at=int(row["updated_at"]),
        )

    def append_upload(
        self,
        upload_id: str,
        source: BinaryIO,
        *,
        offset: int,
        max_bytes: int | None = None,
    ) -> UploadInfo:
        path = self._upload_path(upload_id)
        with path.open("r+b", buffering=0) as handle:
            # Tahto currently targets Homebrew-supported macOS/Linux nodes. The
            # per-upload advisory lock keeps concurrent processes from writing
            # the same temporary object before SQLite's offset CAS can reject
            # a stale caller.
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX)
            try:
                info = self.upload_info(upload_id)
                if offset != info.offset:
                    raise UploadConflict(
                        f"upload {upload_id} expects offset {info.offset}, received {offset}"
                    )
                actual_size = path.stat().st_size
                if actual_size != info.offset:
                    # A crash can leave bytes ahead of the committed SQLite
                    # offset. SQLite is authoritative for resumability.
                    handle.truncate(info.offset)
                    handle.flush()
                    os.fsync(handle.fileno())
                remaining_object = info.expected_size - info.offset
                allowance = (
                    remaining_object
                    if max_bytes is None
                    else min(remaining_object, max_bytes)
                )
                written = 0
                handle.seek(info.offset)
                while written < allowance:
                    request = min(
                        self.config.max_upload_chunk_bytes, allowance - written
                    )
                    chunk = source.read(request)
                    if not chunk:
                        break
                    if not isinstance(chunk, (bytes, bytearray, memoryview)):
                        raise TypeError("upload source must return bytes")
                    if len(chunk) > request:
                        raise UploadConflict(
                            "upload source returned more bytes than requested"
                        )
                    handle.write(chunk)
                    written += len(chunk)
                handle.flush()
                os.fsync(handle.fileno())
                new_offset = info.offset + written
                with self._transaction():
                    cursor = self.db.execute(
                        """
                        UPDATE uploads SET offset = ?, updated_at = ?
                        WHERE upload_id = ? AND offset = ?
                        """,
                        (new_offset, _now(), upload_id, info.offset),
                    )
                    if cursor.rowcount != 1:
                        raise UploadConflict(
                            f"upload {upload_id} advanced concurrently"
                        )
            finally:
                fcntl.flock(handle.fileno(), fcntl.LOCK_UN)
        return self.upload_info(upload_id)

    def finish_upload(self, upload_id: str, *, role: str = "root") -> ObjectInfo:
        if role not in {"root", "staging"}:
            raise ValueError("role must be 'root' or 'staging'")
        info = self.upload_info(upload_id)
        if info.offset != info.expected_size:
            raise UploadConflict(
                f"upload {upload_id} is incomplete: {info.offset}/{info.expected_size}"
            )
        temporary = self._upload_path(upload_id)
        destination = self._object_path(info.expected_digest)
        source = temporary if temporary.exists() else destination
        if not source.exists():
            raise UploadConflict(
                f"upload {upload_id} has neither temporary nor installed bytes"
            )
        digest, size = self._hash_file(source)
        if size != info.expected_size:
            raise UploadConflict(
                f"upload file size changed: expected {info.expected_size}, observed {size}"
            )
        if digest != info.expected_digest:
            raise DigestMismatch(
                f"upload digest mismatch: expected {info.expected_digest}, observed {digest}"
            )

        now = _now()
        with self._transaction():
            self._check_quota_for_reference(
                info.application, info.namespace, digest, size
            )
            destination.parent.mkdir(parents=True, exist_ok=True)
            if destination.exists():
                existing_digest, existing_size = self._hash_file(destination)
                if existing_digest != digest or existing_size != size:
                    raise DigestMismatch(f"installed object is corrupt: {digest}")
                temporary.unlink(missing_ok=True)
            else:
                os.replace(temporary, destination)
                self._fsync_directory(destination.parent)
            self.db.execute(
                """
                INSERT INTO objects(digest, size, media_type, created_at, verified_at)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(digest) DO UPDATE SET
                  verified_at = excluded.verified_at,
                  media_type = COALESCE(objects.media_type, excluded.media_type)
                """,
                (digest, size, info.media_type, now, now),
            )
            self.db.execute(
                """
                INSERT INTO object_refs(application, namespace, digest, role, created_at)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(application, namespace, digest)
                DO UPDATE SET role = CASE
                  WHEN object_refs.role = 'root' THEN 'root'
                  ELSE excluded.role END
                """,
                (info.application, info.namespace, digest, role, now),
            )
            self.db.execute("DELETE FROM uploads WHERE upload_id = ?", (upload_id,))
        return self.object_info(digest)

    def put_bytes(
        self,
        application: str,
        namespace: str,
        data: bytes,
        media_type: str | None = None,
        *,
        role: str = "root",
    ) -> ObjectInfo:
        digest = sha256_digest(data)
        session = self.begin_upload(
            application, namespace, digest, len(data), media_type
        )
        try:
            self.append_upload(session.upload_id, io.BytesIO(data), offset=0)
            return self.finish_upload(session.upload_id, role=role)
        except Exception:
            try:
                self.abort_upload(session.upload_id)
            except UploadNotFound:
                pass
            raise

    def put_file(
        self,
        application: str,
        namespace: str,
        path: os.PathLike[str] | str,
        media_type: str | None = None,
        *,
        role: str = "root",
    ) -> ObjectInfo:
        source_path = Path(path)
        digest, size = self._hash_file(source_path)
        session = self.begin_upload(
            application, namespace, digest, size, media_type
        )
        try:
            with source_path.open("rb", buffering=0) as source:
                offset = 0
                while offset < size:
                    info = self.append_upload(
                        session.upload_id, source, offset=offset
                    )
                    if info.offset == offset:
                        raise UploadConflict("file source ended before its measured size")
                    offset = info.offset
            return self.finish_upload(session.upload_id, role=role)
        except Exception:
            # Preserve digest mismatches for inspection; abort only when the
            # session still owns a temporary file and no verified install exists.
            try:
                self.abort_upload(session.upload_id)
            except UploadNotFound:
                pass
            raise

    def abort_upload(self, upload_id: str) -> None:
        self.upload_info(upload_id)
        path = self._upload_path(upload_id)
        with self._transaction():
            self.db.execute("DELETE FROM uploads WHERE upload_id = ?", (upload_id,))
        path.unlink(missing_ok=True)
