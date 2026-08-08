# Tahto semantic object admission

## Decision

The first semantic mutation admits one already-installed
`tahto.semantic-object/1` envelope, one exact canonical application-value proof
and one collection coordinate into Tahto's existing object graph.

```text
installed semantic envelope root
  + bounded decoded semantic-object value
  + tahto.canonical-value-verification/1
        |
        v
pure HAL admission
  -> bounded metadata projection
  -> value and typed-link graph edges
```

This slice defines the pure law. The generic bounded object reader that proves a
decoded envelope belongs to actual immutable bytes is #34.

## Authority boundary

Tahto owns:

- installed object and namespace authorization;
- semantic-envelope and proof-shape validation;
- exact schema-ref/value-root/value-size agreement;
- stable ID and typed-link projection;
- idempotency and immutable-root collision rejection;
- graph-edge materialization.

The installed specification package owns application-value invariants. The
generic host capability owns actual-byte digest verification and canonical HTA
decoding. Greenways OS owns package/provider installation and approval.

No caller supplies a validator function, package resolver, path, provider or
credential to the admission kernel.

## Input

The pure internal request is closed:

```clojure
{:application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :root "sha256:semantic-envelope..."
 :semantic-object
 {:protocol "tahto.semantic-object/1"
  :id "entity/tree-1"
  :schema-ref {...}
  :value-root "sha256:application-value..."
  :value-size 512
  :links [...]}
 :verification
 {:protocol "tahto.canonical-value-verification/1"
  :verified true
  :profile "hara.hta/1"
  :schema-ref {...}
  :value-root "sha256:application-value..."
  :value-size 512}}
```

`admit-projected` assumes the semantic-envelope map came from an exact bounded
decoding boundary. It does not claim to recompute `root`. The production
composition in #34 binds that decoded map to the installed immutable envelope
object before calling this law.

## Durable projection

Tahto retains only:

```clojure
{:root "sha256:semantic-envelope..."
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :id "entity/tree-1"
 :schema-ref {...}
 :value-root "sha256:application-value..."
 :value-size 512
 :links [...]}
```

The complete application value remains only in the immutable object at
`value-root`.

The projection is keyed by semantic-envelope root. Multiple immutable envelope
roots may carry the same logical `id`; current stable-ID selection is #32.

## Graph law

Admission projects:

```text
semantic envelope root
  -> value-root
  -> every link target-root
```

Including `value-root` in the edge set ensures closure, transfer, backup, restore
and GC retain the exact application value with its semantic envelope.

Every root must already be installed and namespace-authorized. Link targets need
not already be semantically admitted, allowing cyclic graphs to be installed in
any order before a complete collection root validates them.

## Acceptance order

The kernel rejects before mutation unless all laws hold:

1. exact closed request fields;
2. valid application/namespace/collection coordinate;
3. canonical semantic-envelope root;
4. valid `tahto.semantic-object/1` value;
5. installed and authorized envelope root;
6. installed and authorized application-value root;
7. exact installed value size;
8. valid proof matching schema ref, value root and size position-for-position;
9. every link target installed and authorized;
10. no conflicting prior semantic projection or edge claim.

Only then are the semantic projection and graph edges installed together in one
returned state value.

## Idempotency and conflicts

Repeating the exact same root, projection and child edge vector returns the
existing projection without changing state.

The same immutable root with a different projection is
`tahto.semantic/object-conflict`. A pre-existing non-semantic edge claim at the
same root is `tahto.semantic/edge-conflict`.

These failures preserve the original state exactly.

## Compatibility

- existing object descriptors remain unchanged;
- existing graph closure and GC traverse the new edges automatically;
- ordinary objects need not be semantically admitted;
- legacy states without `:semantic-objects` read it as an empty map;
- no collection mode, core record or signing profile changes.

## Security laws

- every admitted root shares the current namespace authorization closure;
- proof and envelope identity must agree exactly;
- complete application values are not duplicated in metadata;
- malformed or failed admission changes neither projection nor edges;
- source handles, callbacks, paths, providers and credentials are not
  representable in the semantic profiles;
- cyclic links do not bypass installation and authorization requirements.

## Follow-up boundaries

- #32 builds stable-ID indexes and complete semantic roots;
- #33 binds roots to ordinary commits and divergent heads;
- #34 supplies the production bounded canonical object verification boundary;
- #35 exposes authenticated semantic operations.
