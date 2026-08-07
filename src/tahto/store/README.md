# Tahto state kernels

This directory contains the Hara-owned TAHTO-3 and TAHTO-4 transition logic.

```text
model.hal    canonical identity, limits and immutable state
host.hal     closed opaque-handle byte effect/result boundary
graph.hal    verified manifests, shared closure, roots and dry-run GC
vault.hal    uploads, installation, namespace references, quotas and ranges
history.hal  immutable commits, CAS heads, backups, restore and receipts
```

Record shape and signed verification contracts live in:

```text
tahto.protocol.records
tahto.protocol.validate
```

The kernels never accept a destination path, upstream URL, raw request body,
private key, bearer credential or native command. Large bodies remain behind
non-zero Hoplite resource handles. Signed history records enter only through a
`tahto.record-verification/1` proof issued by a trusted installed provider.

Reducer results contain a tentative next `:state` plus bounded effects or
verification requests. A node executor commits the next metadata state only
after the corresponding provider operation succeeds. Head comparison, head
replacement and GC-root replacement must be one durable metadata transaction.

Tahto preserves divergent valid commit roots. Applications, not the fabric,
decide whether and how those roots are reconciled.

Normative contracts:

- [`protocol/object-vault.md`](../../../protocol/object-vault.md)
- [`protocol/history.md`](../../../protocol/history.md)
