# Deterministic semantic prepare

`semantic.prepare` constructs exact unsigned semantic commit and head bodies from explicit application choices. It is a pure advisory operation: it does not sign, reserve authority, complete idempotency or change Tahto state.

```text
pending verified device context
  + admitted semantic root
  + explicit commit root, parents and sequence
  + explicit commit and head timestamps
  + explicit desired and expected head sets
  + expected metadata revision
        ↓
closed commit signing intent
  + closed head signing intent
```

## Why the commit root is explicit

The existing `tahto.commit/1` record includes its immutable root, while Tahto's HAL kernel deliberately does not claim to hash arbitrary records or hold signing authority. The client therefore supplies the proposed commit root as preparation evidence. A later installed verification provider must bind the signed canonical record to that root before `semantic.submit` accepts it.

Prepare does not treat a caller-supplied root as verified. It only ensures the root is canonical in shape, unused in current history and preserved exactly in both signing intents.

## Request

The closed request supplies every ambient value:

```clojure
{:semantic-root "sha256:..."
 :commit-root "sha256:..."
 :device "device.a"
 :sequence 3
 :commit-timestamp "2026-08-09T02:00:00Z"
 :parents ["sha256:left" "sha256:right"]
 :head-kind "main"
 :head-name "primary"
 :head-commits ["sha256:new"]
 :expected-head ["sha256:left" "sha256:right"]
 :head-updated-at "2026-08-09T02:01:00Z"
 :expected-metadata-revision 7}
```

The operation context supplies the exact application, namespace, collection, request digest and authenticated device.

## Preconditions

Tahto requires:

- a pending `semantic.prepare` device context from a currently enrolled device;
- the request device to equal the context device;
- the exact next accepted sequence for that device and collection;
- the exact current metadata revision;
- an admitted semantic root for the context coordinate;
- an unused proposed commit root;
- every parent to be an accepted semantic commit for the same coordinate;
- the exact current head vector as `:expected-head`;
- every retained old commit in `:head-commits` to come from that expected head;
- the new commit root to occur in `:head-commits`;
- bounded distinct digest vectors and valid explicit timestamps.

A wider head is rejected rather than truncated.

## Result

The result contains two signing intents. The commit body is an ordinary `tahto.commit/1` body without `:signature`; the head body is an ordinary `tahto.head/1` body without `:signature`.

```clojure
{:protocol "tahto.semantic-prepare-result/1"
 :device "device.a"
 :request-digest "sha256:..."
 :expected-metadata-revision 7
 :current-head-digest "sha256:..."
 :commit-intent
 {:protocol "tahto.semantic-signing-intent/1"
  :record-protocol "tahto.commit/1"
  :body {...}}
 :head-intent
 {:protocol "tahto.semantic-signing-intent/1"
  :record-protocol "tahto.head/1"
  :body {...}}}
```

The intents contain no signature, private key, provider, path, URL, credential, clock, generated nonce, callback or command.

## Concurrency and divergence

`:expected-head` is compare evidence, not a winner selection. `:head-commits` is an explicit application choice and must include the proposed commit. It may also retain one or more currently selected commits, allowing an application to preserve divergence deliberately.

Prepare never mutates the current head. `semantic.submit` must recheck both metadata revision and head compare-and-swap after installed providers verify the signed records.

## Compatibility

- existing commit and head record formats are unchanged;
- existing semantic history admission remains authoritative;
- no new durable table is introduced;
- ordinary non-semantic history is unaffected;
- HTTP routing and authentication realms remain later work;
- applications retain merge and reconciliation policy.
