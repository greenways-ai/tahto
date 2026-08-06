# Tahto object vault, version 1

TAHTO-3 implements the first local content-addressed storage profile for the Tahto fabric. It is an application-neutral, streaming filesystem/SQLite vault. Greenways OS grants access to application namespaces; applications decide what their objects mean.

## Storage layout

The default node root is compatible with the Greenways home layout:

```text
~/.greenways/tahto/
├── metadata.sqlite
├── objects/sha256/
│   └── ab/cdef…
└── tmp/uploads/
    └── <server-generated-upload-id>.part
```

An object destination is derived only from a validated lower-case SHA-256 digest. A request, application, or remote peer cannot choose a filesystem destination or an upstream URL.

SQLite stores metadata, namespace references, upload reservations, closure edges, and garbage-collection roots. The database uses WAL journaling, foreign keys, full synchronous durability, and explicit immediate write transactions.

## Immutable objects

An installed object is identified by:

```text
sha256:<64 lower-case hexadecimal characters>
```

Installation requires all of the following:

1. the declared size is within the configured object limit;
2. the upload has reached exactly the declared size;
3. the complete temporary file hashes to the declared digest;
4. any existing file at the digest path hashes to the same digest and size; and
5. the application/namespace quota permits the logical reference.

The verified temporary file is installed with an atomic rename into its digest-derived destination. The containing directory is synchronized before the metadata transaction completes.

A crash after the rename but before SQLite commit can leave a verified, unindexed immutable file. Retrying `finish_upload` is idempotent: it recognizes the destination bytes, verifies them again, and completes metadata installation. This failure mode leaks storage temporarily rather than losing or misidentifying data.

## Resumable upload contract

An upload is server-assigned state containing:

```text
upload ID
application
namespace
expected digest
expected size
media type
authoritative committed offset
created and updated timestamps
```

The caller appends at an exact offset. A stale or conflicting offset is rejected. Each read is bounded by `max_upload_chunk_bytes`; the implementation never reads an unbounded request into memory.

On macOS and Linux, a per-upload advisory file lock prevents concurrent processes from modifying the temporary file before SQLite's offset compare-and-swap rejects a stale writer. If a crash leaves file bytes beyond the committed SQLite offset, the file is truncated back to the committed offset before resumption.

Active uploads reserve their entire expected size against the application/namespace quota. Concurrent unfinished uploads therefore cannot oversubscribe a namespace and rely on later digest deduplication to escape accounting.

## Existence negotiation

`missing(digests)` validates every digest, removes duplicate requests while preserving order, and returns only objects absent from both metadata and the derived object path. It does not accept paths, URLs, object names, or application-specific identifiers.

## Range reads

Range reads use a validated object digest plus a half-open byte interval:

```text
[start, end-exclusive)
```

The interval must be within the verified metadata length. Reads are yielded in configured bounded chunks. HTTP `Range` parsing and `206 Partial Content` response planning belong to Hoplite's native data-plane adapter; the vault provides the seekable native object source.

## Application and namespace quotas

Object bytes are globally deduplicated by digest. Quotas are logical and apply independently to each `(application, namespace)` reference set.

- Referencing the same digest twice in one namespace counts once.
- Referencing it from another namespace consumes that namespace's logical quota.
- An active upload reserves its declared size.
- Reducing a quota below committed usage plus active reservations is rejected.

Tahto does not infer application identity from a path or object payload. The caller must already hold an OS-issued grant for the application and namespace; transport enforcement is wired through HOPLITE-1 and device enforcement through TAHTO-5.

## Bounded chunk manifests

The first manifest profile is:

```text
tahto.chunk-manifest/1
```

Its canonical JSON body contains:

```json
{
  "protocol": "tahto.chunk-manifest/1",
  "mediaType": "application/octet-stream",
  "totalSize": 1234,
  "chunks": [
    {"digest": "sha256:…", "size": 512},
    {"digest": "sha256:…", "size": 722}
  ]
}
```

The manifest is itself an immutable object. Closure edges preserve chunk order by ordinal and deliberately allow the same digest at multiple ordinals. Repeated chunks are a valid content-addressed representation and must not be collapsed from the manifest sequence.

A manifest can reference no more than `max_manifest_chunks`, and every child must already exist before edges are registered.

## Closure verification

Closure traversal is iterative and bounded by `max_closure_objects`. It reports the complete missing digest set rather than silently accepting a partial graph. Callers may request byte verification for every visited object when creating a durable boundary such as a backup.

Tahto core verifies presence and integrity. It does not interpret an application's payload graph or choose application merge semantics.

## Garbage-collection roots

The initial safe root set is the union of:

- application/namespace object references; and
- explicit `application`, `head`, `backup`, or `retention` roots.

Garbage collection computes the complete reachable closure. It is dry-run by default. Applying collection removes metadata transactionally before unlinking immutable files. A failed unlink leaves an unindexed storage leak, not a loss of reachable data.

TAHTO-4 replaces generic head and backup root usage with atomic signed head and immutable backup records. This PR supplies the closure mechanism they rely on.

## Operator interface

The dependency-free operator CLI is:

```sh
bin/tahto-vault --root ~/.greenways/tahto init
bin/tahto-vault --root ~/.greenways/tahto quota-set app.example profile.primary 1073741824
bin/tahto-vault --root ~/.greenways/tahto put app.example profile.primary ./archive.pack
bin/tahto-vault --root ~/.greenways/tahto missing sha256:… sha256:…
bin/tahto-vault --root ~/.greenways/tahto read sha256:… --start 0 --end 1024
bin/tahto-vault --root ~/.greenways/tahto verify-object sha256:…
bin/tahto-vault --root ~/.greenways/tahto verify-closure sha256:…
bin/tahto-vault --root ~/.greenways/tahto gc
bin/tahto-vault --root ~/.greenways/tahto gc --apply
```

A path accepted by `put` is an operator-selected local source file. It never determines the vault destination, which remains digest-derived.

## Security laws

A conforming object-vault adapter must never:

- install bytes before verifying their complete SHA-256 digest;
- derive a destination from an application-controlled path component;
- accept upper-case, truncated, alternate-algorithm, or path-like digests;
- let global deduplication bypass per-namespace quota accounting;
- treat a temporary upload as an authoritative object;
- collect an object reachable from an application, head, backup, or retention root;
- invent application-specific closure meaning; or
- materialize large object bytes as ordinary Hara values.

## Deferred integration

This library is the node-local data plane. The following remain separate PRs:

- Hoplite/Nginx request-body and ranged-response wiring;
- signed device and nonce enforcement;
- stable commit persistence and atomic head updates;
- immutable backup records and restore manifests;
- service workers and durable jobs; and
- remote replication and sealed replicas.
