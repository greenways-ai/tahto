# Tahto

**Tahto is the Greenways application-state and semantic fabric.** It stores,
synchronizes, backs up, restores and routes application state while Greenways OS
retains installation, consent, private-key, credential and grant authority.

```text
Greenways OS applications
          │ exact grants and verified requests
          ▼
       Tahto HAL
  objects · commits · heads
  sync · backup · restore
          │ closed generic capability calls
          ▼
  hara.store · hara.blob
          │ trusted host installation
          ▼
 SQLite · filesystem · Nginx
```

Applications and specification packages own the meaning of their values. Tahto
owns application-neutral identity, custody, graph closure, immutable history,
divergent heads and recovery. It does not invent a universal merge policy.

The next release train extends this existing substrate with exact schema
references, stable semantic identities and typed content-addressed links. See
[#29](https://github.com/greenways-ai/tahto/issues/29).

## Node surface

The current Hara/Hoplite control-plane application exposes:

```text
GET /.well-known/tahto
GET /tahto/v1/health
GET /tahto/v1/status
POST /tahto/v1/pairing/prepare
POST /tahto/v1/pairing/complete
```

The loopback operator command `bin/tahto invite` issues a short-lived
`invite.*~token` value through the non-advertised management route. Only the
token digest enters durable metadata. Greenways OS sends the raw token to
prepare, signs the exact returned intent, and completes enrolment without
granting administrator or application authority.

One bounded compatibility release also exposes:

```text
GET /.well-known/greenways-beacon
GET /beacon/v1/health
GET /beacon/v1/status
```

These are discovery and status routes. The authenticated semantic read/prepare/
submit surface is a later additive slice; the node does not currently advertise
it as installed.

## Repository layout

```text
src/tahto/node/       Hoplite control-plane application
src/tahto/protocol/   application-neutral records and verification contracts
src/tahto/store/      objects, history and generic capability orchestration
src/tahto/sync/       device and replication laws
src/tahto/service/    inert service descriptors and durable jobs
protocol/             normative protocol and integration documents
test/                 executable Hara conformance suites
conformance/          dependency-free record and architecture guards
adapters/             optional integrations outside Tahto core
native/               frozen metadata migration evidence; not target runtime
```

## Authority boundary

```text
Greenways OS
  installation · consent · grants · credentials · private keys

Tahto HAL
  state meaning · authorization · orchestration · validation · recovery

Generic host capabilities
  canonical value persistence · byte custody · bounded streaming

Applications and specification packages
  domain fields · invariants · migrations · transforms · merge policy
```

A Hara value cannot select a native ABI, provider package, driver, database path,
storage root, credential, command or remote executable catalogue.

## Generic metadata persistence

Tahto metadata is persisted through the application-neutral service:

```text
service: hara.store
operations:
  load
  initialize
  compare-and-swap
  receipt
```

`tahto.store.provider` prepares and validates the exact generic requests and
results. The installed Hoplite SQLite provider sees opaque canonical values,
bounded revisions and opaque receipts. It does not parse Tahto object graphs,
transaction meaning, authorization or replay policy.

TAHTO-7 remains the deterministic HAL transaction boundary: it checks expected
revision, request replay evidence, one reviewed effect-free domain transition
and exact canonical-result verification before a generic provider CAS is
exposed.

The Rust code under `native/` is retained only as frozen migration evidence until
provider parity, semantic recovery and fault fixtures complete under #17.

See:

- [`protocol/transactions.md`](protocol/transactions.md)
- [`protocol/metadata-host.md`](protocol/metadata-host.md)
- [`protocol/metadata-store.md`](protocol/metadata-store.md)
- [`test/tahto/store/provider_test.hal`](test/tahto/store/provider_test.hal)

## Generic object custody

Large object bodies never become ordinary Hara values. Tahto emits closed domain
effects which `tahto.store.capability` maps to the installed generic service:

```text
service: hara.blob

upload/open            -> staging/open
upload/append          -> staging/append-from-source
upload/abort           -> staging/abort
upload/verify-install  -> staging/verify-commit
object/read-range      -> object/open-source
```

`tahto.store.upload` treats vault transitions as candidate state until the exact
generic result passes HAL validation. Provider failure or identity mismatch
rolls back optimistic offsets, removal and verification state.

The installed Hoplite providers supply restart-safe filesystem custody, actual
SHA-256 verification, request-scoped ingress sources and immutable egress
sources. Driver selection, storage roots and limits are trusted worker
configuration rather than portable request fields.

See:

- [`protocol/object-vault.md`](protocol/object-vault.md)
- [`protocol/host-capabilities.md`](protocol/host-capabilities.md)
- [`protocol/upload-integration.md`](protocol/upload-integration.md)
- [`test/tahto/store/upload_test.hal`](test/tahto/store/upload_test.hal)

## Authorized response sources

An authorized object range is projected as one closed portable value:

```clojure
{:protocol "hara.response-source/1"
 :source-handle 31
 :offset 2
 :length 7}
```

The full boundary is:

```text
Tahto namespace authority
  -> vault/plan-range
  -> hara.blob object/open-source
  -> exact result validation
  -> hara.response-source/1
  -> request-scoped Nginx transport
```

Hoplite binds source authority to the exact opaque request context, work and
handle. It streams with a reusable bounded buffer under output backpressure and
closes on success, error, timeout, disconnect, cancellation, `HEAD` or request
cleanup. A copied numeric handle is insufficient authority and source handles
never enter durable Tahto state.

See:

- [`protocol/response-sources.md`](protocol/response-sources.md)
- [`test/tahto/store/response_source_test.hal`](test/tahto/store/response_source_test.hal)

## Immutable history, synchronization and recovery

The history kernel owns:

```text
verified immutable commits
strict per-device sequence slots
signed compare-and-swap heads
divergent-head preservation
immutable backup closure pins
deterministic restore manifests
verified receipt evidence
```

The sync kernel owns device enrolment and revocation, durable nonce and request
idempotency evidence, exact missing-object offers, bounded push negotiation and
monotonic per-device collection cursors.

PR #38 established the first portable signed two-device object law: device A
publishes an immutable object through a signed application reference, commit,
head and receipt; device B receives the exact closure and opens the same digest
under its own verified work scope. Cross-work source use and post-revocation
requests fail closed. The production filesystem/container-restart proof remains
tracked by #36.

See:

- [`protocol/history.md`](protocol/history.md)
- [`protocol/sync.md`](protocol/sync.md)
- [`protocol/two-device-object-transfer.md`](protocol/two-device-object-transfer.md)
- [`test/tahto/store/history_test.hal`](test/tahto/store/history_test.hal)
- [`test/tahto/sync/two_device_object_test.hal`](test/tahto/sync/two_device_object_test.hal)

## Services and jobs

Service descriptors are inert, digest-pinned records. Worker implementation,
installation, scheduling and credentials remain outside Tahto core. Durable jobs
retain an application/namespace/collection coordinate, exact input/output roots,
idempotency and terminal state without admitting executable source into the
record.

See [`protocol/services.md`](protocol/services.md).

## Semantic Fabric release train

The semantic layer is additive over existing `object-graph/1` collections,
objects, commits and heads:

```text
exact specification package
  -> tahto.schema-ref/1
  -> semantic object + typed links
  -> stable-ID index + semantic root
  -> ordinary tahto.commit/1
  -> ordinary tahto.head/1
```

Tahto will validate generic envelopes and exact immutable references. The
installed specification package continues to own application fields, domain
invariants, migration and merge behavior. Runtime validation does not call the
public Specs website and a schema reference cannot install code.

Ordered work:

```text
#23  reconcile the current baseline
#30  freeze semantic value profiles
#31  admit verified semantic objects
#32  build stable indexes and roots
#33  bind roots to existing history
#34  add bounded canonical-value verification
#35  expose authenticated semantic operations
#17  prove recovery and remove transitional native code
```

## Current status

```text
control-plane discovery/status                 ready
pure-HAL object vault and history              ready
hara.store client and Hoplite SQLite provider  ready
hara.blob upload orchestration                 ready
filesystem object custody                      ready
response-source projection and transport       ready
portable signed two-device law                 ready
production restart/two-device fixture          pending #36
signature and canonical-record provider        not installed
semantic value profiles                        pending #30
semantic node service                          pending #35
transitional native metadata source            retained until #17
```

Operate the current control-plane node with:

```sh
bin/tahto check
bin/tahto run
```

Then inspect:

```sh
curl http://127.0.0.1:58100/.well-known/tahto
curl http://127.0.0.1:58100/tahto/v1/status
```

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and
[`protocol/tahto.md`](protocol/tahto.md) for the product boundary.
