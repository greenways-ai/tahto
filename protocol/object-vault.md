# Tahto object vault profile, version 1

TAHTO-3 defines the application-neutral state machine for immutable object custody. The authoritative implementation is Hara under `tahto.store.*`.

The kernel deliberately does not implement a byte store in Python, JavaScript, or application code. It emits a small closed set of effects to a trusted Tahto host adapter. Hara owns validation, quota accounting, upload transitions, object metadata, manifest semantics, closure verification, root pins, and garbage-collection planning. The host owns bounded byte movement and durable filesystem operations.

## Authority boundary

```text
Greenways OS
  issues exact application / namespace grants

Hara object-vault kernel
  validates identifiers and state transitions
  accounts quotas and references
  defines manifests, closures, roots and GC plans

Hoplite / native Tahto host
  owns request-body handles
  streams, seeks, hashes, fsyncs and atomically installs bytes

Applications
  define payload meaning and merge semantics
```

Large object bodies never become ordinary Hara collections. A Hoplite request body is represented to Hara by an opaque bounded handle plus an exact byte count. The host adapter may consume that handle only for the effect that named it.

## Hara state

The in-memory conformance representation is:

```clojure
{:protocol "tahto.object-vault-state/1"
 :limits {...}
 :objects {digest object-record}
 :uploads {upload-id upload-record}
 :quotas {[application namespace] bytes}
 :references {[application namespace] #{digest ...}}
 :manifests {digest manifest-record}
 :roots {[kind root-id] root-record}}
```

A durable metadata provider may store the same logical records in SQLite, D1, PostgreSQL or another transactional store. Storage engines do not change the state-transition laws.

## Native host effects

Hara may emit only:

```text
upload/open
upload/append
upload/verify-install
object/read-range
object/delete
```

Every effect uses `tahto.store.host-effect/1`. The host returns versioned result records. The request vocabulary intentionally has no path, destination, upstream or URL field.

A future HOPLITE-1 adapter must enforce the same closed operation set and derive every temporary or final path from node configuration, server-generated upload identity, and a validated digest.

## Digest identity

An object identifier is exactly:

```text
sha256:<64 lower-case hexadecimal characters>
```

Upper-case digests, partial digests, paths and URLs are invalid. A host may derive a local layout such as:

```text
objects/sha256/ab/cdef...
```

but that layout is never request vocabulary.

## Resumable uploads

An upload record contains:

```text
server-generated upload ID
application
namespace
digest
declared size
media type
committed offset
status: open | verifying
```

Opening an upload reserves the complete declared size against the application namespace. Verification remains an active reservation; changing the state from `open` to `verifying` must not temporarily release quota.

Append requires:

- an exact current offset;
- a positive bounded chunk length;
- an opaque body handle;
- no overflow beyond the declared size.

Hara advances the logical offset and emits `upload/append`. The native adapter must serialize or compare-and-swap the corresponding physical append. If the effect fails, its transaction boundary must not commit the Hara transition.

A zero-byte object moves directly from `open` to `verifying`.

## Verified installation

Once the committed offset equals the declared size, Hara emits:

```clojure
{:protocol "tahto.store.host-effect/1"
 :operation "upload/verify-install"
 :request {:upload-id ...
           :digest ...
           :size ...}}
```

The host must:

1. hash the complete temporary byte stream;
2. compare the exact digest and length;
3. reject any incompatible existing object at the derived digest path;
4. atomically install verified bytes;
5. synchronize the required durability boundary; and
6. return a matching `tahto.store.host-result/1` proof.

Hara accepts installation only when upload ID, digest, size, verification and installation all match. It then creates immutable object metadata, creates the logical namespace reference, and removes the upload reservation.

## Existence negotiation

`missing` validates every requested digest, removes duplicate requests while preserving first-seen order, and returns only objects absent from the vault metadata.

The host must not report a digest as installed merely because an unverified file exists at a derived path.

## Quotas and deduplication

Object bytes may be globally deduplicated. Quotas are logical per `(application, namespace)` reference set.

- The same digest counts once within one namespace.
- The same digest consumes quota again when attached to another namespace.
- Active uploads reserve their declared sizes.
- A quota cannot be reduced below committed use plus active reservations.
- Existing global bytes do not grant application access; an explicit namespace attachment is required.

## Range reads

Hara validates a half-open range:

```text
[start, end-exclusive)
```

The range must be inside verified object length and no larger than the configured range limit. Hara emits an `object/read-range` plan. The native/Hoplite adapter owns seekable streaming and HTTP `Range` response details.

## Chunk manifests

The first large-object graph profile is:

```text
tahto.chunk-manifest/1
```

Its Hara representation contains:

```clojure
{:protocol "tahto.chunk-manifest/1"
 :digest manifest-object-digest
 :total-size total-logical-bytes
 :media-type "application/octet-stream"
 :chunks [{:digest chunk-digest :size chunk-size} ...]}
```

The manifest is itself an immutable object. Every child object must exist and have the declared size before registration. Chunk order is normative, and the same digest may appear at multiple ordinals. Repeated chunks must not be collapsed from the sequence.

Manifest fan-out is bounded.

## Closure verification

Closure traversal is iterative and bounded. It follows registered chunk manifests, reports the complete visited object set and missing digest set, and does not interpret application payloads.

A head, backup or retention boundary may be pinned only when its complete closure is present.

## Roots and garbage collection

The safe root set is the union of:

- application/namespace object references; and
- explicit `application`, `head`, `backup`, or `retention` roots.

Hara computes the complete reachable closure and emits one `object/delete` effect for each unreachable digest. A GC plan is dry-run data; a native adapter must not apply it without an explicit operator or policy decision and a metadata transaction that rechecks the roots.

## Transaction rule

The functions in `tahto.store.vault` are deterministic transition planners. A production node applies one command as:

```text
read metadata snapshot
  -> evaluate Hara transition
  -> execute bounded host effects
  -> validate host results
  -> commit new metadata state atomically
```

A host-effect failure aborts the corresponding metadata transition. This keeps Hara authoritative without materializing large bytes in the language runtime.

## Security laws

- no request-selected filesystem destination or upstream;
- no unbounded object body in a Hara value;
- exact offsets precede append effects;
- complete SHA-256 and size verification precede installation metadata;
- verification retains quota reservation;
- global deduplication cannot grant access or bypass logical quota;
- manifest fan-out and closure traversal are bounded;
- incomplete roots cannot be pinned;
- GC begins as an explicit dry-run plan; and
- application-specific fields and merge rules never enter Tahto core.

## Conformance

`test/tahto/store/vault_test.hal` proves:

- canonical digest rejection;
- forbidden host request fields;
- active upload quota reservation;
- zero-byte verification;
- stale offset rejection before host I/O;
- opaque request-body handles;
- exact verified-install proof matching;
- immutable object/reference creation;
- order-preserving missing-object negotiation;
- bounded half-open range planning;
- ordered repeated chunks;
- complete and incomplete closure reporting;
- pinned-root GC safety; and
- namespace quota accounting under global deduplication.

The existing Python programs under `conformance/` remain dependency-free repository, route and JSON-Schema guards. They are not the Tahto object-vault runtime.
