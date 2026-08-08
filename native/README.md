# Transitional native metadata migration source

Tahto's authoritative implementation is HAL under `src/tahto/`. This directory
is retained temporarily so the reviewed durability behavior can be migrated to
an application-neutral `hara.store` capability without losing executable
conformance evidence.

It is **not** the target Tahto architecture and must not gain new Tahto domain
semantics.

```text
abi/metadata-store
  superseded tahto-metadata-store/1 contract retained for migration

provider/metadata-sqlite
  SQLite behavior used as the compatibility fixture for Hoplite issue #45
```

## Generic mechanics to extract

```text
canonical HTA value persistence
bounded revisions
initialize-if-absent
exact compare-and-swap
atomic value and opaque receipt commit
receipt-key lookup and replay
state digest recomputation
restart and fault safety
```

Those mechanics belong in generic host infrastructure.

## Tahto semantics that stay in HAL

```text
state and object graph meaning
transaction-plan validation
request and result evidence
receipt payload interpretation
applied-versus-replayed policy
recovery and merge decisions
authorization and quotas
```

The revised `tahto.store.provider` calls `hara.store` with an opaque state value,
digest, revision and receipt payload. The generic driver must not parse the
Tahto payload. A native ABI version, database path, driver, provider package or
credential is selected only by trusted host installation and never appears in
HAL application values.

## Removal condition

This tree is deleted after a generic in-memory driver and the migrated SQLite
driver pass equivalent load, initialize, CAS, receipt, restart and fault
fixtures through Hoplite. Git history remains the permanent archive of the
superseded Tahto-specific ABI.
