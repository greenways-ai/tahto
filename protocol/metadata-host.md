# Tahto metadata over the generic durable-store capability

## Decision

Tahto metadata state, transaction plans, receipt meaning and recovery policy are
implemented in HAL. The production host supplies only application-neutral
durable value storage and atomic revision compare-and-swap.

```text
Tahto HAL
  state meaning · transaction validation · receipt interpretation · recovery
        |
        | one exact generic Host/call
        v
hoplite.store
  opaque canonical values · revision CAS · atomic receipt storage
        |
        v
installed Hoplite SQLite or in-memory driver
```

There is no active `tahto.metadata` service in the target architecture and no
Tahto native ABI selected by application code.

## Installed baseline

The generic boundary is implemented:

- `tahto.store.provider` prepares and validates the exact HAL calls;
- the deterministic pure-HAL memory store supplies conformance behavior;
- Hoplite owns the generic SQLite provider and trusted worker registration;
- canonical nested HTA values and opaque receipts survive restart exactly;
- initialization, compare-and-swap, stale-writer rejection, receipt lookup and
  exact replay have generic provider coverage.

The current public Tahto node exposes discovery, health and status only. A
future authenticated semantic service will compose these already-installed
mechanics; it is not advertised as present today.

## Host call boundary

Tahto prepares calls with exactly three fields:

```clojure
{:service "hoplite.store"
 :operation "compare-and-swap"
 :arguments [request]}
```

A call plan mirrors:

```clojure
(std.foundation.host/call "hoplite.store" "compare-and-swap" request)
```

Trusted host installation associates `hoplite.store` with a reviewed provider.
HAL cannot select an ABI, driver, database path, credential, provider package or
native library.

## Generic request profile

```text
request protocol: hoplite.store-request/0-alpha
result protocol: hoplite.store-result/0-alpha
operations:
  load
  initialize
  compare-and-swap
  receipt
```

The generic driver understands only:

- one signed-64-bit-compatible revision;
- an expected revision for compare-and-swap;
- an opaque canonical `:value` and its digest;
- an opaque receipt key and receipt value;
- atomic installation and lookup.

It does not understand Tahto objects, semantic graphs, request contexts,
transaction meaning, authorization, merge policy or receipt fields.

## Initialization

After HAL validates canonical state-encoding evidence, it prepares:

```clojure
{:protocol "hoplite.store-request/0-alpha"
 :operation "initialize"
 :revision 1
 :value tahto-state
 :value-digest "sha256:..."}
```

The driver recomputes the canonical value digest and installs the value only
when the configured store is empty. A generic result returns the same opaque
value, revision and digest. HAL translates and revalidates it before exposing
Tahto state.

## Compare-and-swap

HAL validates the complete TAHTO transaction plan and canonical encoding proof
before preparing:

```clojure
{:protocol "hoplite.store-request/0-alpha"
 :operation "compare-and-swap"
 :expected-revision 0
 :revision 1
 :value tahto-state
 :value-digest "sha256:..."
 :receipt-key "sha256:plan..."
 :receipt {:protocol "tahto.metadata-receipt-payload/0-alpha"
           :revision 1
           :plan-digest "sha256:plan..."
           :request-digest "sha256:request..."
           :result-digest "sha256:result..."
           :state-digest "sha256:state..."
           :completed-at "..."}}
```

The receipt payload is opaque to the store and deliberately has no application
status. The generic result reports only mechanical `applied` or `replayed`.
Tahto HAL combines that status with the validated payload to construct the final
commit receipt.

## Load and receipt recovery

`load` returns either absence or one exact generic snapshot. `receipt` looks up
one opaque canonical key and returns the exact stored payload. Tahto translates
and validates both results before accepting state or recovery evidence.

Exact retry replay is checked before stale-revision failure, allowing recovery
when a durable commit succeeded but its first result was lost.

## Ownership

Tahto HAL owns:

- state and metadata revision meaning;
- exact transaction-plan validation;
- request and result evidence;
- application receipt payloads;
- applied-versus-replayed interpretation;
- recovery and merge decisions.

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

- application values cannot select an ABI, driver or storage location;
- application values and receipts remain opaque canonical HTA spans below HAL;
- every commit advances exactly one bounded revision;
- state digests bind the actual value persisted by the driver;
- receipt keys are canonical opaque identities, not paths;
- a receipt payload cannot be rebound to another plan or revision;
- generic mechanical status becomes Tahto replay meaning only in HAL;
- no raw object body, private key, credential, URL or native command enters a
  store request; and
- completed idempotent errors use the same durable boundary as successful
  transitions.

## Transitional native source

The `native/` tree is no longer awaiting extraction into Hoplite; that generic
provider work is complete. It remains frozen only as executable migration and
parity evidence until #17 proves:

- memory/SQLite equivalence through the exact Tahto HAL client;
- restart and fault recovery;
- signed production two-device transfer from #36;
- semantic divergence, merge, backup and restore from #30–#35.

After those gates, Tahto deletes the native tree and CI rejects future
Tahto-specific provider implementations.
