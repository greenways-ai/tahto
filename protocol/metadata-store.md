# Transitional metadata-store migration source

## Status

The Rust code under `native/` preserves the reviewed behavior of the first
metadata durability experiment. It is no longer the target Tahto architecture.
Tahto is a HAL system; the code is retained only until its application-neutral
storage mechanics are extracted into Hoplite issue #45.

The superseded identity is:

```text
tahto-metadata-store/1
```

New Tahto HAL code must not refer to this identity or to `tahto.metadata`.

## Mechanics worth preserving

The migration source demonstrates generic behavior that belongs in a
`hara.store` driver:

- bounded signed-64-bit-compatible revisions;
- initialize only when absent;
- exact expected-revision compare-and-swap;
- canonical HTA value persistence;
- digest recomputation over the actual stored bytes;
- atomic value and receipt commit;
- opaque receipt-key lookup and replay;
- crash safety across transaction boundaries;
- stale-writer and mismatched-evidence rejection.

These mechanics do not require Tahto concepts.

## Mechanics that must not move below HAL

The generic driver must not interpret:

- Tahto state shape or object graphs;
- request and result semantics;
- application authorization;
- transaction-plan meaning;
- receipt fields or replay policy;
- recovery and merge decisions.

The revised `tahto.store.provider` places the complete Tahto receipt evidence in
an opaque value. The generic store persists it without parsing it and returns
only mechanical `applied` or `replayed` status. HAL constructs and validates the
final Tahto receipt.

## Generic target

```text
service: hara.store
operations:
  load
  initialize
  compare-and-swap
  receipt
```

A versioned native provider ABI may exist inside Hoplite, but trusted host
installation binds it to `hara.store`. ABI identities, database paths, drivers,
credentials and provider packages never enter Hara application values.

## Migration sequence

1. reproduce the current in-memory and SQLite behavior under a generic store
   contract in Hoplite;
2. run the same load, initialization, CAS, receipt, restart and fault fixtures
   against both implementations;
3. point Tahto's pure-HAL client only at `hara.store`;
4. remove the compatibility identity and Tahto `native/` tree; and
5. make Tahto CI reject any new native implementation directory or
   Tahto-specific provider identity.

Git history remains the archive for the superseded ABI after removal; the Tahto
repository does not need to carry a dormant native product indefinitely.
