# Transitional native metadata migration source

Tahto's authoritative implementation is HAL under `src/tahto/`. The generic
metadata mechanics formerly demonstrated here have been extracted into the
application-neutral `hoplite.store` providers in Hoplite.

This directory is retained only as frozen executable migration and parity
evidence. It is **not** the current Tahto architecture and must not gain new
Tahto domain semantics.

```text
abi/metadata-store
  superseded tahto-metadata-store/1 contract retained for comparison

provider/metadata-sqlite
  original SQLite durability experiment retained for parity fixtures
```

## Current target boundary

```text
Tahto HAL
  state meaning · transaction validation · receipt interpretation · recovery
        |
        v
hoplite.store
  opaque canonical value · exact revision CAS · opaque atomic receipt
        |
        v
trusted Hoplite memory or SQLite provider
```

The current `tahto.store.provider` calls only `hoplite.store`. A native ABI version,
database path, driver, provider package or credential is selected by trusted
host installation and never appears in HAL application values.

## Generic mechanics now owned by Hoplite

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

## Tahto semantics that remain in HAL

```text
state and object/semantic graph meaning
transaction-plan validation
request and result evidence
receipt payload interpretation
applied-versus-replayed policy
recovery and merge decisions
authorization and quotas
```

## Frozen-source law

Until removal:

- CI continues to compile and test this tree;
- no production Tahto code may invoke its ABI;
- no new protocol, authorization, semantic or recovery rule may be added here;
- the superseded service identities may appear only in explicitly marked
  migration documentation and source.

## Removal condition

Issue #17 deletes this tree after:

- the exact Tahto HAL client passes memory/SQLite parity and fault fixtures;
- #36 proves production signed two-device transfer across restart;
- #30–#35 prove semantic divergence, application-authored merge, backup and
  fresh-node restore; and
- any evidenced deployed native database has an explicit tested migration path.

Git history remains the permanent archive after removal.
