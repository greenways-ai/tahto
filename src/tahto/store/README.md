# Store boundary

TAHTO-3 provides the first operational Tahto object vault in `src-python/tahto/store/`.

The split is deliberate:

```text
Hara / Hoplite control plane
  authorizes application operations and returns small records

Native object data plane
  streams bytes, verifies digests, installs immutable objects,
  serves ranges, enforces quotas, and traverses closures
```

Large object bytes must not become ordinary Hara values. The Python library is dependency-free and supplies the node-local filesystem/SQLite implementation plus an operator CLI. HOPLITE-1 supplies the native request and response adapter contract that will bind HTTP transport to this vault.

Core storage remains application-neutral. Its public identifiers are application, namespace, server-assigned upload ID, and validated SHA-256 digest. It never accepts a caller-selected destination path or proxy upstream.

See [`protocol/object-vault.md`](../../../protocol/object-vault.md) for the normative storage profile.
