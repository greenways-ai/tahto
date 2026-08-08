# Semantic object admission

## Decision

T-SF-02 adds the first pure-HAL state transition for the Semantic Fabric.

The admission law consumes one already-installed semantic-object envelope, one
decoded `tahto.semantic-object/1` value and one exact
`tahto.canonical-value-verification/1` proof. It retains only a bounded semantic
projection and reuses Tahto's existing graph edges.

It does not read bytes, decode HTA, resolve packages, call a validator, dispatch
a host provider, create commits or expose a node route.

## Two immutable objects

Admission keeps the semantic envelope and its application value distinct:

```text
semantic-object-root
  canonical tahto.semantic-object/1 bytes

value-root
  canonical application value bytes
```

The envelope and value roots must not be equal.

The production boundary in #34 independently binds both decoded values to their
actual immutable bytes. T-SF-02 receives an already-decoded envelope and does not
claim to recompute its root.

## Input

The pure entry point is:

```clojure
(admit-projected
 state
 {:application "fixture.world"
  :namespace "world.a"
  :collection "scene"
  :root "sha256:semantic-object..."
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
   :value-size 512}})
```

The input is closed. Provider, path, source-handle, credential and callback
fields are rejected before mutation.

## Validation order

The transition requires:

1. a valid application, namespace and collection coordinate;
2. a canonical semantic envelope root;
3. one closed semantic-object value;
4. one closed canonical-value verification proof;
5. distinct envelope and application value roots;
6. the envelope root installed and namespace-authorized;
7. the value root installed and namespace-authorized;
8. installed value size equal to the envelope's declared size;
9. proof schema reference, value root and value size equal to the envelope;
10. every typed-link target installed and namespace-authorized.

Target roots need not yet have semantic projections. This permits all immutable
objects for a bounded cycle to be installed before either semantic envelope is
admitted.

Every failure returns the original state and no effects.

## Durable projection

The full application value is never copied into Tahto metadata. The projection
is:

```clojure
{:root "sha256:semantic-object..."
 :id "entity/tree-1"
 :schema-ref {...}
 :value-root "sha256:application-value..."
 :value-size 512
 :links [...]}
```

Projections are stored in an additive table:

```clojure
:semantic-objects
{semantic-object-root projection}
```

The table is absent from ordinary empty state and read through an accessor that
defaults to `{}`. Non-semantic state therefore retains its existing canonical
shape.

The collection coordinate authorizes the transition but is not copied into the
projection. One immutable semantic object may be selected by more than one
collection root; selection and current-version meaning belong to #32.

## Graph projection

Admission materializes:

```text
semantic-object-root
  -> value-root
  -> every exact typed-link target-root
```

Children are deterministic and duplicate-free: `value-root` appears first,
followed by target roots in the already-validated semantic-link order.

This edge is the ordinary Tahto graph edge. Backup, sync, restore, closure and
garbage collection therefore retain application value bytes and linked semantic
envelopes without a second semantic traversal engine.

## Idempotency and collisions

An exact replay succeeds only when both the stored projection and the graph edge
equal the expected values.

A different projection for the same immutable root fails. A pre-existing
non-semantic edge for the root also fails rather than being overwritten. These
checks prevent a decoded-value substitution or dual-use graph role from silently
changing closure.

Different immutable roots may retain the same stable logical ID. #32 decides
which exact version a semantic index selects.

## Authority boundary

Tahto owns:

```text
object and namespace authorization
closed envelope and proof validation
exact proof/envelope agreement
bounded projection
graph-edge mutation
idempotency and collision rejection
```

The exact specification package owns application value meaning. The generic
provider in #34 owns bounded byte reading, actual-byte SHA-256 verification and
canonical HTA decoding.

No production caller supplies a validator function to `admit-projected`.

## Compatibility

- the `tahto.object-vault-state/1` protocol is unchanged;
- ordinary states do not gain an empty semantic table;
- existing objects and graph edges are unchanged;
- no object record gains semantic fields;
- no collection mode, commit, head, signature or route changes;
- stable current selection remains outside this slice.

## Security laws

- all roots are canonical lowercase SHA-256 values;
- envelope, value and link targets are installed and authorized before mutation;
- envelope and application value roots are distinct;
- proof schema, root and size match exactly;
- existing edges cannot be overwritten;
- failed admission changes no state;
- complete application values and host authority never enter metadata;
- numeric handles cannot become semantic links or roots.

## Conformance

The permanent fixture covers:

- zero-link and multi-link admission;
- exact projection and graph children;
- absent semantic state before first admission;
- exact replay and projection conflicts;
- pre-existing edge conflicts;
- missing and unauthorized envelope/value/target roots;
- installed value-size mismatch;
- proof schema/root/size mismatch and malformed proof;
- installed but not-yet-admitted link targets;
- cyclic graphs admitted in either order;
- two immutable versions sharing one stable ID;
- closed inputs and authority-free durable projections.
