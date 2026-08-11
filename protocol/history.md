# Tahto signed history, heads and backup profile, version 1

TAHTO-4 adds the application-neutral history layer above the TAHTO-3 object
vault. The authoritative transition logic is Hara under
`tahto.store.history`. It accepts only records that have already been bound to
their canonical bytes and signature by a trusted verifier.

## Authority boundary

```text
Greenways OS / installed key provider
  authorises the application and verifies canonical signed records

Tahto Hara history kernel
  validates record shape and coordinates
  accepts immutable commits
  advances heads with compare-and-swap
  preserves divergent valid commits
  pins immutable backup closure
  produces deterministic restore plans

Tahto object graph
  proves that every pinned commit and object closure is locally complete

Applications
  decide payload meaning, reconciliation and which divergent commits to merge
```

Tahto never chooses an application merge winner. A head containing two valid
commit roots remains a head containing two roots until an application submits a
new signed commit or head record.

## Record verification boundary

A production verifier returns:

```clojure
{:protocol "tahto.record-verification/0-alpha"
 :record-protocol "tahto.commit/0-alpha"
 :verified true
 :canonical-digest "sha256:..."
 :key-id "key.device-a"
 :record {...}}
```

The proof means that the provider:

1. decoded the exact bounded record bytes;
2. applied `tahto-canonical-json/0-alpha`;
3. recomputed the canonical SHA-256 digest;
4. verified the record's `tahto-signature/0-alpha` signature;
5. resolved the signing key and revocation state; and
6. projected no private key or bearer credential into Hara.

TAHTO-4 validates and consumes this proof but does not implement key enrolment,
cryptographic verification or revocation. Those remain TAHTO-5. Tests use
explicit proof fixtures rather than pretending that a structural signature is
cryptographically verified.

For content-addressed commit, backup and receipt records, Tahto first emits or
returns a bounded `tahto.record-verification-request/0-alpha` for an object already
authorised in the application namespace. Mutable head records also use the
same proof profile, but their canonical digest is stored separately from the
head coordinate.

## Immutable commits

A `tahto.commit/0-alpha` is accepted only when:

- its `root` equals the verifier's canonical digest;
- application and namespace match the authorised request context;
- its commit object is reachable in that namespace;
- parent, object and tombstone lists are unique and bounded;
- every parent is an accepted commit in the same application, namespace and
  collection;
- every object root is authorised in the namespace;
- every object graph closure is complete and within the configured bound;
- one digest is not both an object and a tombstone;
- the device-local sequence is exactly the next accepted sequence; and
- a device sequence slot cannot be reused for another root.

The device sequence is independent of commit ancestry. A newly enrolled device
may create its first device-sequence commit on top of commits from another
device. Tahto preserves both the device sequence and the application commit
DAG without interpreting application merge semantics.

An accepted commit adds graph edges from the commit object to its parent commit
objects and application object roots. The object, commit and manifest graphs
therefore share one bounded closure traversal for head pinning, backup pinning,
restore and garbage-collection safety.

Replaying the identical verified commit is idempotent. Reusing one root with a
different record is a conflict.

## Signed compare-and-swap heads

A `tahto.head/0-alpha` update must include an `expected` commit set. The transition is:

```text
read current signed head
compare current commits to expected commits
verify every proposed commit coordinate and complete closure
atomically replace the head and its GC root
```

The comparison treats commit arrays as sets for the concurrency decision while
retaining the exact signed record bytes and order as evidence. A stale
`expected` set fails without changing state.

A head may contain multiple commit roots. This is the normal representation of
concurrent valid work:

```clojure
{:kind "main"
 :name "primary"
 :commits [commit-from-device-a
           commit-from-device-b]
 :expected [previous-commit]}
```

Tahto does not apply last-write-wins, timestamp ordering, lexicographic choice
or an implicit merge. The application may later publish a signed merge commit
and advance the head through another compare-and-swap.

The signed head record is stored by its application/namespace/collection/kind/
name coordinate. Its canonical verification digest is stored separately. A
replay of the exact same verified head is idempotent even though its original
`expected` set is no longer current.

## Immutable backups

A `tahto.backup/0-alpha` is a content-addressed signed record whose `id` equals its
canonical verification digest. A backup is accepted only when:

- its object is authorised in the request namespace;
- every root is an accepted commit for the declared application;
- commit ancestry is complete and bounded;
- the complete object closure is locally present; and
- the same backup digest has not already been assigned different roots,
  retention or signature meaning.

Acceptance adds an edge from the backup record object to all commit roots and
installs an explicit backup GC root. The backup may include several collections
or namespaces of one application without asking Tahto to interpret any of
them.

Releasing retention removes only the explicit root. The immutable backup record
remains as evidence and may be repinned by accepting the same verified record.

## Restore plans

`restore-plan` walks the exact backup closure and returns:

```clojure
{:protocol "tahto.restore-manifest/0-alpha"
 :backup "sha256:..."
 :application "app.example"
 :complete true
 :overflow false
 :commits [...]
 :objects [...]
 :missing []}
```

The plan is deterministic, bounded and application-neutral. It does not mutate
heads, choose a winning branch, decode payloads or run application code. A later
restore executor transfers missing immutable objects, re-verifies them and
submits explicit signed head changes.

## Receipts

A verified `tahto.receipt/0-alpha` may be stored as immutable evidence for actions such
as `commit.accept`, `head.update`, `backup.pin` and `restore.manifest`. The
history transition returns receipt action and subject intents, but it does not
forge a node signature. An authorised node signer must produce the receipt and
the verifier must return the normal record-verification proof before storage.

## Transaction rule

A durable implementation applies one transition as:

```text
read metadata snapshot
  -> verify exact signed record through the installed provider
  -> evaluate the deterministic Hara transition
  -> verify all object and ancestry closure
  -> atomically commit metadata and root changes
```

The head comparison and root replacement belong to one metadata transaction.
No observer may see the new head without its corresponding closure root.

## Security laws

- no unsigned or unverified record enters authoritative history;
- canonical record digest, signed key identity and record signature key agree;
- commit roots and backup IDs are immutable content addresses;
- device sequence slots cannot be reused;
- stale head expectations cannot overwrite concurrent work;
- divergent valid heads are preserved rather than silently resolved;
- head and backup closure must be complete before pinning;
- backup release never deletes immutable evidence directly;
- restore planning is inert data and cannot execute application payloads;
- application merge semantics remain outside Tahto core; and
- private keys, bearer credentials, paths, upstreams and raw object bytes do
  not enter Hara history values.

## Conformance

`test/tahto/store/history_test.hal` proves:

- bounded content-addressed record verification requests;
- namespace isolation before record disclosure;
- verified immutable commit acceptance;
- idempotent commit replay;
- strict device sequence progression;
- parent coordinate validation;
- authorised and complete object closure;
- verification key/signature key agreement;
- signed head compare-and-swap;
- preservation of divergent valid commits;
- idempotent signed head replay;
- rejection of incomplete head closure;
- immutable backup closure pinning;
- backup digest conflict rejection;
- deterministic restore manifests;
- retention release without evidence deletion; and
- immutable verified receipt storage.
