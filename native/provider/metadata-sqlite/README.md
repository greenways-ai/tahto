# Tahto SQLite metadata provider

This crate implements `tahto-metadata-store/0-alpha` as a local SQLite compare-and-swap store.

It persists one canonical HTA metadata snapshot and an append-only receipt row for each committed TAHTO-7 plan. Snapshot replacement and receipt insertion occur in one `BEGIN IMMEDIATE` transaction.

The provider owns only durable storage mechanics:

```text
open and verify the schema
initialize one snapshot
load and revalidate canonical state
compare the expected revision
install the next snapshot
record or replay the exact plan receipt
```

It recomputes SHA-256 over every snapshot before accepting or returning it. It does not execute Hara, verify signatures, interpret application payloads, transfer object bodies, access private keys or select merge winners.

The first schema uses WAL mode, foreign keys, full synchronous commits and SQLite `user_version = 1`. A host integration layer is still required to encode/decode canonical HTA state and invoke this provider from the Tahto node.
