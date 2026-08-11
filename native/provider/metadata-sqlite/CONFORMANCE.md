# SQLite metadata provider conformance

The provider is accepted only when the release gate proves all of the following against the exact pull-request head:

```text
schema version and STRICT-table verification
canonical HTA0 snapshot load and restart recovery
SHA-256 state verification on initialize, commit and load
one-step expected-revision compare-and-swap
exact plan-digest replay after later commits
plan-digest and receipt-revision conflict rejection
stale concurrent-writer rejection
snapshot and receipt atomicity under forced receipt failure
future schema rejection without rewrite
corrupt snapshot and receipt evidence rejection
provider-entry revalidation after public value mutation
locked Rust dependency graph
complete Tahto protocol guards
complete 97-test Hara state-kernel line
```

The suite does not represent the provider as installed in the running Tahto node. Canonical state encoding, host registration, signing/freshness verification, object transfer and the two-device Gate B scenario remain separate integration work.
