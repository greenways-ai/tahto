# Tahto state kernels

This directory contains the Hara-owned TAHTO state, transaction and
capability-interpreter logic.

```text
model.hal        canonical identity, limits and immutable state
host.hal         closed Tahto domain effects and result validation
capability.hal   pure-HAL mapping to generic blob/source capabilities
memory_blob.hal  deterministic pure-HAL hara.blob reference provider
graph.hal        verified manifests, shared closure, roots and dry-run GC
vault.hal        uploads, installation, namespace references, quotas and ranges
history.hal      immutable commits, CAS heads, backups, restore and receipts
transaction.hal  atomic verified request-to-metadata commit plans
provider.hal     pure-HAL mapping to a generic durable value store
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
snapshot and receipt meaning
closed effect and result validation
```

They never accept a destination path, upstream URL, raw request body, private
key, bearer credential, driver, native ABI, library or executable command.
Large bodies remain behind work-scoped resource handles.

## Generic blob and response capabilities

`tahto.store.capability` maps existing Tahto effects to `hara.blob` calls with
the exact portable shape:

```clojure
{:service "hara.blob"
 :operation "..."
 :arguments [request]}
```

A logical `:staging-key` is never a path or native handle. Request-body and
response resources cross the boundary as `:source-handle` values that the host
must resolve with their owning work scope. Generic range requests use offset
plus length; Tahto continues to own authorization and half-open range planning.

Mapping, request validation, result matching and translation back into Tahto
results remain pure HAL. Hoplite or another host may implement byte movement,
hashing, atomic installation and backpressured response streaming without
containing Tahto upload, quota, graph or authorization rules.

### Deterministic reference profile

`tahto.store.memory-blob` implements the same generic `hara.blob` request and
result profile entirely in HAL for conformance tests. It models:

```text
logical staging and resume offsets
work-owned input source handles
exactly-once bounded source consumption
idempotent abort
complete-object commit and replay
bounded immutable output sources
work-owned exactly-once source close
```

The reference provider stores small fixture descriptors, not request-body byte
payloads. Each fixture declares its work owner, bounded segment, object digest
and object size. This lets the HAL suite exercise lifecycle and identity laws
without turning byte streams into Tahto application values.

The reference provider deliberately does not pretend to prove a cryptographic
digest over bytes it never receives. Production digest calculation, short-read
and excess-read detection, filesystem durability and backpressure remain generic
host mechanics. The same closed request/result profile is exercised beneath the
Tahto interpreter in both cases.

See [`protocol/host-capabilities.md`](../../../protocol/host-capabilities.md).

## Generic durable metadata capability

TAHTO transaction reducers produce a tentative next state and a bounded atomic
metadata plan. `tahto.store.provider` validates Tahto state and receipt evidence,
then calls `hara.store` with opaque canonical values and exact revisions.

The generic store may initialize, load, compare-and-swap and retrieve an opaque
receipt value. It does not parse Tahto state or receipt fields. Its mechanical
`applied` or `replayed` result receives Tahto meaning only after HAL translates
and validates the stored payload.

The Rust metadata implementation under `native/` is a temporary migration
source for Hoplite issue #45, not Tahto's target implementation. It is removed
once generic in-memory and SQLite drivers pass equivalent restart and fault
conformance.

Tahto preserves divergent valid commit roots. Applications, not the fabric,
decide whether and how those roots are reconciled.

Normative contracts:

- [`protocol/object-vault.md`](../../../protocol/object-vault.md)
- [`protocol/host-capabilities.md`](../../../protocol/host-capabilities.md)
- [`protocol/history.md`](../../../protocol/history.md)
- [`protocol/transactions.md`](../../../protocol/transactions.md)
- [`protocol/metadata-store.md`](../../../protocol/metadata-store.md)
- [`protocol/metadata-host.md`](../../../protocol/metadata-host.md)
