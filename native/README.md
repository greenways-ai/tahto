# Native provider contracts

Tahto's authoritative transition rules remain deterministic Hara code under `src/tahto/`. Native crates under this directory define closed provider boundaries for work that Hara must not emulate, including durable storage, canonical byte identity and object transfer.

The first contract is `tahto-metadata-store/1` under `abi/metadata-store`.

It carries only:

```text
canonical HTA state bytes
current and expected metadata revisions
canonical plan, request, result and state digests
bounded completion timestamps
applied or replayed commit receipts
```

A provider implementing this contract may load one snapshot, initialize an empty store, atomically compare-and-swap one TAHTO-7 plan, and retrieve a receipt by plan digest.

The ABI does not:

```text
execute Hara transitions
parse application payloads
choose divergent-head winners
verify user consent or application grants
hold private keys or credentials
install remote code
claim an in-memory map is durable
```

Greenways OS retains installation, consent, grants, credentials and private-key authority. Tahto owns the state and concurrency laws. Installed provider packages own concrete persistence and must recompute the digest of the canonical state bytes before commit.
