# Tahto HAL host-capability interpretation

## Status

This document defines the application boundary introduced by Tahto issue #16.
It is an integration profile under review, not yet a stable public Hara ABI.

## Decision

Tahto domain and orchestration code is authored in HAL. Tahto does not require a
Tahto-specific Rust or C runtime.

Operating-system mechanics remain below the language boundary as generic
installed capabilities:

```text
Tahto HAL
  state transitions · authorization · quotas · object graph · recovery policy
        |
        | closed application effect
        v
Tahto HAL capability interpreter
  exact mapping · generic request validation · generic result validation
        |
        | std.native.Host/call in production
        v
generic Hara host capability
  bounded byte movement · hashing · atomic storage · response sources
        |
        v
installed driver and operating system
```

The presence of a native host driver does not make Tahto a native
implementation. The driver must remain application-neutral and must not contain
Tahto's object, namespace, quota, authorization, manifest, history or merge
rules.

## Authority boundary

Tahto's object kernel emits only values accepted by
`tahto.store.host/effect?`. The HAL capability interpreter accepts those values
and exposes a generic host request only after a second exact-shape validation.

Neither layer accepts:

```text
filesystem paths
storage destinations
upstream URLs
bearer credentials
native library names
callbacks
commands or executables
raw request-body bytes
```

An opaque body or source handle is transport state. It is not authorization.
The production host must combine the handle with the owning work scope before
returning any native bytes.

## Initial generic blob profile

The first proposed host identity is:

```text
service: hara.blob
ABI: hara-blob-store/1
request protocol: hara.blob-request/1
result protocol: hara.blob-result/1
```

The names remain provisional until the Hoplite capability is reviewed. They are
intentionally application-neutral.

### Effect mapping

| Tahto effect | Generic host operation |
| --- | --- |
| `upload/open` | `staging-open` |
| `upload/append` | `staging-append-request` |
| `upload/abort` | `staging-abort` |
| `upload/verify-install` | `staging-verify-commit` |
| `object/read-range` | `object-open-range` |

`manifest/verify` is not mapped by the first slice. Manifest parsing belongs in
HAL and requires a separate bounded small-object read profile rather than a
Tahto-specific native verifier.

## Requests

### `staging-open`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging-open"
 :staging-id "upload.a"
 :digest "sha256:..."
 :size 4096
 :media-type "application/octet-stream"}
```

The host derives physical storage from trusted configuration. `:staging-id` is
a logical opaque identity and is not a path.

### `staging-append-request`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging-append-request"
 :staging-id "upload.a"
 :offset 0
 :length 4096
 :body-handle 17}
```

The host call itself supplies the owning work ID. A provider must resolve the
body only with `(work ID, body handle)`, consume exactly `:length` bytes and
finish the source exactly once.

### `staging-abort`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging-abort"
 :staging-id "upload.a"}
```

Abort is idempotent at the generic storage layer. Tahto still decides when an
application upload is allowed to abort.

### `staging-verify-commit`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging-verify-commit"
 :staging-id "upload.a"
 :digest "sha256:..."
 :size 4096}
```

The generic driver verifies actual bytes and atomically installs immutable
content. Tahto separately validates that this completion matches its upload
state before accepting the object into the domain graph.

### `object-open-range`

```clojure
{:protocol "hara.blob-request/1"
 :operation "object-open-range"
 :digest "sha256:..."
 :start 0
 :end 4096}
```

Tahto HAL has already authorized the object and range. The generic host checks
storage integrity and returns a work-scoped immutable source handle. Hoplite
streams that source through Nginx; bytes do not travel through HTA host-call
events.

## Results

Generic results use `hara.blob-result/1` and exact operation-specific fields.
The HAL interpreter validates the result against the original generic call.
For example, a committed object result is accepted only when staging identity,
digest and size match the request exactly.

The interpreter then creates the existing Tahto result record:

```clojure
{:protocol "tahto.store.host-result/1"
 :operation "upload/verify-install"
 :verified true
 :installed true
 :upload-id "upload.a"
 :digest "sha256:..."
 :size 4096}
```

Tahto's existing `verified-install?` validator remains the final application
check. A generic driver result never bypasses the HAL domain boundary.

## Execution profiles

The mapping is useful in three environments without changing Tahto code:

1. **Pure HAL tests** — inspect or execute the planned call through an in-memory
   HAL capability.
2. **Hara CLI host** — install a compatible generic blob driver.
3. **Hoplite** — dispatch through the registered provider ABI and bind request
   and response resources to Nginx work lifetimes.

A thin asynchronous HAL wrapper may call:

```clojure
(std.native.Host/call service operation [request])
```

The mapping and validation functions remain pure and deterministic.

## Migration of native Tahto code

The existing metadata ABI and SQLite provider under `native/` are migration
sources. Tahto issue #17 moves their generic durability mechanics behind an
application-neutral `hara.store` capability. Once compatibility and conformance
are complete, Tahto CI will reject new native implementation code in this
repository.

## Conformance laws

- Tahto application semantics remain in HAL.
- Every mapped effect produces one exact generic request.
- Unknown or malformed effects expose no host call.
- Generic requests and results reject extra authority-bearing fields.
- Native handles are never serialized as portable application state.
- Result translation happens in HAL and requires exact request/result identity.
- Host cancellation precedes work-scope closure.
- Generic drivers never infer Tahto authorization or merge semantics.
- Production and in-memory providers are substitutable beneath the same HAL
  interpreter.
