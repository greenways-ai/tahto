# Deterministic semantic preparation

`semantic.prepare` validates an already-admitted semantic root against the exact current collection state and returns closed signing intents. It does not sign, hash arbitrary records, reserve mutable authority or change Tahto state.

```text
pending verified device context
  + explicit semantic revision inputs
  + exact current metadata/head expectations
        ↓
commit signing intent
  + head continuation intent
```

## Why preparation is staged

An exact `tahto.head/1` body must contain the canonical root of the newly signed commit. That root is not available until the client has constructed the commit through the installed canonical signing boundary.

The first prepare result therefore contains:

- the complete semantic commit body except client-produced `root` and `signature`;
- an exact head continuation policy except the future signed commit root;
- the rule `head.commits = retained + signed-commit-root`.

The next prepare/submit slice accepts the verified signed commit, materializes the exact head body, and rechecks every expectation before atomic mutation.

## Request

```clojure
{:device "device.a"
 :semantic-root "sha256:..."
 :sequence 2
 :commit-timestamp "2026-08-09T01:10:00Z"
 :parents ["sha256:parent..."]
 :head-kind "main"
 :head-name "primary"
 :head-updated-at "2026-08-09T01:11:00Z"
 :expected-head ["sha256:current..."]
 :retain-head []
 :expected-metadata-revision 4}
```

Every field is explicit. Prepare does not call a clock, generate randomness, select a device, infer parents, choose a current branch or apply merge policy.

`:retain-head` contains existing current commit roots that the application deliberately wants the later signed head to retain. An empty vector selects only the future commit. Retaining sibling roots preserves divergence by explicit application choice.

## Commit intent

```clojure
{:protocol "tahto.semantic-commit-intent/1"
 :record-protocol "tahto.commit/1"
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :schema "tahto.semantic-root"
 :schema-version 1
 :device "device.a"
 :parents [...]
 :objects [semantic-root]
 :tombstones []
 :sequence 2
 :timestamp "2026-08-09T01:10:00Z"}
```

The intent deliberately has no `root`, `signature`, key, provider or command. The client and installed canonical signing boundary create the signed record.

## Head continuation intent

```clojure
{:protocol "tahto.semantic-head-intent/1"
 :record-protocol "tahto.head/1"
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :kind "main"
 :name "primary"
 :expected [...]
 :retained [...]
 :commit-insertion "append-signed-commit"
 :updated-at "2026-08-09T01:11:00Z"}
```

The later exact head must append the verified commit root after the retained roots. Submit rechecks the metadata revision and current head; preparation is advisory evidence, not a lease.

## Validation laws

- the device context is pending `semantic.prepare` and the device remains enrolled;
- context and request devices agree;
- expected metadata revision equals current state;
- the semantic root has an exact admitted projection for the current coordinate;
- sequence is exactly next for that device and collection;
- every parent is an accepted semantic commit for the same coordinate;
- current head, when present, is a valid semantic head;
- expected head roots equal the current set;
- retained roots are distinct and drawn from the current head;
- parents and retained-plus-future head stay within configured limits;
- every input and result profile is closed;
- all failures and successes return the original state unchanged.

## Authority boundary

Tahto owns deterministic validation and intent construction. Clients own private keys and signatures. Installed providers own canonical digest/signature verification. Applications own branch retention, parent selection and merge policy.

No path, URL, credential, provider, source handle, request context, private key, signature or executable callback enters a signing intent.
