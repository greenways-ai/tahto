# Tahto HAL host-capability interpretation

## Status

This document defines the application boundary used by Tahto's pure-HAL object
store. It is an integration profile under review, not a Tahto native ABI.

## Decision

Tahto domain and orchestration code is authored in HAL. Operating-system
mechanics remain below the language boundary as generic installed
capabilities:

```text
Tahto HAL
  state transitions · authorization · quotas · object graph · recovery policy
        |
        | closed Tahto effect
        v
Tahto HAL capability interpreter
  exact mapping · generic request validation · generic result translation
        |
        | std.foundation.host/call
        v
generic Hara host capability
  bounded byte movement · hashing · atomic storage · response sources
        |
        v
installed driver and operating system
```

A native provider descriptor may have its own ABI version, but that descriptor
is selected by trusted host installation. Its ABI identity is not an argument,
field or choice available to Tahto.

## Host call shape

Every prepared call mirrors the portable Hara host boundary:

```clojure
{:service "hara.blob"
 :operation "staging/append-from-source"
 :arguments [{:protocol "hara.blob-request/1"
              :operation "staging/append-from-source"
              ...}]}
```

This corresponds to:

```clojure
(std.foundation.host/call service operation request)
```

The wrapper packages `request` into the one-element argument vector. A call
plan cannot contain `:native-abi`, a driver, path, credential, package or native
library identity.

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
drivers or provider packages
callbacks
commands or executables
raw request-body bytes
```

Opaque source handles are transport resources. A positive number alone grants
no authority: production hosts must resolve it with the exact owning work and
resource scope.

## Generic blob profile

```text
service: hara.blob
request protocol: hara.blob-request/1
result protocol: hara.blob-result/1
```

The installed provider ABI is deliberately absent from the HAL profile.

### Effect mapping

| Tahto effect | Generic operation |
| --- | --- |
| `upload/open` | `staging/open` |
| `upload/append` | `staging/append-from-source` |
| `upload/abort` | `staging/abort` |
| `upload/verify-install` | `staging/verify-commit` |
| `object/read-range` | `object/open-source` |

`manifest/verify` is not mapped by this profile. Manifest parsing remains HAL
work and needs a separate bounded small-value read profile.

## Staging identity and source identity

A staging key and a native handle are different things.

- `:staging-key` is a bounded logical key used to create or resume server-owned
  staging state. It is never interpreted as a path.
- `:source-handle` is a work-scoped opaque source supplied by the host, such as
  an Nginx request body.
- physical paths, file descriptors and sink objects remain entirely below the
  host boundary.

This distinction preserves resumable uploads across requests without treating
an application upload ID as a file handle or native authority.

## Requests

### `staging/open`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging/open"
 :staging-key "upload.a"
 :expected-digest "sha256:..."
 :expected-size 4096
 :media-type "application/octet-stream"}
```

The host creates or resumes server-owned staging state and returns its verified
current offset. The offset cannot exceed the declared object size.

### `staging/append-from-source`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging/append-from-source"
 :staging-key "upload.a"
 :offset 0
 :length 4096
 :source-handle 17}
```

The host resolves the source only under the owning work, consumes exactly
`:length` bytes, rejects short or excess input, and closes the source exactly
once. The operation name is transport-neutral: a CLI or test host can supply a
source that is not an HTTP request body.

### `staging/abort`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging/abort"
 :staging-key "upload.a"}
```

Abort is idempotent at the generic storage layer. Tahto still decides when an
application upload may abort.

### `staging/verify-commit`

```clojure
{:protocol "hara.blob-request/1"
 :operation "staging/verify-commit"
 :staging-key "upload.a"
 :expected-digest "sha256:..."
 :expected-size 4096}
```

The generic driver verifies actual bytes and atomically installs immutable
content. Tahto separately verifies that the completion matches its upload state
before accepting the object into its graph.

### `object/open-source`

```clojure
{:protocol "hara.blob-request/1"
 :operation "object/open-source"
 :digest "sha256:..."
 :offset 0
 :length 4096}
```

Tahto HAL has already authorized the object and planned the half-open range.
The generic result returns the exact offset, length and a work-scoped immutable
source handle. Hoplite streams that source with backpressure; object bytes do
not travel through HTA host-call events.

## Result translation

Generic results use `hara.blob-result/1` and exact operation-specific fields.
The HAL interpreter validates each result against its originating call.

A successful generic commit becomes the existing Tahto installation result in
HAL. A successful source open becomes:

```clojure
{:protocol "tahto.store.host-result/1"
 :operation "object/read-range"
 :opened true
 :digest "sha256:..."
 :start 0
 :end 4096
 :length 4096
 :source-handle 31}
```

The handle is response-lifetime transport state and must never be serialized as
portable Tahto application state.

## Execution profiles

The same interpreter supports:

1. deterministic HAL tests with a pure in-memory capability;
2. a Hara CLI host with generic byte sources and stores; and
3. Hoplite, where request and response resources are bound to Nginx work
   lifetimes.

Mapping and validation remain pure. `execute-call` is the thin asynchronous HAL
edge that invokes `std.foundation.host/call` after the call plan is validated.

## Metadata follows the same rule

`tahto.store.provider` targets `hara.store`, not `tahto.metadata`. The generic
store persists opaque canonical values and receipt payloads with revision CAS.
Tahto snapshot and receipt meaning remains in HAL.

The existing Rust metadata code under `native/` is a transitional migration
source for Hoplite issue #45. It is not Tahto's implementation architecture and
will be removed after the generic driver passes equivalent conformance.

## Conformance laws

- Tahto application semantics remain in HAL.
- Every mapped effect produces one exact generic request.
- Unknown or malformed effects expose no host call.
- HAL values cannot select a native ABI, provider or driver.
- Generic requests and results reject extra authority-bearing fields.
- Native handles are work-scoped and never durable application state.
- Range requests use offset plus length across the generic boundary.
- Result translation happens in HAL and requires exact request/result identity.
- Host cancellation precedes work-scope closure.
- Production and in-memory capabilities are substitutable beneath the same HAL
  interpreter.
