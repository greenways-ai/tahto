# Tahto state kernels

This directory contains the Hara-owned TAHTO-3 through TAHTO-8 state and provider-boundary logic.

```text
model.hal       canonical identity, limits and immutable state
host.hal        closed opaque-handle byte effect/result boundary
graph.hal       verified manifests, shared closure, roots and dry-run GC
vault.hal       uploads, installation, namespace references, quotas and ranges
history.hal     immutable commits, CAS heads, backups, restore and receipts
transaction.hal atomic verified request-to-metadata commit plans
provider.hal    fixed metadata host calls, snapshots and commit receipts
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

TAHTO-7 reducer results contain a tentative next state and a bounded atomic
metadata transaction plan. TAHTO-8 validates canonical state and plan evidence
before exposing one call to the fixed `tahto.metadata` host service using the
`tahto-metadata-store/1` native ABI. Requests cannot select a provider package,
database path, credential, upstream, callback or executable command.

The SQLite provider under `native/provider/metadata-sqlite/` implements the
native compare-and-swap and receipt laws. The Hoplite host registry and HTA
encoder/decoder remain explicit integration work, so the repository does not
claim that metadata persistence is wired into the running node.

Tahto preserves divergent valid commit roots. Applications, not the fabric,
decide whether and how those roots are reconciled.

Normative contracts:

- [`protocol/object-vault.md`](../../../protocol/object-vault.md)
- [`protocol/history.md`](../../../protocol/history.md)
- [`protocol/transactions.md`](../../../protocol/transactions.md)
- [`protocol/metadata-store.md`](../../../protocol/metadata-store.md)
- [`protocol/metadata-host.md`](../../../protocol/metadata-host.md)
