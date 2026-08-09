# Semantic roots over unchanged Tahto history

## Decision

T-SF-04 binds admitted `tahto.semantic-root/1` objects to the existing immutable
history model. It does not introduce a semantic commit, semantic head or second
ledger.

```text
admitted semantic root
  -> ordinary tahto.commit/1
  -> ordinary tahto.head/1
  -> existing divergence, CAS, replay and durable store plan
```

## Commit profile

A semantic revision is an unchanged commit:

```clojure
{:protocol "tahto.commit/1"
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :schema "tahto.semantic-root"
 :schema-version 1
 :parents [...]
 :objects [semantic-root]
 ...}
```

The first profile requires exactly one object. That object must have an admitted
semantic-root projection with the same application, namespace and collection.

Every parent must already be an accepted semantic commit for the same collection
profile. Ordinary commits and cross-collection parents fail before the existing
history transition runs.

After these profile checks, Tahto delegates to `tahto.store.history/accept-commit`.
The existing kernel remains authoritative for record verification, root identity,
namespace authorization, object closure, device sequence, parent existence,
tombstones, idempotency and graph edges.

## Head profile

Heads remain unchanged:

```text
linear       commits [current]
divergent    commits [left right]
merged       commits [merge]
```

Every selected commit must be an accepted semantic commit for the exact head
coordinate. The semantic helper then delegates to the existing signed head update
law.

The existing `:expected` commit set remains the only compare-and-swap boundary.
A stale update cannot discard one sibling root.

## Application-authored merge

Tahto does not inspect semantic fields or execute a merge callback.

An application may produce a new admitted semantic root and an ordinary commit
with both siblings as parents:

```text
left commit ---\
                -> application merge -> ordinary two-parent commit
right commit --/
```

Tahto validates the output root and parent profile. It does not choose the output.

## TAHTO-7 and generic persistence

The semantic helpers expose transitions compatible with the existing atomic
coordinator:

```text
semantic-history/execute-commit
semantic-history/execute-head
```

These call `tahto.store.transaction/execute` without changing request proof,
nonce, idempotency, result verification or metadata revision semantics.

A completed transition is prepared for durable publication through:

```text
semantic-history/prepare-store-commit
  -> tahto.store.provider/prepare-compare-and-swap
  -> {service "hoplite.store", operation "compare-and-swap", arguments [...]}
```

The call plan contains no driver, path, provider package or database identity.
Lost-result replay returns the exact prior result without re-running semantic
history mutation.

## Compatibility

- `tahto.commit/1` and `tahto.head/1` bytes are unchanged;
- signing contexts and verification records are unchanged;
- ordinary non-semantic history remains valid;
- existing sequence, closure, head, receipt and provider tests remain authoritative;
- old nodes may retain semantic objects without understanding the additive profile;
- no state table or core protocol is added by this slice.

## Security laws

- a semantic commit selects exactly one admitted semantic root;
- commit and root coordinates agree exactly;
- every parent is a same-coordinate semantic commit;
- every head commit is a same-coordinate semantic commit;
- existing closure completes before commit acceptance;
- head CAS cannot silently collapse divergence;
- no merge function, package selector, executable source or host authority enters
  a commit or head;
- durable publication remains atomic and request-bound.

## Conformance

The permanent fixture covers:

- semantic genesis and linear successor commits;
- wrong schema, version and root cardinality;
- missing or coordinate-mismatched semantic roots;
- ordinary and cross-collection parent rejection;
- sibling commits from separate devices;
- divergent heads and application-authored two-parent merge;
- stale head compare-and-swap;
- ordinary or cross-collection head commit rejection;
- TAHTO-7 execution and exact replay;
- generic `hoplite.store` compare-and-swap planning without installation authority.
