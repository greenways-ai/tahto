# Tahto object vault: Hara kernel profile

TAHTO-3 defines the application-neutral object-vault semantics in Hara. Python,
JavaScript, SQLite, a filesystem layout and a particular HTTP server are not the
source of truth for vault behaviour.

```text
Hoplite request / node service
          │ exact OS grant + opaque ResourceHandle
          ▼
Hara tahto.store.* reducer
  lifecycle · quota · graph · roots · GC planning
          │ closed effects, never caller paths
          ▼
Trusted native object host
  stream · seek · hash · fsync · atomic install · range read
```

Hara owns every application-neutral transition. The native host owns byte
movement that must not become an ordinary Hara collection. The host is an
installed node capability, never remotely supplied application code.

## Source and serialization boundary

The implementation uses the source and serialized vocabulary released by the
completed Hara namespace and metadata cuts:

```text
tahto.store.model
tahto.store.host
tahto.store.vault
tahto.store.graph
```

Hara compiler metadata is canonical under `:lang/*`; Tahto owns its versioned
`tahto.*` protocol identifiers without a forwarding language namespace. Tahto's
wire discriminators remain strings such as `tahto.object/1`.

## Immutable reducer state

The kernel state is a small immutable value:

```clojure
{:protocol "tahto.object-vault-state/1"
 :limits {...}
 :objects {digest object-record}
 :uploads {upload-id upload-record}
 :quotas {namespace-key bytes}
 :references {namespace-key #{digest ...}}
 :manifests {manifest-digest [{:digest child :size bytes} ...]}
 :edges {object-digest [child-digest ...]}
 :roots {root-id {:kind kind
                  :application application
                  :namespace namespace
                  :digests [digest ...]}}}
```

The global object records align with the TAHTO-2 `tahto.object/1` envelope:

```clojure
{:protocol "tahto.object/1"
 :digest "sha256:..."
 :size 1234
 :encoding "identity"}
```

TAHTO-3 supports identity objects only. Sealed object encoding remains the
separate opaque-replica profile. Media type is an upload/application hint rather
than global custody identity, so two applications cannot overwrite one another's
semantic metadata when the same bytes deduplicate.

Application, namespace, upload and root identifiers use the TAHTO-2 core
identifier grammar and are bounded to 128 ASCII characters. The private
`namespace-key` is a length-prefixed string used only because portable Hara maps
require scalar keys; it is not a wire identifier or a filesystem path.

Object bodies, temporary upload files, file descriptors, sockets and database
handles are never stored in this value.

Every successful command returns:

```clojure
{:ok true
 :state tentative-next-state
 :value result
 :effects [closed-host-effect ...]}
```

The executor commits `:state` only after all effects succeed. This preserves the
atomic boundary between Hara metadata and native byte operations without
pretending the portable whole-file API is a streaming vault.

## Canonical object identity

An object identifier is exactly:

```text
sha256:<64 lower-case hexadecimal characters>
```

Hara rejects upper-case, shortened, path-like and otherwise non-canonical values
before emitting a host effect. A native provider derives internal locations from
the validated digest. Devices and applications cannot select a destination path,
upstream URL or proxy target.

Zero-byte immutable objects are valid. Upload chunks are non-empty and bounded.

## Upload lifecycle

```text
begin-upload
  reserve the full declared size against one application namespace
  emit upload/open

append-upload
  require the exact committed offset
  enforce the chunk and object bounds
  emit upload/append with a non-zero ResourceHandle

request-install
  require offset == declared size
  keep the quota reservation while verification runs
  emit upload/verify-install

accept-install
  accept only a matching trusted digest/size/install proof
  install global object metadata and the namespace reference
  release the upload reservation

abort-upload
  release the reservation
  emit idempotent native temporary-upload cleanup
```

A stale append receives no host effect. Large bytes are never embedded in an
effect or result.

## Closed host effect boundary

Version 1 recognizes only:

```text
upload/open
upload/append
upload/abort
upload/verify-install
manifest/verify
object/read-range
```

Effects use `tahto.store.host-effect/1`; trusted results use
`tahto.store.host-result/1`. Records containing `path`, `destination`,
`upstream`, `url`, raw `bytes`, a raw `body`, `command` or `executable` are
invalid.

The install provider must hash the complete stream, verify exact size, install
immutably and durably, then return the matching proof. Hara accepts no weaker
result as authoritative installation.

## Logical quotas and global deduplication

Bytes may be globally deduplicated by digest. Authority and accounting remain
logical per `(application, namespace)` reference set.

- One namespace reference consumes the full object size once.
- The same digest in another namespace consumes that namespace's quota.
- An active or verifying upload reserves its complete declared size.
- Objects retained only through an application/head/backup/retention root remain
  part of that namespace's unique closure usage.
- A quota cannot be reduced below retained closure use plus reservations.
- Physical presence owned by another namespace or application is neither
  disclosed nor granted without a new verified upload or a later explicit
  sharing protocol.

Existence negotiation is scoped to one exact `(application, namespace)` closure,
bounds the requested digest count before traversal, and returns two stable,
order-preserving sets:

```text
present      stored and reachable from this namespace's references or roots
missing      absent, incomplete or not disclosed to this namespace
```

A digest held elsewhere is reported as `missing`; there is no generic attach
shortcut. Another namespace or application gains authority only by completing
its own verified upload or through a later explicit OS-authorized sharing
protocol. The native host may deduplicate verified bytes without exposing that
physical fact to the caller.

## Range reads and authority

A range is a non-empty half-open interval `[start, end)`. Hara checks canonical
identity, namespace reachability, object size and configured range bounds before
emitting `object/read-range`. An unreferenced digest receives the same
not-authorized result whether or not another application physically stores it.

Namespace reachability is the closure of direct object references plus explicit
application/head/backup/retention roots belonging to that same namespace. A
caller that merely guesses another application's digest receives no read plan.

## Verified chunk manifests

The first manifest profile is `tahto.chunk-manifest/1`. A manifest is itself an
immutable object. Its ordered chunk vector may repeat a digest at multiple
ordinals; occurrences are not collapsed.

The native host reads the bounded manifest object and returns a trusted proof of
the exact decoded fields. Hara then checks:

- protocol and manifest digest;
- maximum fan-out;
- canonical child identities;
- child presence and exact size;
- namespace reachability of every child; and
- equality between logical size and the sum of chunk sizes.

Only then does Hara register closure edges. This prevents an application from
attaching an arbitrary graph to unrelated immutable bytes.

## Closures, roots and garbage collection

Closure traversal is iterative, bounded and cycle-safe. It preserves first-seen
order, collapses repeated identities for traversal, and reports every encountered
missing object.

A root can be pinned only when its complete closure is present and reachable by
the named application namespace. Safe GC roots are the union of all namespace
references and explicit roots.

`gc-plan` is deterministic and dry-run only:

```clojure
{:apply false
 :roots [...]
 :reachable [...]
 :candidates [...]}
```

It refuses to plan deletion when a rooted closure is incomplete or exceeds the
configured traversal bound. Applying a plan belongs to a later authorized node
operation with generation revalidation.

## Persistence boundary

The current portable Hara file API is whole-value oriented, and the portable
SQLite provider is transient. Neither is used to disguise a production data
plane. Hoplite's merged bounded-streaming ABI defines opaque request and response
handles, byte limits and range planning. The remaining runtime/Nginx binding and
durable metadata provider must implement those contracts without changing these
Hara reducer laws.

## Security laws

- Greenways OS grants exact application and namespace authority.
- Upload IDs and resource handles are server generated and never interpreted as
  paths.
- Complete native digest verification precedes authoritative installation.
- Exact offsets reject stale and concurrent append attempts.
- Object, chunk, range, manifest and closure sizes are bounded.
- Verifying uploads continue to reserve quota, and pinned closures remain
  accounted after direct references detach.
- Global deduplication cannot bypass namespace accounting, application
  confidentiality or read authority.
- Manifest edges are proved against the stored immutable manifest bytes.
- Manifests preserve order and repeated chunk occurrences.
- Roots require a complete, authorized closure.
- Garbage collection is dry-run first and fails closed.
- Tahto core contains no application-specific fields or merge rules.

## Conformance

The Hara suites prove:

- canonical digest rejection;
- absence of paths, raw bodies and executable instructions in effects;
- numeric opaque Hoplite handles;
- zero-byte objects and bounded resumable offsets;
- quota reservation and cross-namespace deduplication accounting;
- matching host proof before installation;
- bounded, order-preserving existence negotiation;
- namespace-isolated range and manifest plans without global-presence leakage;
- byte-anchored manifests with repeated chunks;
- bounded unique closure traversal; and
- root-safe dry-run garbage collection.

Python remains only in the pre-existing schema and architecture guards. It is
not a Tahto runtime, storage library or test implementation for this slice.
