# Atomic semantic submission

`semantic.submit` binds a prior deterministic prepare result to exact verified signed commit and head records, then applies both records as one TAHTO-7 metadata transition.

```text
verified semantic.submit request
  + tahto.semantic-prepare-result/1
  + verified tahto.commit/1
  + verified tahto.head/1
        ↓
revalidate current revision and head
  -> accept commit
  -> compare-and-swap head
  -> complete idempotency
  -> one generic hara.store compare-and-swap
```

Commit acceptance and head replacement are atomic. A head failure returns the original state, including the original device sequence, sequence slot, commit table and edge table.

## Closed input

```clojure
{:prepared prepare-result
 :commit-verification commit-record-verification
 :head-verification head-record-verification}
```

The outer signed `semantic.submit` request digest binds this complete value. The input cannot select a provider, path, URL, credential, key, callback, source handle or command.

## Revalidating preparation

Submit reconstructs the original prepare request from the two closed intents and reruns the pure `semantic.prepare` law against current state.

The reconstructed prepare context retains:

- the original prepare request digest;
- the submit device and collection coordinate;
- the exact prepared metadata revision, semantic root, sequence, timestamps, parents, head expectation and retained roots.

The regenerated result must equal the supplied prepare result exactly. A stale revision, changed head, changed sequence, removed semantic root or altered prepared field therefore fails before commit acceptance.

Prepare remains advisory evidence. It is never a lease.

## Commit proof binding

The verified signed commit must equal the prepared commit intent for every field except the client-produced content root and signature:

```text
application / namespace / collection
schema / schema-version
device / parents / objects / tombstones
sequence / timestamp
```

The verification digest must equal the signed record's `root`. Existing semantic commit admission then rechecks object installation, namespace authorization, closure, parents and the next sequence slot.

## Head proof binding

The verified signed head must preserve the continuation intent exactly:

```text
application / namespace / collection
kind / name
expected current commit set
updated-at
commits = retained roots + signed commit root
```

The existing signed-head law remains authoritative for compare-and-swap, commit profile, ancestry, closure and head-root replacement.

## Atomic transition

The pure transition applies the commit to a candidate state and then applies the head to that candidate. If either step fails, it returns a failure over the original state.

On success it returns only:

```clojure
{:protocol "tahto.semantic-submit-result/1"
 :device "device.a"
 :commit-root "sha256:..."
 :head-digest "sha256:..."
 :head-kind "main"
 :head-name "primary"
 :commits ["sha256:..."]
 :previous ["sha256:..."]}
```

No complete application value, private key, signature, provider configuration or transport authority is copied into the result.

## TAHTO-7 and durable storage

`semantic.submit/execute` delegates to `tahto.store.transaction/execute` with an explicit current expected revision.

The coordinator owns:

- verified device authorization;
- nonce and idempotency evidence;
- transition result verification;
- exact replay without rerunning the semantic mutation;
- one metadata revision increment.

A completed retry supplies the current metadata revision and a fresh nonce while preserving the same request digest and idempotency key. The coordinator returns the prior result digest without re-entering the submit transition.

`semantic.submit/prepare-store-commit` delegates to `tahto.store.provider/prepare-compare-and-swap`. Durable publication remains one application-neutral `hara.store` request containing opaque state and receipt evidence.

## Security laws

- the transition context is exactly pending `semantic.submit`;
- submit and prepared devices agree;
- every prepared field is regenerated from current state;
- commit and head proofs cannot substitute roots, parents, sequence, timestamps, expected head or retained branches;
- commit success followed by head failure leaves no commit mutation visible;
- invalid result evidence rolls back authorization and domain mutation;
- exact replay does not rerun commit or head acceptance;
- durable publication contains no driver, path, credential or application-specific provider selection;
- private keys remain client-side.

## Not included

- HTTP routing or the required semantic authentication realm;
- server-side signing;
- automatic merge, parent inference or branch selection;
- canonical application-value decoding;
- production signed ingress, tracked by Hoplite #73.
