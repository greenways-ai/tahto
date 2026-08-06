"""Bounded chunk manifests, closure pins, and safe garbage collection."""

from __future__ import annotations

import json
from collections import deque
from typing import Iterable, Sequence

from .model import (
    _ALLOWED_ROOT_KINDS,
    _MANIFEST_PROTOCOL,
    _now,
    _validate_digest,
    ClosureIncomplete,
    ManifestError,
    ObjectInfo,
)


class ClosureMixin:
    def register_edges(self, parent_digest: str, children: Sequence[str]) -> None:
        parent_digest = _validate_digest(parent_digest)
        self.object_info(parent_digest)
        if len(children) > self.config.max_manifest_chunks:
            raise ManifestError(
                f"closure edge count exceeds {self.config.max_manifest_chunks}"
            )
        normalized = [_validate_digest(child) for child in children]
        missing = self.missing(normalized)
        if missing:
            raise ClosureIncomplete(missing)
        with self._transaction():
            self.db.execute(
                "DELETE FROM closure_edges WHERE parent_digest = ?", (parent_digest,)
            )
            self.db.executemany(
                """
                INSERT INTO closure_edges(parent_digest, child_digest, ordinal)
                VALUES (?, ?, ?)
                """,
                [(parent_digest, child, index) for index, child in enumerate(normalized)],
            )

    def create_chunk_manifest(
        self,
        application: str,
        namespace: str,
        chunks: Sequence[str],
        *,
        media_type: str = "application/octet-stream",
        role: str = "root",
    ) -> ObjectInfo:
        if not chunks or len(chunks) > self.config.max_manifest_chunks:
            raise ManifestError(
                f"manifest requires 1..{self.config.max_manifest_chunks} chunks"
            )
        normalized = [_validate_digest(digest) for digest in chunks]
        missing = self.missing(normalized)
        if missing:
            raise ClosureIncomplete(missing)
        entries = []
        total = 0
        for digest in normalized:
            info = self.object_info(digest)
            entries.append({"digest": digest, "size": info.size})
            total += info.size
        document = {
            "protocol": _MANIFEST_PROTOCOL,
            "mediaType": media_type,
            "totalSize": total,
            "chunks": entries,
        }
        encoded = json.dumps(
            document, sort_keys=True, separators=(",", ":"), ensure_ascii=False
        ).encode("utf-8")
        manifest = self.put_bytes(
            application,
            namespace,
            encoded,
            "application/vnd.tahto.chunk-manifest+json",
            role=role,
        )
        self.register_edges(manifest.digest, normalized)
        return manifest

    def verify_closure(
        self, roots: Iterable[str], *, verify_bytes: bool = False
    ) -> tuple[str, ...]:
        queue = deque(_validate_digest(root) for root in roots)
        visited: set[str] = set()
        missing: list[str] = []
        while queue:
            digest = queue.popleft()
            if digest in visited:
                continue
            if len(visited) >= self.config.max_closure_objects:
                raise ManifestError(
                    f"closure exceeds {self.config.max_closure_objects} objects"
                )
            visited.add(digest)
            if not self.has_object(digest):
                missing.append(digest)
                continue
            if verify_bytes:
                self.verify_object(digest)
            rows = self.db.execute(
                """
                SELECT child_digest FROM closure_edges
                WHERE parent_digest = ? ORDER BY ordinal
                """,
                (digest,),
            ).fetchall()
            queue.extend(row["child_digest"] for row in rows)
        if missing:
            raise ClosureIncomplete(sorted(missing))
        return tuple(sorted(visited))

    def pin_root(
        self,
        kind: str,
        application: str,
        namespace: str,
        name: str,
        digest: str,
    ) -> tuple[str, ...]:
        if kind not in _ALLOWED_ROOT_KINDS:
            raise ValueError(f"unsupported root kind: {kind}")
        closure = self.verify_closure([digest])
        with self._transaction():
            self.db.execute(
                """
                INSERT INTO roots(kind, application, namespace, name, digest, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                ON CONFLICT(kind, application, namespace, name)
                DO UPDATE SET digest = excluded.digest, created_at = excluded.created_at
                """,
                (kind, application, namespace, name, digest, _now()),
            )
        return closure

    def unpin_root(self, kind: str, application: str, namespace: str, name: str) -> None:
        with self._transaction():
            self.db.execute(
                """
                DELETE FROM roots
                WHERE kind = ? AND application = ? AND namespace = ? AND name = ?
                """,
                (kind, application, namespace, name),
            )

    def collect_garbage(self, *, dry_run: bool = True) -> tuple[str, ...]:
        rows = self.db.execute(
            """
            SELECT digest FROM roots
            UNION
            SELECT digest FROM object_refs
            """
        ).fetchall()
        roots = [row["digest"] for row in rows]
        reachable = set(self.verify_closure(roots)) if roots else set()
        all_objects = {
            row["digest"]
            for row in self.db.execute("SELECT digest FROM objects").fetchall()
        }
        garbage = tuple(sorted(all_objects - reachable))
        if dry_run or not garbage:
            return garbage
        placeholders = ",".join("?" for _ in garbage)
        with self._transaction():
            # Remove every outgoing edge first so deletion order cannot make one
            # garbage child appear referenced by another garbage parent.
            self.db.execute(
                f"DELETE FROM closure_edges WHERE parent_digest IN ({placeholders})",
                garbage,
            )
            self.db.execute(
                f"DELETE FROM objects WHERE digest IN ({placeholders})", garbage
            )
        for digest in garbage:
            # A failed unlink leaves an unindexed immutable file, which is a
            # storage leak rather than data loss. A later repair/GC pass can
            # remove it safely.
            self._object_path(digest).unlink(missing_ok=True)
        return garbage
