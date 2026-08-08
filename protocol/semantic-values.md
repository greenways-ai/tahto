# Tahto semantic value profiles, version 1

## Decision

The first Semantic Fabric release defines closed, application-neutral values for
exact schema identity, stable logical identity, typed content-addressed links and
complete collection roots.

This slice defines values and pure predicates only. It does not read object
bytes, invoke validators, mutate Tahto state or expose service routes.

## Authority boundary

Tahto validates:

```text
closed field sets
bounded identities and collections
canonical lowercase SHA-256 roots
exact immutable package references
deterministic vector ordering
```

Installed specification packages own:

```text
application fields and invariants
canonical value acceptance
migrations and transforms
merge behavior
executable interpretation
```

Greenways OS and trusted package installation own package approval and local
resolution. A schema reference cannot install, fetch or select executable code.
Runtime validation does not call the public Specs service.

## Application values remain separate

A semantic object does not inline an arbitrary application value:

```text
canonical application value bytes
  -> immutable object at value-root

semantic object
  -> stable ID
  -> exact schema reference
  -> value-root and declared size
  -> typed links
```

This keeps every semantic envelope bounded and prevents functions, cycles,
native handles, callbacks or host objects from entering through an inline value
field.

## `tahto.schema-ref/1`

```clojure
{:protocol "tahto.schema-ref/1"
 :schema "greenways.world.entity"
 :schema-version 1
 :package "greenways/world-specs"
 :package-version "0.1.0"
 :package-root "sha256:..."
 :entry "greenways.world.entity/spec"}
```

Fields:

- `schema` — stable application-neutral schema identity;
- `schema-version` — positive integer schema version;
- `package` — exact Hara package coordinate;
- `package-version` — immutable version beginning with a decimal digit;
- `package-root` — canonical SHA-256 root of the installed immutable package;
- `entry` — reviewed exported Hara validator value.

There is no URL, branch, registry origin, path, provider, credential or command.
The package root is mandatory.

## `tahto.canonical-value-verification/1`

```clojure
{:protocol "tahto.canonical-value-verification/1"
 :verified true
 :profile "hara.hta/1"
 :schema-ref {...}
 :value-root "sha256:..."
 :value-size 512}
```

This is a closed proof shape consumed by later admission work. It states that
the exact immutable bytes at `value-root` were canonically decoded under the
named profile and accepted by the exact schema package entry.

T-SF-01 validates only the proof envelope. The generic bounded value provider and
installed validator that produce it are tracked separately by #34.

The initial value-size ceiling is 1 MiB. Larger media or dense engine data must
remain in specialist object formats and may be linked from a bounded semantic
value.

## `tahto.semantic-link/1`

```clojure
{:protocol "tahto.semantic-link/1"
 :role "world.parent"
 :target-id "world/main"
 :target-root "sha256:..."}
```

A link binds a stable application-owned role and target ID to one exact immutable
semantic-object root. It does not select a resolver, callback or runtime object.

Link vectors contain at most 256 entries and are strictly ordered by:

```text
role -> target-id -> target-root
```

Strict ordering rejects duplicates and avoids silently normalizing signed data.
An empty link vector is valid.

## `tahto.semantic-object/1`

```clojure
{:protocol "tahto.semantic-object/1"
 :id "entity/tree-1"
 :schema-ref {...}
 :value-root "sha256:..."
 :value-size 512
 :links [{...}]}
```

The logical ID remains stable across immutable object versions. The object points
to one canonical application value and carries only typed semantic links.

The object contains no inline `value`, source handle, path, callback, executable
source or provider metadata.

## `tahto.semantic-index/1`

```clojure
{:protocol "tahto.semantic-index/1"
 :entries
 [{:id "entity/tree-1" :root "sha256:..."}
  {:id "world/main" :root "sha256:..."}]}
```

An index maps stable logical IDs to exact semantic-object roots. It contains at
least one and at most 4,096 entries. Entries are strictly ordered by `id`, which
also rejects duplicate logical IDs.

This first bounded vector is deliberately simple. It does not allocate a new
tree codec or block the release on HPT1.

## `tahto.semantic-root/1`

```clojure
{:protocol "tahto.semantic-root/1"
 :application "fixture.world"
 :namespace "world.a"
 :collection "scene"
 :schema-refs [{...}]
 :index-root "sha256:..."
 :roots
 [{:name "document"
   :id "world/main"
   :root "sha256:..."}]}
```

A semantic root names one existing Tahto collection coordinate, the exact schema
packages used by its values, one immutable index and one or more named entry
objects.

Schema references are strictly ordered by their complete schema/package
coordinate and limited to 64. Named roots are strictly ordered by name and
limited to 64.

The root does not contain a commit, head, device, signature or provider. #33
binds verified roots to the existing `tahto.commit/1` and `tahto.head/1` laws.

## Deterministic ordering

Validators reject unsorted input rather than sorting it:

```text
schema refs  full schema/package coordinate
links        role, target ID, target root
index        logical ID
named roots  name
```

Comparison is field-wise. String fields use lexical ordering and
`schema-version` uses numeric ordering. Coordinates are not flattened into a
delimiter-joined string, so ordinary prefix ordering such as `a` before `aa`
remains stable.

All nested maps are closed. Unknown fields fail even when the extra value appears
inert.

## Limits

```text
schema references          64
links per semantic object  256
index entries              4,096
named roots                64
canonical value bytes      1,048,576
```

The limits are protocol values in `tahto.semantic.model` and have exact boundary
coverage.

## Compatibility

- existing Tahto core protocols and JSON Schema remain unchanged;
- no collection mode is added;
- semantic collections use existing `object-graph/1` and application-owned
  `schema`/`schemaVersion` fields;
- ordinary non-semantic collections remain valid;
- a later general Hara schema-reference contract requires an explicit
  compatibility profile rather than silently rewriting stored values.

## Security laws

- exact immutable package roots are mandatory;
- mutable package aliases such as `latest` are invalid;
- package locations and providers are not representable;
- application value bytes remain behind immutable roots;
- response-source and request-body handles are not semantic values;
- no host object or executable callback is accepted;
- deterministic ordering prevents duplicate substitution;
- dense voxels, samples and bytes are not expanded into individual semantic
  objects.

## Follow-up boundaries

- #31 admits verified objects and projects their links into the existing graph;
- #32 builds and selects stable indexes and complete roots;
- #33 uses ordinary commits and divergent heads;
- #34 produces canonical value verification through a generic bounded provider;
- #35 exposes authenticated semantic read, prepare and submit operations.
