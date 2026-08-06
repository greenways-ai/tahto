# Tahto core records, version 1

TAHTO-2 defines the stable application-neutral records used by the first Tahto fabric. The normative machine schema is [`schema/tahto-core-1.schema.json`](schema/tahto-core-1.schema.json).

## Scope

Core records identify nodes, devices, applications, namespaces, collections, content-addressed objects, commits, heads, backups, receipts, services, and jobs. Application payloads remain in referenced objects. Core records never grow Historia-, Hestia-, Worlds-, Spaces-, Hodos-, or Ignatius-specific fields.

Every core object is a closed record. Unknown top-level fields are rejected. Applications evolve their own payload schemas through the `schema` and `schemaVersion` fields on collections and commits rather than extending the Tahto envelope.

## Record registry

| Protocol | Purpose |
|---|---|
| `tahto.node/1` | Node identity, endpoints, and advertised fabric features |
| `tahto.device/1` | Enrolled or revoked device identity at one node |
| `tahto.application/1` | Installed application identity, publisher, version, and lock digest |
| `tahto.namespace/1` | Application-owned isolation boundary |
| `tahto.collection/1` | Named state collection and its storage mode |
| `tahto.object/1` | Immutable SHA-256-addressed object descriptor |
| `tahto.commit/1` | Signed transition that references parents, object roots, and tombstones |
| `tahto.head/1` | Signed named set of current commit roots |
| `tahto.backup/1` | Immutable pin over a complete verified closure |
| `tahto.receipt/1` | Node-signed evidence for storage and state operations |
| `tahto.service/1` | Inert registration for an application-owned worker |
| `tahto.job/1` | Durable application-worker job state |

## Identifiers and digests

Identifiers are lower-case, stable protocol names. They may contain `.`, `_`, `:`, `/`, and `-`, but are not filesystem paths and must never be used directly to choose a storage location.

Content digests use:

```text
sha256:<64 lower-case hexadecimal characters>
```

A digest identifies bytes, not an application meaning. The object vault verifies bytes before atomic installation.

## Collection modes

The initial collection-mode vocabulary is closed:

| Mode | Meaning |
|---|---|
| `snapshot/1` | A succession of complete application snapshots |
| `event-log/1` | An append-oriented sequence of application events |
| `object-graph/1` | Application-owned objects connected by digest references |
| `git-dag/1` | Git-compatible object and ref closures |
| `derived/1` | Rebuildable indexes, caches, vectors, thumbnails, or projections |

`derived/1` collections must declare `authority: rebuildable`. They are not replicated or backed up by default. New collection modes require a Tahto protocol revision and conformance fixtures; an application cannot introduce a private core mode name.

## Commit contract

A `tahto.commit/1` contains exactly the application-neutral transition fields:

```text
application identity
namespace
collection
schema and version
device identity
parent commit roots
object roots
tombstones
sequence
timestamp
signature
```

The `root` is the SHA-256 digest of the canonical unsigned commit body with `root` and `signature` omitted. The signature signs the context string `tahto.commit/1`, a newline, and that root.

A commit does not state how an application merges or interprets its objects. Tombstones are application-owned references and Tahto does not reinterpret them.

## Canonical signing profile

Signed records use `tahto-signature/1`:

```json
{
  "profile": "tahto-signature/1",
  "algorithm": "ed25519",
  "keyId": "device.a",
  "value": "<base64url signature>"
}
```

The canonical JSON profile is `tahto-canonical-json/1`:

- UTF-8 encoding;
- object keys sorted lexicographically by Unicode code point;
- no insignificant whitespace;
- arrays retain their declared order;
- integers use base-10 without leading zeroes;
- floating-point numbers are not part of core records; and
- strings use JSON escaping without Unicode normalization.

For a signed record other than a commit, the signing digest is SHA-256 over the canonical record with `signature` omitted. The signature signs the protocol identifier, a newline, and the textual digest. TAHTO-5 implements request and record verification; TAHTO-2 fixes the envelope and fixtures.

## Heads and conflicts

A head is a signed named set of one or more commit roots. More than one current root is valid and represents divergence:

```json
{
  "protocol": "tahto.head/1",
  "kind": "main",
  "name": "main",
  "commits": [
    "sha256:...",
    "sha256:..."
  ]
}
```

Tahto preserves every valid divergent root. It does not choose a winner, apply last-write-wins, synthesize an application merge commit, or discard a branch. A later application-authored commit may reconcile the branches.

The optional `expected` field records the roots against which a compare-and-swap update was attempted. TAHTO-4 defines atomic head-update behavior.

## Backups

A synchronized head and a backup point are different records. A `tahto.backup/1` is immutable and pins one or more complete verified roots under a retention label. Creation must fail when any referenced closure is incomplete. TAHTO-4 adds receipts and restore manifests around this contract.

## Services and jobs

A service record is inert metadata pinned to:

```text
application
service protocol
worker version
package or binary digest
allowed collections
allowed operations
resource policy
```

The implementation remains in the application repository and is installed through a separately authorized path. A Tahto service record cannot introduce JavaScript, HTML, HAL, arbitrary Wasm, or a native command into Greenways OS.

Job states are closed:

```text
queued
running
blocked
completed
failed
cancelled
```

State transitions and idempotency are implemented in TAHTO-6.

## Conformance

`conformance/check-protocol.py` provides a dependency-free executable subset of the JSON Schema contract. It proves:

- every required protocol has one closed record definition;
- collection modes are exactly the closed initial vocabulary;
- the commit required-field set is unchanged;
- valid fixtures cover all twelve records;
- a head can retain two divergent roots;
- application-specific fields are rejected;
- private collection modes are rejected;
- unsigned commits are rejected;
- duplicate heads are rejected;
- derived collections cannot claim source authority; and
- unknown job states are rejected.

The fixtures are examples of envelopes only. Their keys and signatures are deliberately non-production test values.
