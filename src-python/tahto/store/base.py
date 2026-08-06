"""Filesystem and SQLite foundation for the object vault."""

from __future__ import annotations

import hashlib
import os
import sqlite3
from contextlib import contextmanager
from pathlib import Path

from .model import (
    _DIGEST_RE,
    _UPLOAD_RE,
    _now,
    _validate_digest,
    QuotaExceeded,
    UploadNotFound,
    VaultConfig,
)


class VaultBase:
    def __init__(self, root: os.PathLike[str] | str, config: VaultConfig | None = None):
        self.root = Path(root).expanduser().resolve()
        self.config = (config or VaultConfig()).validate()
        self.object_root = self.root / "objects" / "sha256"
        self.upload_root = self.root / "tmp" / "uploads"
        self.db_path = self.root / "metadata.sqlite"
        self.object_root.mkdir(parents=True, exist_ok=True)
        self.upload_root.mkdir(parents=True, exist_ok=True)
        self.db = sqlite3.connect(self.db_path, isolation_level=None, timeout=30)
        self.db.row_factory = sqlite3.Row
        self.db.execute("PRAGMA foreign_keys = ON")
        self.db.execute("PRAGMA journal_mode = WAL")
        self.db.execute("PRAGMA synchronous = FULL")
        self._migrate()

    def close(self) -> None:
        self.db.close()

    def __enter__(self) -> "VaultBase":
        return self

    def __exit__(self, exc_type, exc, traceback) -> None:
        self.close()

    @contextmanager
    def _transaction(self):
        self.db.execute("BEGIN IMMEDIATE")
        try:
            yield
        except Exception:
            self.db.execute("ROLLBACK")
            raise
        else:
            self.db.execute("COMMIT")

    def _migrate(self) -> None:
        self.db.executescript(
            """
            BEGIN IMMEDIATE;
            CREATE TABLE IF NOT EXISTS objects (
              digest TEXT PRIMARY KEY,
              size INTEGER NOT NULL CHECK (size >= 0),
              media_type TEXT,
              created_at INTEGER NOT NULL,
              verified_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS object_refs (
              application TEXT NOT NULL,
              namespace TEXT NOT NULL,
              digest TEXT NOT NULL REFERENCES objects(digest) ON DELETE CASCADE,
              role TEXT NOT NULL CHECK (role IN ('root', 'staging')),
              created_at INTEGER NOT NULL,
              PRIMARY KEY (application, namespace, digest)
            );
            CREATE TABLE IF NOT EXISTS quotas (
              application TEXT NOT NULL,
              namespace TEXT NOT NULL,
              max_bytes INTEGER NOT NULL CHECK (max_bytes >= 0),
              PRIMARY KEY (application, namespace)
            );
            CREATE TABLE IF NOT EXISTS uploads (
              upload_id TEXT PRIMARY KEY,
              application TEXT NOT NULL,
              namespace TEXT NOT NULL,
              expected_digest TEXT NOT NULL,
              expected_size INTEGER NOT NULL CHECK (expected_size >= 0),
              media_type TEXT,
              offset INTEGER NOT NULL CHECK (offset >= 0),
              temp_name TEXT NOT NULL UNIQUE,
              created_at INTEGER NOT NULL,
              updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS closure_edges (
              parent_digest TEXT NOT NULL REFERENCES objects(digest) ON DELETE CASCADE,
              child_digest TEXT NOT NULL REFERENCES objects(digest) ON DELETE RESTRICT,
              ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
              PRIMARY KEY (parent_digest, ordinal)
            );
            CREATE TABLE IF NOT EXISTS roots (
              kind TEXT NOT NULL,
              application TEXT NOT NULL,
              namespace TEXT NOT NULL,
              name TEXT NOT NULL,
              digest TEXT NOT NULL REFERENCES objects(digest) ON DELETE RESTRICT,
              created_at INTEGER NOT NULL,
              PRIMARY KEY (kind, application, namespace, name)
            );
            CREATE INDEX IF NOT EXISTS object_refs_digest ON object_refs(digest);
            CREATE INDEX IF NOT EXISTS closure_edges_child ON closure_edges(child_digest);
            CREATE INDEX IF NOT EXISTS roots_digest ON roots(digest);
            COMMIT;
            """
        )

    def _object_path(self, digest: str) -> Path:
        match = _DIGEST_RE.fullmatch(_validate_digest(digest))
        assert match is not None
        hex_digest = match.group(1)
        return self.object_root / hex_digest[:2] / hex_digest[2:]

    def _upload_path(self, upload_id: str) -> Path:
        if _UPLOAD_RE.fullmatch(upload_id) is None:
            raise UploadNotFound(f"invalid upload id: {upload_id!r}")
        return self.upload_root / f"{upload_id}.part"

    def set_quota(self, application: str, namespace: str, max_bytes: int) -> None:
        if max_bytes < 0:
            raise ValueError("quota cannot be negative")
        with self._transaction():
            self.db.execute(
                """
                INSERT INTO quotas(application, namespace, max_bytes)
                VALUES (?, ?, ?)
                ON CONFLICT(application, namespace)
                DO UPDATE SET max_bytes = excluded.max_bytes
                """,
                (application, namespace, max_bytes),
            )
            committed = self.usage(application, namespace)
            reserved = self.reserved_upload_bytes(application, namespace)
            if committed + reserved > max_bytes:
                raise QuotaExceeded(
                    f"usage {committed} plus upload reservations {reserved} exceeds "
                    f"quota {max_bytes} for {application}/{namespace}"
                )

    def usage(self, application: str, namespace: str) -> int:
        row = self.db.execute(
            """
            SELECT COALESCE(SUM(objects.size), 0) AS used
            FROM object_refs
            JOIN objects USING (digest)
            WHERE application = ? AND namespace = ?
            """,
            (application, namespace),
        ).fetchone()
        return int(row["used"])

    def _quota(self, application: str, namespace: str) -> int | None:
        row = self.db.execute(
            "SELECT max_bytes FROM quotas WHERE application = ? AND namespace = ?",
            (application, namespace),
        ).fetchone()
        return None if row is None else int(row["max_bytes"])

    def reserved_upload_bytes(self, application: str, namespace: str) -> int:
        row = self.db.execute(
            """
            SELECT COALESCE(SUM(expected_size), 0) AS reserved
            FROM uploads
            WHERE application = ? AND namespace = ?
            """,
            (application, namespace),
        ).fetchone()
        return int(row["reserved"])

    def _check_quota_for_reference(
        self, application: str, namespace: str, digest: str, size: int
    ) -> None:
        exists = self.db.execute(
            """
            SELECT 1 FROM object_refs
            WHERE application = ? AND namespace = ? AND digest = ?
            """,
            (application, namespace, digest),
        ).fetchone()
        if exists is not None:
            return
        quota = self._quota(application, namespace)
        if quota is None:
            return
        attempted = self.usage(application, namespace) + size
        if attempted > quota:
            raise QuotaExceeded(
                f"quota {quota} exceeded for {application}/{namespace}: {attempted}"
            )

    def _check_quota_for_upload(
        self, application: str, namespace: str, digest: str, size: int
    ) -> None:
        if self.db.execute(
            """
            SELECT 1 FROM object_refs
            WHERE application = ? AND namespace = ? AND digest = ?
            """,
            (application, namespace, digest),
        ).fetchone() is not None:
            return
        quota = self._quota(application, namespace)
        if quota is None:
            return
        attempted = (
            self.usage(application, namespace)
            + self.reserved_upload_bytes(application, namespace)
            + size
        )
        if attempted > quota:
            raise QuotaExceeded(
                f"quota {quota} exceeded by upload reservation for "
                f"{application}/{namespace}: {attempted}"
            )

    def _hash_file(self, path: Path) -> tuple[str, int]:
        hasher = hashlib.sha256()
        size = 0
        with path.open("rb", buffering=0) as handle:
            while True:
                chunk = handle.read(self.config.max_upload_chunk_bytes)
                if not chunk:
                    break
                size += len(chunk)
                if size > self.config.max_object_bytes:
                    raise QuotaExceeded(
                        f"object exceeds maximum {self.config.max_object_bytes}"
                    )
                hasher.update(chunk)
        return "sha256:" + hasher.hexdigest(), size

    @staticmethod
    def _fsync_directory(directory: Path) -> None:
        descriptor = os.open(directory, os.O_RDONLY)
        try:
            os.fsync(descriptor)
        finally:
            os.close(descriptor)
