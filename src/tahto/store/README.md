# Tahto state kernels

This directory contains the Hara-owned TAHTO-3 through TAHTO-8 state and
provider-boundary logic.

```text
model.hal       canonical identity, limits and immutable state
host.hal        closed Tahto domain effects and result validation
capability.hal  pure-HAL mapping to application-neutral host capabilities
graph.hal       verified manifests, shared closure, roots and dry-run GC
vault.hal       uploads, installation, namespace references, quotas and ranges
history.hal     immutable commits, CAS heads, backups, restore and receipts
transaction.hal atomic verified request-to-metadata commit plans
provider.hal    metadata snapshots, commit receipts and durable-call planning
```

Record shape and signed verification contracts live in:

```text
tahto.protocol.records
tahto.protocol.validate
```

## HAL-only application boundary

Tahto domain behavior is implemented in HAL. The kernels own:

```text
identity and authorization
quotas and offsets
object and history graphs
transaction and recovery plans
closed effect and result validation
```

They never accept a destination path, upstream URL, raw request body, private
key, bearer credential, native library or executable command. Large bodies
remain behind non-zero work-scoped resource handles.

`tahto.store.capability` maps the existing closed Tahto effects to a proposed
generic `hara.blob` host capability. The mapping, request validation, result
matching and translation back into Tahto result records are pure HAL. Hoplite or
another host may implement byte movement, hashing and atomic installation, but
that native layer contains no Tahto upload, quota, graph or authorization rules.

See [`protocol/host-capabilities.md`](../../../protocol/host-capabilities.md).

## Durable metadata migration

TAHTO-7 reducer results contain a tentative next state and a bounded atomic
metadata transaction plan. TAHTO-8 validates canonical state and plan evidence
before exposing one closed durable-store request.

The current `tahto.metadata` identity and the SQLite implementation under
`native/` are migration sources. Tahto issue #17 moves those generic durability
mechanics behind an application-neutral host capability such as `hara.store`.
After compatibility and conformance are complete, Tahto-specific native code is
removed from this repository while the HAL provider validation remains.

Tahto preserves divergent valid commit roots. Applications, not the fabric,
decide whether and how those roots are reconciled.

Normative contracts:

- [`protocol/object-vault.md`](../../../protocol/object-vault.md)
- [`protocol/host-capabilities.md`](../../../protocol/host-capabilities.md)
- [`protocol/history.md`](../../../protocol/history.md)
- [`protocol/transactions.md`](../../../protocol/transactions.md)
- [`protocol/metadata-store.md`](../../../protocol/metadata-store.md)
- [`protocol/metadata-host.md`](../../../protocol/metadata-host.md)
