"""Types and validated identifiers for the Tahto object vault."""

from __future__ import annotations

import hashlib
import re
import time
from dataclasses import dataclass
from typing import Sequence

_DIGEST_RE = re.compile(r"^sha256:([0-9a-f]{64})$")
_UPLOAD_RE = re.compile(r"^[0-9a-f]{32}$")
_ALLOWED_ROOT_KINDS = frozenset({"application", "head", "backup", "retention"})
_MANIFEST_PROTOCOL = "tahto.chunk-manifest/1"


class VaultError(RuntimeError):
    """Base class for deterministic vault failures."""


class InvalidDigest(VaultError):
    pass


class ObjectNotFound(VaultError):
    pass


class UploadNotFound(VaultError):
    pass


class UploadConflict(VaultError):
    pass


class DigestMismatch(VaultError):
    pass


class QuotaExceeded(VaultError):
    pass


class RangeNotSatisfiable(VaultError):
    pass


class ManifestError(VaultError):
    pass


class ClosureIncomplete(VaultError):
    def __init__(self, missing: Sequence[str]):
        self.missing = tuple(missing)
        super().__init__("closure is incomplete: " + ", ".join(self.missing))


def sha256_digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def _validate_digest(value: str) -> str:
    if not isinstance(value, str) or _DIGEST_RE.fullmatch(value) is None:
        raise InvalidDigest(f"invalid SHA-256 digest: {value!r}")
    return value


def _now() -> int:
    return int(time.time())


@dataclass(frozen=True)
class VaultConfig:
    max_object_bytes: int = 8 * 1024 * 1024 * 1024
    max_upload_chunk_bytes: int = 8 * 1024 * 1024
    max_manifest_chunks: int = 65_536
    max_closure_objects: int = 1_000_000
    range_chunk_bytes: int = 1024 * 1024

    def validate(self) -> "VaultConfig":
        for name, value in vars(self).items():
            if not isinstance(value, int) or value <= 0:
                raise ValueError(f"{name} must be a positive integer")
        if self.max_upload_chunk_bytes > self.max_object_bytes:
            raise ValueError("max_upload_chunk_bytes cannot exceed max_object_bytes")
        return self


@dataclass(frozen=True)
class ObjectInfo:
    digest: str
    size: int
    media_type: str | None
    created_at: int
    verified_at: int


@dataclass(frozen=True)
class UploadInfo:
    upload_id: str
    application: str
    namespace: str
    expected_digest: str
    expected_size: int
    media_type: str | None
    offset: int
    created_at: int
    updated_at: int
