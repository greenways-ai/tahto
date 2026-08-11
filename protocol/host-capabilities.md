# Tahto HAL host-capability interpretation

## Status

This is the active application boundary for Tahto's pure-HAL metadata and object
kernels. It is an integration profile over generic Hara/Hoplite capabilities,
not a Tahto native ABI.

## Decision

Tahto domain and orchestration code is authored in HAL. Operating-system
mechanics remain below the language boundary as generic installed capabilities:

```text
Tahto HAL
  state · authorization · quotas · graph · history · recovery
        |
        | closed Tahto transition/effect
        v
Tahto HAL capability adapter
  exact mapping · request validation · result translation
        |
        | std.foundation.host/call
        v
generic Hara capability
  canonical values · bounded bytes · atomic storage · response sources
        |
        v
trusted Hoplite provider and operating system
```

A provider descriptor may have a native ABI internally, but trusted host
installation selects it. ABI identity is never an application argument or field.

## Portable host call shape

Every prepared call contains exactly:

```clojure
{:service "hoplite.blob"
 :operation "staging/append-from-source"
 :arguments [{:protocol "hoplite.blob-request/0-alpha"
              :operation "staging/append-from-source"
              ...}]}
```

This mirrors:

```clojure
(std.foundation.host/call service operation request)
```

A call plan cannot contain a native ABI, driver, path, credential, package,
callback, command, executable or remote catalogue.

## Authority boundary

Tahto's object kernel emits only closed values accepted by
`tahto.store.host/effect?`. The HAL capability adapter exposes a generic host
request only after a second exact-shape validation.

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

Native source handles are transport resources. A positive number alone grants
no authority. Production ingress and egress resolve a handle only through the
exact opaque request context, owning work and live handle registration.

## Generic blob profile

```text
service: hoplite.blob
request: hoplite.blob-request/0-alpha
result:  hoplite.blob-result/0-alpha
```

### Effect mapping

| Tahto effect | Generic operation |
| --- | --- |
| `upload/open` | `staging/open` |
| `upload/append` | `staging/append-from-source` |
| `upload/abort` | `staging/abort` |
| `upload/verify-install` | `staging/verify-commit` |
| `object/read-range` | `object/open-source` |

`manifest/verify` is intentionally not mapped. Manifest interpretation remains
HAL work and is tracked by #19 over a separate bounded small-value capability.

## Upload requests

### Open

```clojure
{:protocol "hoplite.blob-request/0-alpha"
 :operation "staging/open"
 :staging-key "upload.a"
 :expected-digest "sha256:..."
 :expected-size 4096
 :media-type "application/octet-stream"}
```

A staging key is bounded logical metadata, never a physical path. The provider
creates or resumes server-owned staging and reports its verified offset.

### Append

```clojure
{:protocol "hoplite.blob-request/0-alpha"
 :operation "staging/append-from-source"
 :staging-key "upload.a"
 :offset 0
 :length 4096
 :source-handle 17}
```

The host resolves the exact request/work source, consumes exactly `length`
bytes, rejects short or excess input and finishes it exactly once.

### Abort and install

`staging/abort` is mechanically idempotent. `staging/verify-commit` recomputes
the digest over actual staged bytes and atomically installs immutable content.
Tahto separately decides whether an application upload may abort or whether the
verified installation may enter its namespace graph.

`tahto.store.upload` treats every vault transition as candidate state until the
one matching generic result passes the same closed validator used by tests and
production.

## Immutable response sources

Tahto first authorizes and plans a half-open range, then calls:

```clojure
{:protocol "hoplite.blob-request/0-alpha"
 :operation "object/open-source"
 :digest "sha256:..."
 :offset 0
 :length 4096}
```

The provider result must match the exact digest, offset and length and contain no
extra fields. HAL translates it through the existing opened-range result and
projects only:

```clojure
{:protocol "hoplite.response-source/0-alpha"
 :source-handle 31
 :offset 0
 :length 4096}
```

The descriptor contains no Tahto coordinate, digest, provider, path, credential,
HTTP policy or request identity. It is ephemeral response-lifetime evidence and
must never enter objects, metadata snapshots, commits, heads, receipts, backups
or semantic roots.

Hoplite validates the descriptor against its request-scoped native source,
streams under Nginx backpressure and closes the source on every terminal path.

## Generic metadata profile

Metadata follows the same rule through:

```text
service: hoplite.store
request: hoplite.store-request/0-alpha
result:  hoplite.store-result/0-alpha
operations: load · initialize · compare-and-swap · receipt
```

The store persists opaque canonical values and receipts with exact revision CAS.
Tahto snapshot, transaction, replay and receipt meaning remains in HAL.

Injected and production execution both use the same preparation and exact result
validators. `nil` is accepted only where the operation profile allows absence.

The Hoplite memory and SQLite providers are now implemented. Tahto's `native/`
tree is frozen parity evidence and is removed by #17 after production transfer
and semantic recovery gates complete.

## Execution profiles

The same deterministic adapters run in:

1. pure HAL tests with injected memory capabilities;
2. Hara CLI hosts with installed providers; and
3. Hoplite workers with request/body/source resources bound to Nginx lifetimes.

A resolved promise or successful native status never bypasses HAL result
validation.

## Conformance laws

- Tahto application and semantic policy remains in HAL.
- Every mapped effect produces one exact generic request.
- Unknown or malformed effects expose no host call.
- HAL values cannot select a native ABI, provider, driver or path.
- Requests and results reject extra authority-bearing fields.
- Source authority is exact request context + work + handle.
- Native handles are never durable application state.
- Range requests use offset plus length across the generic boundary.
- Result translation requires exact originating request/result identity.
- Injected and production execution share validators.
- Provider cancellation and request cleanup close live sources exactly once.
- Memory and production providers are substitutable beneath the same HAL laws.
