# Signed two-device object transfer

## Decision

The first two-device release law composes existing Tahto kernels without adding
another core record or a native Tahto executor.

```text
device A verified requests
  -> pure-HAL upload orchestration
  -> immutable object and signed application reference
  -> ordinary signed commit and head
  -> signed node receipt
        |
        v
Tahto pull offer
  exact head roots and missing object closure
        |
        v
device B verified requests
  -> exact acknowledgement
  -> independent authorized range plan
  -> hara.blob object/open-source under B's work scope
  -> hara.response-source/1
```

A source handle belongs to one provider-owned request/work scope. It is never
part of the signed reference, commit, head, receipt, sync cursor or durable
Tahto state.

## Portable law fixture

`test/tahto/sync/two_device_object_test.hal` proves the application-neutral law
against the deterministic pure-HAL `hara.blob` reference provider.

The fixture:

1. enrols device A and device B with separate public keys;
2. authorizes A's object upload request;
3. installs an immutable payload, application-owned signed reference and commit
   object through the existing `tahto.store.upload` boundary;
4. projects the application reference edge to the payload digest;
5. accepts an ordinary signed `tahto.commit/1` and `tahto.head/1`;
6. retains an immutable signed `tahto.receipt/1`;
7. authorizes B's pull request and returns the exact payload/reference/commit
   closure behind the current head;
8. acknowledges the exact offered digest batch and advances B's cursor;
9. authorizes B's independent object read and returns one closed
   `hara.response-source/1` descriptor;
10. proves A's work cannot close B's source while B's exact work can; and
11. revokes B, rejects a fresh read request and preserves the historical signed
    commit, head and receipt.

The fixture-specific reference record is application data:

```clojure
{:protocol "fixture.object-reference/1"
 :root "sha256:..."
 :digest "sha256:..."
 :size 9
 :media-type "application/octet-stream"
 :signature {...}}
```

Tahto does not add this protocol to its core vocabulary or interpret its domain
meaning. Its content-addressed edge is fixture/application-owned.

## Authority boundary

Tahto owns:

- device enrolment and revocation identity;
- verified request context, nonces and idempotency;
- application, namespace and collection authorization;
- upload lifecycle and installation acceptance;
- content-addressed object closure;
- commit sequence, parent and signature-proof laws;
- head compare-and-swap and divergence preservation;
- pull offers, acknowledgements and monotonic cursors;
- range authorization and exact provider-result translation.

The generic host capability owns:

- source registration under one exact work;
- staged byte movement and immutable object custody;
- output source allocation and close lifecycle.

Greenways OS or a fixture host owns keys, provider installation and credentials.

## Security laws

- a numeric handle is insufficient authority;
- a source opened for device B's work cannot be closed through device A's work;
- signed application data contains no body handle, source handle or request
  context;
- the exact head closure, rather than arbitrary provider possession, determines
  B's pull offer;
- revocation blocks new requests without rewriting historical signed records;
- retry and completion evidence remains bound to exact request/result digests;
- Tahto core remains independent of the fixture application protocol.

## Production completion

This portable law is the first slice of issue #36. It does **not** claim to prove
filesystem durability, actual byte equality or Nginx behavior.

The production release gate still requires:

```text
real deterministic binary bytes
filesystem-backed hara.blob
container/worker removal and recreation
signed A publication
B sync through an independent execution scope
full-body and non-zero range byte comparison
slow-client backpressure
HEAD cleanup
SHA-256 and declared-length equality
cross-request/work/handle rejection
```

Those mechanics remain in generic Hoplite providers. The production fixture must
reuse the exact portable Tahto law rather than introduce product-specific fields
or another response representation.

## Relationship to the Semantic Fabric

The same substrate later carries a semantic root:

```text
signed semantic commit on A
  -> exact closure offered to B
  -> independently authorized immutable retrieval
  -> schema revalidation on B
```

The semantic object/index/root profiles are tracked by #29–#35. This release law
remains valid for ordinary non-semantic application objects.
