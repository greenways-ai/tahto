# Stable semantic indexes and complete roots

## Decision

T-SF-03 adds immutable stable-ID selection and complete semantic collection roots
over the admitted semantic-object projections from T-SF-02.

The first release uses bounded sorted vectors and the existing Tahto object graph.
It does not introduce a second graph engine, a mutable global ID table, a new
collection mode or a scalable tree codec.

## Authority boundary

Tahto owns:

```text
stable ID uniqueness inside one immutable index
exact ID-to-semantic-object projection matching
typed-link ID/root consistency
exact schema-reference closure
named-root consistency
bounded graph closure
idempotent projection and edge publication
```

Applications choose which immutable versions belong in an index and what named
roots mean. Tahto does not infer a current version or reconcile sibling indexes.

## Semantic index admission

The pure operation consumes:

```clojure
(index/admit-projected
 state
 {:application "fixture.world"
  :namespace "world.a"
  :collection "scene"
  :root index-root
  :semantic-index
  {:protocol "tahto.semantic-index/1"
   :entries
   [{:id "entity/a" :root semantic-object-a}
    {:id "entity/b" :root semantic-object-b}]}})
```

The index object must already be installed and namespace-authorized. Every entry
root must have an admitted semantic-object projection, and the entry ID must equal
the projection ID exactly.

The index root cannot also be one of its semantic-object entries.

## Complete v1 link law

Every typed link in every selected semantic object must resolve through the same
index:

```text
link.target-id
  -> exactly one index entry

link.target-root
  == that entry's exact semantic-object root
```

Unresolved external links and target-ID/root disagreement fail closed. A future
cross-collection reference requires an explicit new profile rather than weakening
v1 completeness.

## Index projection and graph edge

The bounded durable projection is:

```clojure
{:root index-root
 :entries [...]}
```

It is stored in an additive table:

```clojure
:semantic-indexes {index-root projection}
```

The ordinary graph edge is:

```text
index-root
  -> every selected semantic-object root
```

Because semantic-object edges already include application `value-root` and typed
link targets, ordinary Tahto traversal closes over envelopes and application
values.

## Semantic root admission

The pure root operation consumes:

```clojure
(root/admit-projected
 state
 {:application "fixture.world"
  :namespace "world.a"
  :collection "scene"
  :root semantic-root-root
  :semantic-root
  {:protocol "tahto.semantic-root/1"
   :application "fixture.world"
   :namespace "world.a"
   :collection "scene"
   :schema-refs [...]
   :index-root index-root
   :roots
   [{:name "document"
     :id "entity/a"
     :root semantic-object-a}]}})
```

The input coordinate and the semantic-root coordinate must agree exactly. The
root object and selected index must be installed and namespace-authorized, and
the index must already have an immutable projection.

Every named root ID/root pair must equal the corresponding index selection.
Semantic-root, index and named semantic-object roles cannot reuse one immutable
root.

## Derived schema closure

Tahto derives the exact distinct schema-reference vector from every semantic
object selected by the index. It sorts those references using the field-wise
schema coordinate frozen by T-SF-01.

The submitted root's `:schema-refs` must equal that derived vector exactly.
Missing references, extra references and different order all fail. A semantic
root therefore cannot claim an unrelated schema package or omit one required to
revalidate its selected values.

## Root projection and graph edge

The bounded projection retains:

```clojure
{:root semantic-root-root
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :schema-refs [...]
 :index-root index-root
 :roots [...]}
```

It is stored in:

```clojure
:semantic-roots {semantic-root-root projection}
```

The ordinary graph edge is:

```text
semantic-root-root
  -> index-root
  -> every named semantic-object root
```

Before publication, Tahto evaluates the candidate edge through the existing
bounded closure traversal. Missing or overflowed descendants fail without state
mutation.

## Idempotency and collisions

Exact projection and exact edge replay succeeds without changing state.

A different projection for the same immutable root fails. A pre-existing graph
edge for an otherwise unprojected index or semantic root also fails rather than
being overwritten.

No global mutable `current-by-id` table is added. The immutable index is the
source of truth for one collection revision; bounded exact-ID lookup scans its
entries.

## Compatibility

- `tahto.object-vault-state/1` remains unchanged;
- semantic tables are absent until first use and accessed with empty defaults;
- indexes and roots remain ordinary immutable objects;
- `object-graph/1`, closure, GC, sync, backup and restore remain authoritative;
- no object, commit or head record gains fields;
- HPT1 or another scalable index can later use a new profile without rewriting
  v1 roots.

## Security laws

- every selected object and root is installed and namespace-authorized;
- stable IDs and immutable roots agree exactly;
- every v1 link resolves inside the selected index;
- schema references are derived from selected projections;
- candidate closure completes within existing bounds;
- projection publication cannot overwrite another graph role;
- provider, path, source-handle and complete application values never enter index
  or root metadata.

## Conformance

The permanent fixtures cover:

- complete cyclic two-object indexes;
- exact index and root projections and graph edges;
- application values reached through ordinary closure;
- missing projections and ID mismatches;
- unresolved link IDs and wrong link roots;
- missing or unauthorized index/root objects;
- immutable-root role conflicts;
- named-root selection mismatches;
- missing and excess schema references;
- exact replay, projection conflicts and edge conflicts;
- missing and overflowed candidate closure;
- closed inputs and authority-free durable projections.
