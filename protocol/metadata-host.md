# Tahto metadata over the generic durable-store capability

## Decision

Tahto metadata state, transaction plans, receipt meaning and recovery policy are
implemented in HAL. The production host supplies only application-neutral
durable value storage and atomic revision compare-and-swap.

```text
Tahto HAL
  snapshot meaning · transaction validation · receipt interpretation · recovery
        |
        | one exact generic Host/call
        v
hara.store
  opaque canonical values · revision CAS · atomic receipt storage
        |
        v
installed SQLite, PostgreSQL or in-memory driver
```

There is no `tahto.metadata` native service in the target architecture and no
Tahto native ABI selected by application code.

## Host call boundary

Tahto prepares calls with exactly three fields:

```clojure
{:service "hara.store"
 :operation "compare-and-swap"
 :arguments [request]}
```

A call plan mirrors:

```clojure
(std.foundation.host/call "hara.store" "compare-and-swap" request)
```

The installed host associates `hara.store` with a trusted versioned provider
descriptor. HAL cannot select an ABI, driver, database path, credential,
provider package or native library.

## Generic request profile

```text
request protocol: hara.store-request/1
result protocol: hara.store-result/1
operations:
  load
  initialize
  compare-and-swap
  receipt
```

The generic driver understands only storage mechanics:

- a signed-64-bit-compatible revision;
- an expected revision for compare-and-swap;
- an opaque canonical `:value` and its digest;
- an opaque receipt key and receipt value;
- atomic installation and lookup.

It does not understand Tahto object graphs, request contexts, transaction
semantics, authorization, replay policy or receipt fields.

## Initialization

After HAL validates canonical state-encoding evidence, it prepares:

```clojure
{:protocol "hara.store-request/1"
 :operation "initialize"
 :revision 1
 :value tahto-state
 :value-digest "sha256:..."}
```

The driver verifies the canonical value bytes and installs them only when the
configured store is empty. A generic result returns the same opaque value,
revision and digest. HAL translates that result into
`tahto.metadata-snapshot/1` and performs the final Tahto checks.

## Compare-and-swap

HAL first validates the complete TAHTO transaction plan and its canonical
encoding proof. It then prepares a generic request:

```clojure
{:protocol "hara.store-request/1"
 :operation "compare-and-swap"
 :expected-revision 0
 :revision 1
 :value tahto-state
 :value-digest "sha256:..."
 :receipt-key "sha256:plan..."
 :receipt {:protocol "tahto.metadata-receipt-payload/1"
           :revision 1
           :plan-digest "sha256:plan..."
           :request-digest "sha256:request..."
           :result-digest "sha256:result..."
           :state-digest "sha256:state..."
           :completed-at "..."}}
```

The receipt payload is opaque to the store. It deliberately has no
`:status`. The generic result says only whether the atomic operation was
`applied` or `replayed`; HAL combines that mechanical status with the validated
payload to create `tahto.metadata-commit-receipt/1`.

This prevents the storage driver from owning Tahto replay meaning while still
allowing exact retry recovery.

## Load and receipt recovery

`load` returns either no value or one generic snapshot result. HAL translates
and validates it before exposing Tahto state.

`receipt` takes one opaque canonical receipt key. A found result must use
`replayed` status and contain the exact stored payload. HAL reconstructs the
Tahto receipt and checks it against the original commit verification evidence.

## Canonical evidence

Hara does not claim that arbitrary state is canonical merely because it is a
map. Before initialization or commit, the reviewed encoding boundary supplies
closed evidence containing the revision, canonical digest and timestamps.

Tahto HAL validates that evidence against its state and transaction plan before
any generic store call is exposed. The generic driver independently recomputes
the digest over the actual canonical value span it stores.

## Ownership

Tahto HAL owns:

- state and metadata revision meaning;
- exact transaction-plan validation;
- request and result evidence;
- application receipt payloads;
- applied-versus-replayed interpretation;
- recovery decisions.

The generic store owns:

- trusted storage configuration;
- canonical value persistence;
- initialize-if-absent;
- exact revision compare-and-swap;
- atomic value and receipt commit;
- receipt lookup;
- stale-writer rejection;
- driver-level durability and cancellation.

## Security laws

- application values cannot select a native ABI, driver or storage location;
- the generic store treats application values and receipts as opaque canonical
  HTA spans;
- every commit advances exactly one bounded revision;
- state digests bind the exact value persisted by the driver;
- receipt keys are canonical opaque identities, not paths;
- a receipt payload cannot be rebound to another plan or revision;
- generic mechanical status becomes Tahto replay meaning only in HAL;
- no raw object body, private key, credential, URL or native command enters the
  request; and
- successful and completed idempotent-error transitions use the same durable
  boundary.

## Migration

The existing `native/` metadata ABI and SQLite provider are retained temporarily
as migration sources. Hoplite issue #45 extracts their generic CAS and
transaction mechanics behind `hara.store`. Once the generic in-memory and
SQLite drivers pass equivalent restart and fault conformance, Tahto removes the
native tree and its transitional CI job.
