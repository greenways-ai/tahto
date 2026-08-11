# Tahto state kernels

This directory contains the Hara-owned Tahto state, transaction, semantic and
generic-capability orchestration logic.

```text
model.hal             canonical identities, limits and immutable state
host.hal              closed Tahto domain effects and result validation
capability.hal        pure-HAL mapping to generic hoplite.blob calls
upload.hal            candidate-state upload orchestration and rollback
response_source.hal   authorized range projection to hoplite.response-source/0-alpha
memory_blob.hal       deterministic pure-HAL hoplite.blob reference provider
graph.hal             manifests, content graph, closure, roots and dry-run GC
vault.hal             uploads, installation, references, quotas and ranges
history.hal           immutable commits, CAS heads, backups, restore, receipts
transaction.hal       verified request-to-metadata commit plans
provider.hal          pure-HAL mapping to generic hoplite.store
memory_store.hal      deterministic pure-HAL hoplite.store reference provider
```

Record and signed-verification contracts live under `tahto.protocol.*`.

## HAL-only domain boundary

The kernels own:

```text
identity and authorization
quotas and range policy
object and history graph meaning
transaction and recovery plans
snapshot and receipt interpretation
closed request and result validation
```

They never accept a destination path, remote URL, raw request body, private key,
bearer credential, provider package, driver, native ABI, library or executable
command.

## Generic object capability

`tahto.store.capability` maps the closed Tahto object effects to:

```clojure
{:service "hoplite.blob"
 :operation "..."
 :arguments [request]}
```

The supported operations are:

```text
staging/open
staging/append-from-source
staging/abort
staging/verify-commit
object/open-source
```

A logical staging key is not a path. A source handle is an ephemeral resource
which a production host resolves only through exact request context + work +
handle ownership.

`tahto.store.upload` runs the vault transition as candidate state, invokes one
matching generic effect and commits only after the exact generic result passes
HAL validation. All provider preparation, identity and result failures return
the original state.

`tahto.store.response-source` first runs `vault/plan-range`, then validates the
exact generic source result and projects only:

```clojure
{:protocol "hoplite.response-source/0-alpha"
 :service "hoplite.blob"
 :source-handle 31
 :offset 2
 :length 7}
```

Object bytes and source ownership never enter Tahto values. An independently
packaged filesystem provider owns byte custody, digests and restart safety;
the Hoplite Nginx transport owns bounded streaming, backpressure and cleanup.

### Deterministic blob profile

`memory_blob.hal` supplies the same request/result profile for pure HAL laws. It
models logical staging, exact offsets, work-owned source descriptors,
idempotent abort, object installation, output-source ownership and exactly-once
close. It does not pretend to hash bytes it never receives.

## Generic metadata capability

TAHTO transaction reducers produce a tentative next state and one bounded atomic
metadata plan. `tahto.store.provider` validates Tahto state and receipt evidence,
then calls:

```text
service: hoplite.store
operations: load · initialize · compare-and-swap · receipt
```

The store treats the state and receipt as opaque canonical values. Mechanical
`applied` or `replayed` status becomes Tahto meaning only after HAL translation.

### Deterministic store profile

`memory_store.hal` models absent/present load, exact initialization,
single-revision CAS, atomic snapshot/receipt publication, exact replay,
stale-writer and key-collision rejection, both fault windows and lost-result
recovery.

An independently packaged SQLite provider preserves the same mechanics while
recomputing digests over actual canonical HTA spans and retaining state across
restart.

## Transitional native source

The Rust metadata implementation under `native/` is frozen migration and parity
evidence. Generic provider extraction is complete. The tree is removed by #17
after memory/SQLite parity, production two-device transfer and semantic
recovery/restore gates pass. New Tahto semantics must never be added there.

## History and semantic direction

Tahto preserves divergent valid commit roots. Applications decide whether and
how those roots are reconciled.

The first Semantic Fabric profiles (#30–#35) add exact schema references, stable
semantic identities, typed content-addressed links, bounded stable-ID indexes
and complete semantic roots. They compose over the existing object graph and
ordinary commit/head laws rather than replacing them.

Normative and integration documents:

- [`protocol/object-vault.md`](../../../protocol/object-vault.md)
- [`protocol/host-capabilities.md`](../../../protocol/host-capabilities.md)
- [`protocol/upload-integration.md`](../../../protocol/upload-integration.md)
- [`protocol/response-sources.md`](../../../protocol/response-sources.md)
- [`protocol/history.md`](../../../protocol/history.md)
- [`protocol/two-device-object-transfer.md`](../../../protocol/two-device-object-transfer.md)
- [`protocol/transactions.md`](../../../protocol/transactions.md)
- [`protocol/metadata-store.md`](../../../protocol/metadata-store.md)
- [`protocol/metadata-host.md`](../../../protocol/metadata-host.md)
