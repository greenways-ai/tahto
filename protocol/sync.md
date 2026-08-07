# Tahto device enrolment and incremental sync

TAHTO-5 adds the application-neutral Hara state kernel for enrolled devices, signed request replay protection, idempotency and incremental object synchronization.

It does not install a cryptographic provider or network transport. Those are trusted host responsibilities wired through Hoplite and Greenways OS.

## Authority boundary

```text
Greenways OS
  approves pairing, retains keys and application grants

Hoplite / installed verifier
  verifies canonical request signatures, freshness and public-key possession

Tahto Hara kernel
  admits bounded proofs, remembers nonces and idempotency state,
  preserves device status, and produces inert synchronization plans

Application
  owns payload meaning and conflict reconciliation
```

Enrolment stores only:

```text
device identity
node identity
public key
status
enrolment and optional revocation timestamps
```

It stores no administrator role, bearer credential, private key, provider credential or application grant. Merely pairing a device never grants management authority.

## Provider proofs

### Enrolment

An installed authority provider returns:

```clojure
{:protocol "tahto.device-enrolment/1"
 :authorized true
 :node "node.home"
 :approval-digest "sha256:..."
 :device
 {:protocol "tahto.device/1"
  :id "device.a"
  :node "node.home"
  :public-key "ed25519:..."
  :status "enrolled"
  :enrolled-at "2026-08-07T00:00:00Z"}}
```

The approval digest identifies the reviewed pairing ceremony. Hara validates the closed proof and device record before changing state.

One device ID cannot be rebound to a different key, and one public key cannot be paired to two device IDs.

### Revocation

Revocation is an explicit, durable authority proof:

```clojure
{:protocol "tahto.device-revocation/1"
 :authorized true
 :node "node.home"
 :device "device.a"
 :revoked-at "2026-08-07T01:00:00Z"
 :approval-digest "sha256:..."}
```

A revoked device cannot authorize new requests. Its public key remains reserved so revocation cannot be bypassed by re-enrolling the same key under another identity.

### Signed application request

After canonical request verification, freshness checking and application-grant evaluation, the installed verifier returns:

```clojure
{:protocol "tahto.request-verification/1"
 :verified true
 :device "device.a"
 :public-key "ed25519:..."
 :operation "sync.pull"
 :application "app.example"
 :namespace "profile.primary"
 :collection "archive"
 :request-digest "sha256:..."
 :nonce "opaque-nonce-..."
 :idempotency-key "request-..."
 :timestamp "2026-08-07T00:01:00Z"}
```

The operation vocabulary is closed to application data-plane actions. Management and administrator operations are deliberately absent.

Hara confirms that:

```text
the device exists and is enrolled
the proof key equals the enrolled public key
the operation is application-scoped
the nonce has never been accepted
the idempotency key is unused or bound to identical canonical bytes
table bounds have not been exceeded
```

An accepted proof becomes a `tahto.device-request-context/1`. This context is an operation- and coordinate-specific input to later reducers; it is not a reusable principal or credential.

## Nonces and idempotency

Every accepted nonce is durable replay evidence. Reusing it fails even when the request bytes are identical.

An idempotency key is bound to:

```text
device
operation
application
namespace
collection
canonical request digest
```

A retry must use a fresh nonce. The same key and same canonical request returns the existing pending/completed state. The same key with different bytes or coordinates fails.

Completion stores only a canonical result digest and completion timestamp. It cannot replace a previously completed result with another digest.

Retention and compaction of old nonce/idempotency evidence belong to the durable metadata-provider profile. A provider must never prune evidence that is still inside its configured replay window.

## Push negotiation

`plan-push` delegates digest classification to the namespace-aware object vault.

A digest is reported present only when it is already reachable from the authorized application namespace. A physically deduplicated object stored solely for another namespace is reported missing, preventing existence disclosure and accidental authority transfer.

The plan is inert data:

```clojure
{:protocol "tahto.sync-push-plan/1"
 :device "device.a"
 :application "app.example"
 :namespace "profile.primary"
 :collection "archive"
 :present ["sha256:..."]
 :missing ["sha256:..."]
 :request-digest "sha256:..."}
```

Uploading and installing the missing bytes remains a native Hoplite/Tahto host effect governed by the existing object-vault transition.

## Pull offers

A pull request supplies a bounded, distinct vector of already-known digests. Tahto gathers current commit roots from every matching device/main head, verifies the complete bounded closure, subtracts known digests and creates one pending offer:

```clojure
{:protocol "tahto.sync-offer/1"
 :direction "pull"
 :device "device.a"
 :application "app.example"
 :namespace "profile.primary"
 :collection "archive"
 :cursor 1
 :request-digest "sha256:..."
 :known-digests []
 :heads ["sha256:..."]
 :missing ["sha256:..."]
 :complete true}
```

The missing vector is sorted and bounded. When the closure is larger than one batch, `:complete` is false and the next round includes newly received objects in `:known-digests`.

Only one unacknowledged offer may exist per device/application/namespace/collection coordinate. Retrying identical canonical request bytes returns the same offer; another request cannot replace it.

## Offline acknowledgement and cursors

An acknowledgement is separately signed and must match:

```text
the pending offer cursor
the offer request digest
the exact offered digest batch
```

On acceptance, Tahto advances the per-device collection cursor exactly once and removes the pending offer.

Cursors count acknowledged synchronization rounds. They are not global ordering claims and do not choose between divergent application heads.

## Conflict preservation

Pull offers include all valid current commit roots for the collection. Tahto neither selects a winning root nor performs last-write-wins merging. Applications reconcile divergent heads explicitly.

## Security laws

- pairing never creates administrator authority;
- no private key or bearer credential enters Hara state;
- request proof operation and application coordinate are immutable inputs;
- nonces cannot be replayed;
- idempotency keys cannot be rebound to different canonical bytes;
- revoked devices cannot admit new requests;
- namespace-external physical objects are never disclosed as present;
- incomplete or overflowing closures are never advertised;
- pending offers cannot be overwritten by a competing request;
- acknowledgements cannot skip or mutate a cursor;
- plans contain no executable payload; and
- device, nonce, idempotency and sync transitions must be committed atomically by the durable provider.

## Deferred implementation

This kernel does not yet provide:

```text
Ed25519 or P-256 verification
canonical request-byte construction
clock/freshness enforcement
SQLite or PostgreSQL transactions
HTTP routes and Nginx backpressure
device pairing UX
nonce retention compaction
object-transfer execution
node-to-node replication
```

Those host and durability slices must preserve the Hara reducer results exactly.
