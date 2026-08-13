# Tahto portable client facade

`tahto.client/0-alpha` is the transport-neutral entry point for embedding
hosts. It exports the following closed operations:

```text
tahto.semantic/read
tahto.semantic/prepare
tahto.semantic/submit
tahto.head/read
tahto.sync/plan
tahto.sync/push
tahto.sync/pull
tahto.sync/ack
```

All mutating client calls carry two distinct inputs before Tahto constructs a
request context:

- `tahto.request-verification/0-alpha` proves the signed request facts;
- `tahto.request-authority/0-alpha` records the already-composed Greenways OS
  decision that the exact device, operation, application, namespace,
  collection, and request digest are allowed.

Tahto compares those coordinates exactly and emits
`tahto.request-context/0-alpha` with the authority decision root. The context
means the request was admitted; it is not a grant and does not transfer
resource ownership. Namespace graph checks are reported as in-scope or
out-of-scope, while byte and record checks remain verification failures.

Every request carries an explicit application, namespace and collection
coordinate, the exact installed package digest, and the expected generic
provider revision. A verified Tahto device request context is supplied
separately by the embedding host. The facade requires the signed kernel
operation and coordinate to match before delegating to the existing semantic,
transaction or sync function.

The facade does not expose a database query language, provider selection,
paths, credentials, private keys, callback pointers, native handles, merge
policy or source evaluation. `tahto.head/read` returns every current commit in
a divergent head without selecting a winner.

Transport adapters serialize the returned values but do not reimplement these
checks. Provider compare-and-swap and signed request verification retain their
existing Tahto protocols.
