# Tahto metadata-store provider contract, version 1

The native `tahto-metadata-store/1` ABI is the closed provider boundary between TAHTO-7's deterministic Hara transaction plan and a concrete durable compare-and-swap store.

## Ownership

Tahto owns the state and transaction semantics. A host-installed provider owns durable storage and atomic compare-and-swap execution. Greenways OS retains installation, consent, grants, credentials and private keys.

The provider does not execute Hara, parse application payloads, verify request signatures, choose divergent-head winners or install code. It receives canonical HTA state bytes plus bounded transaction evidence produced by reviewed host code.

## Snapshot

A snapshot contains:

```text
revision
canonical HTA1 state bytes
canonical state digest
```

The revision must fit in a signed 64-bit storage column. State bytes are bounded and must be a canonical HTA1 frame. The native ABI validates the evidence shape; a concrete provider must recompute the SHA-256 digest before accepting bytes.

## Commit plan

A commit plan contains:

```text
expected revision
next revision
plan digest
request digest
result digest
canonical HTA1 state bytes
state digest
completion timestamp
```

The next revision must equal the expected revision plus one. Digests use lowercase `sha256:` notation. The plan digest identifies the exact provider operation and is the key for storage-level retry detection.

## Provider operations

```text
load
initialize
compare-and-swap
receipt lookup
```

`initialize` may create the first snapshot only when the store is empty. `compare-and-swap` atomically verifies the expected revision, installs the new snapshot and records a commit receipt. Repeating the exact plan returns the stored receipt as a replay. Reusing a plan digest with different evidence fails closed.

## Durability law

A provider commit must atomically persist:

```text
new canonical state bytes and digest
new metadata revision
plan/request/result evidence
completion timestamp
```

No caller may observe the new revision without its state bytes or its receipt. A crash before commit exposes the old snapshot; a crash after commit exposes the complete new snapshot and receipt.

## Not provided by the ABI

The ABI does not select SQLite or PostgreSQL, create database files, run migrations, compute request signatures, transfer object bodies, compact replay state, or close Gate B. Concrete providers and end-to-end restart conformance are separate release slices.
