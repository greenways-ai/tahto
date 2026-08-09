# Transitional metadata-store migration source

## Status

The Rust code under `native/` preserves the first reviewed metadata durability
experiment. It is no longer the Tahto runtime or the provider selected by the
current HAL client.

The generic replacement has landed in Hoplite:

```text
service: hoplite.store
operations:
  load
  initialize
  compare-and-swap
  receipt
```

Tahto is a HAL system. `tahto.store.provider` prepares and validates that generic
profile, while trusted Hoplite installation selects the SQLite or in-memory
driver.

The superseded migration identity is:

```text
tahto-metadata-store/1
```

New Tahto HAL code must not refer to this identity or to `tahto.metadata`.

## Generic mechanics already extracted

The current `hoplite.store` providers preserve:

- bounded signed-64-bit-compatible revisions;
- initialize only when absent;
- exact expected-revision compare-and-swap;
- canonical HTA value persistence;
- digest recomputation over actual stored bytes;
- atomic value and opaque receipt commit;
- receipt-key lookup and exact replay;
- crash safety across transaction boundaries;
- stale-writer and mismatched-evidence rejection;
- restart recovery and lost-result retry.

These mechanics are application-neutral and live outside Tahto core.

## Semantics that remain in HAL

A generic driver must not interpret:

- Tahto state, object or semantic graph shape;
- request and result meaning;
- application authorization;
- transaction-plan meaning;
- receipt fields or replay policy;
- recovery and merge decisions.

Tahto places complete receipt evidence in an opaque canonical value. The store
persists it without parsing it and reports only mechanical `applied` or
`replayed`. HAL constructs and validates the final Tahto receipt.

## Why `native/` still exists

The migration source remains temporarily because it is executable evidence for
parity and failure behavior. It is frozen: no new Tahto domain rule may be added
beneath this directory.

Removal is tracked by #17 and requires:

1. the same Tahto HAL client passing against memory and Hoplite SQLite stores;
2. load, initialization, CAS, receipt, concurrent writer, restart and both fault
   windows;
3. the production signed two-device object fixture from #36;
4. semantic object, history, divergence, backup and restore conformance from
   #30–#35; and
5. an explicit migration path only if a real deployed native database exists.

After that gate, Git history remains the archive and Tahto CI rejects any new
native implementation directory or Tahto-specific provider identity.

## Security boundary

A native provider ABI may exist inside generic host infrastructure, but trusted
installation binds it to `hoplite.store`. ABI identities, paths, drivers,
credentials and provider packages never enter application values.
