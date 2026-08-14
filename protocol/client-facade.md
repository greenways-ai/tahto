# Tahto transport-neutral client facade

`tahto.client/0-alpha` is the reviewed Hara-facing entry point for embedding
hosts. It exports exactly these operations:

```text
tahto.semantic/read
tahto.semantic/prepare
tahto.semantic/submit
tahto.semantic/value-source
tahto.head/read
tahto.sync/plan
tahto.sync/push
tahto.sync/pull
tahto.sync/ack
```

## Closed envelope

Every call uses one closed `tahto.client-request/0-alpha` value containing only:

```clojure
{:protocol "tahto.client-request/0-alpha"
 :operation ...
 :coordinate {:application ...
              :namespace ...
              :collection ...}
 :payload ...
 :package-revision "sha256:..."
 :provider-revision 0}
```

The package revision must equal the exact package installed by the embedding
host. The provider revision must equal Tahto's current generic metadata state
revision. Neither field selects a provider package, driver, path, credential,
transport, route, callback, or native handle.

The host supplies one already-admitted, pending
`tahto.request-context/0-alpha` separately. The facade rejects completed,
replayed, cross-operation, cross-coordinate, changed-digest, changed-device,
changed-idempotency, and changed-authority contexts before dispatch.

## Authorised semantic operations

`tahto.semantic/read`, `tahto.semantic/prepare`, and
`tahto.semantic/submit` no longer carry a second facade-specific verification or
authority payload. Their payload contains exactly one complete closed
`tahto.semantic-service-request/0-alpha`:

```clojure
{:service-request
 {:protocol "tahto.semantic-service-request/0-alpha"
  :route ...
  :wire ...
  :authority ...
  :result-verification ...}}
```

The facade requires all of the following to agree exactly before using the
trusted authorised-service seam:

- the facade operation and selected semantic route;
- the facade coordinate, route coordinate, and signed wire coordinate;
- the facade provider revision and signed expected revision;
- the server-derived context device, operation, coordinate, request digest, and
  idempotency key;
- the imported authority decision root.

It then delegates through `tahto.semantic.service/dispatch-verified`; it does
not call the semantic read, prepare, or submit kernels directly. The trusted
embedding host is responsible for producing the supplied context only after the
exact signed wire has passed the reviewed request-verification boundary.
Transport adapters may serialize this same request and result, but may not
reconstruct or broaden the authority checks.

The retired submit payload containing caller-supplied `request-verification`,
`request-authority`, `input`, and `result-verification` fields is rejected.

## Semantic value sources

`tahto.semantic/value-source` contains exactly one
`tahto.semantic-value-source-request/0-alpha`. It is available only through the
reviewed `dispatch-with` host seam with one source-open callback supplied by the
installed host. The request cannot select that callback or any provider state.
The resulting `hoplite.response-source/1` descriptor remains request/work
scoped and is never persisted in Tahto state.

## Head and synchronization operations

`tahto.head/read` and the sync planning, push, pull, and acknowledgement
operations delegate directly to their existing bounded Tahto laws only under an
already-admitted exact request context. Head reads preserve every current commit
in a divergent head and never select a winner. Sync planning preserves existing
replay, coordinate, bounds, cursor, and divergence behavior.

## Result and security law

Successful portable values are wrapped in `tahto.client-result/0-alpha` with
the exact operation, coordinate, package revision, and provider revision.
Underlying typed failures are preserved unchanged so in-process and
transport-adapted clients observe the same canonical outcome.

The facade exposes no database query language, arbitrary Hara evaluation,
filesystem or network access, provider selection, route internals, credentials,
private keys, signing operation, native handles, automatic merge policy, or
application-domain semantics.
