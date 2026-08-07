# Native provider contracts and adapters

Tahto's authoritative transition rules remain deterministic Hara code under `src/tahto/`. Native crates under this directory define closed provider boundaries for work that Hara must not emulate, including durable storage, canonical byte identity and object transfer.

The metadata boundary is split deliberately:

```text
abi/metadata-store
  dependency-free tahto-metadata-store/1 contract

provider/metadata-sqlite
  local SQLite implementation of that exact contract
```

The contract carries only:

```text
canonical HTA state bytes
current and expected metadata revisions
canonical plan, request, result and state digests
bounded completion timestamps
applied or replayed commit receipts
```

A provider may load one snapshot, initialize an empty store, atomically compare-and-swap one TAHTO-7 plan, and retrieve a receipt by plan digest. The SQLite provider recomputes every state digest and commits snapshot replacement plus receipt insertion in one immediate transaction.

Native providers do not:

```text
execute Hara transitions
parse application payloads
choose divergent-head winners
verify user consent or application grants
hold private keys or credentials
install remote code
claim an in-memory map is durable
```

Greenways OS retains installation, consent, grants, credentials and private-key authority. Tahto owns the state and concurrency laws. Installed provider packages own concrete persistence. A separate host integration must still encode canonical HTA state and invoke the selected provider from the Tahto node.
