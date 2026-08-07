# Tahto metadata host contract, version 1

TAHTO-8 defines the Hara-side contract that turns a reviewed TAHTO-7 transaction result into one fixed call to an installed durable metadata provider.

The contract is deliberately split from the native `tahto-metadata-store/1` ABI:

```text
Hara state and transaction semantics
  tahto.store.provider
            │ fixed service and closed request
            ▼
Hoplite native host integration
            │ canonical HTA1 bytes
            ▼
tahto-metadata-store/1
            │
            ▼
installed SQLite or other durable provider
```

## Authority

Greenways OS owns provider installation, user consent, application grants, credentials and private keys. Tahto owns metadata state, request replay semantics and revision laws. Hoplite owns the fixed host registry and canonical HTA transport. The installed provider owns durable storage mechanics.

A request cannot choose a provider package, database path, upstream, credential, callback or native command. The only host service identity is:

```text
tahto.metadata
```

The only native ABI identity is:

```text
tahto-metadata-store/1
```

## Operations

The closed operation vocabulary is:

```text
load
initialize
compare-and-swap
receipt
```

`load` returns either no installed snapshot or one validated snapshot. `initialize` installs an exact reviewed snapshot only when the provider is empty. `compare-and-swap` commits one TAHTO-7 state revision and its receipt. `receipt` looks up an exact plan identity for crash and retry recovery.

## Canonical state evidence

Hara never claims to serialize or hash arbitrary state values. Before initialization, an installed canonical encoder returns a closed verification proof containing:

```text
verified = true
metadata revision
state digest
encoding timestamp
```

The proof is accepted only when its revision equals the state value's `:metadata-revision`, treating a missing legacy revision as zero.

For a commit, the installed encoder binds:

```text
expected revision
next revision
plan digest
request digest
result digest
state digest
completion timestamp
```

TAHTO-8 checks that evidence against the exact TAHTO-7 transaction plan before exposing a host call.

## Result validation

Provider snapshots and receipts use closed Hara records. Unknown fields fail closed. A commit receipt must match the exact plan, request, result and state digests plus revision and completion timestamp supplied by the canonical verification proof.

Both successful domain results and completed idempotent error results use the same durable boundary. A rejected domain transition may therefore be remembered without preserving any partial mutation returned by the rejected transition.

## Security laws

- the host service and native ABI are fixed by installed code;
- provider operations are closed and operation-specific;
- public values are revalidated before a call is exposed;
- every commit advances exactly one signed-64-bit-compatible revision;
- request and result digests must equal the completed TAHTO-7 request context;
- no raw object body, private key, bearer credential, database path, URL or native command enters the request;
- load, initialization and receipt results are closed records;
- a provider receipt cannot be rebound to another canonical plan; and
- the Hara contract does not represent the native provider as wired into Hoplite.

## Not included

TAHTO-8 does not implement the Hoplite host registry, canonical HTA encoder/decoder, provider lifecycle, HTTP routes, object transfer, signing, freshness verification, replay compaction, pairing UX or the complete two-device Gate B fixture.
