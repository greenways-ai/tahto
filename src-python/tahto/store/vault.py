"""Composed Tahto object vault."""

from __future__ import annotations

from .base import VaultBase
from .closure import ClosureMixin
from .object import ObjectMixin
from .upload import UploadMixin


class Vault(UploadMixin, ObjectMixin, ClosureMixin, VaultBase):
    """Streaming filesystem/SQLite content-addressed object vault.

    Public operations accept application identifiers, namespace identifiers,
    and validated SHA-256 digests. They never accept a caller-selected storage
    path or upstream URL.
    """

    pass
