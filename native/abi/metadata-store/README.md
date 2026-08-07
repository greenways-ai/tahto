# `tahto-metadata-store-abi`

Dependency-free Rust types for the host-owned durable metadata boundary.

The ABI carries canonical HTA snapshots and TAHTO-7 commit evidence through four operations:

```text
load
initialize
compare-and-swap
receipt
```

A concrete provider must recompute the state SHA-256 digest, perform one atomic revision compare-and-swap, and record the commit receipt in the same transaction. The ABI itself performs no I/O and owns no keys, credentials, application semantics or worker execution.

See [`protocol/metadata-store.md`](../../../protocol/metadata-store.md) for the normative provider laws.
