# Tahto

**Tahto is the Greenways application-state and semantic fabric.** It stores,
synchronizes, backs up, restores and routes application state while Greenways OS
retains installation, consent, private-key, credential and grant authority.

```text
Greenways OS applications
          │ exact grants and verified requests
          ▼
       Tahto HAL
  semantic values · objects
  commits · heads · sync
  backup · restore
          │ closed generic capability calls
          ▼
 hara.value · hara.store · hara.blob
          │ trusted host installation
          ▼
 specification packages · SQLite · filesystem · Nginx
```

Applications and exact installed specification packages own the meaning of their
values. Tahto owns application-neutral identity, custody, graph closure,
immutable history, divergent heads and recovery. It does not invent a universal
merge policy.

The build is now **late kernel / early production integration**. The semantic
model, history and pure read/prepare/submit kernels are implemented. The
remaining work is installed specification validation, semantic service routing
and authentication, compiler-free production boot, and the complete signed
two-device restart/recovery proof. See
[#29](https://github.com/greenways-ai/tahto/issues/29).

## Node surface

The current Hara/Hoplite control-plane application exposes:

```text
GET  /.well-known/tahto
GET  /tahto/v1/health
GET  /tahto/v1/status
POST /tahto/v1/pairing/prepare
POST /tahto/v1/pairing/complete
```

The loopback operator command `bin/tahto invite` issues a short-lived
`invite.*~token` value through the non-advertised management route. Only the
token digest enters durable metadata. Greenways OS sends the raw token to
prepare, signs the exact returned intent, and completes identity-only enrolment
without gaining administrator or application authority.

One bounded compatibility release also exposes:

```text
GET /.well-known/greenways-beacon
GET /beacon/v1/health
GET /beacon/v1/status
```

These are discovery and status aliases. The pure semantic operation kernels are
present, but `semantic/read`, `semantic/prepare` and `semantic/submit` are not yet
advertised as installed HTTP operations. Their remaining service work is tracked
by [#65](https://github.com/greenways-ai/tahto/issues/65),
[#66](https://github.com/greenways-ai/tahto/issues/66) and
[#67](https://github.com/greenways-ai/tahto/issues/67).

## Readiness matrix

| Boundary | Status |
| --- | --- |
| Semantic value profiles | ready |
| Semantic object admission | ready |
| Stable semantic indexes and roots | ready |
| Semantic history over existing commits and heads | ready |
| `semantic.read` pure-HAL kernel | ready |
| `semantic.prepare` pure-HAL kernel | ready |
| `semantic.submit` pure-HAL kernel | ready |
| Pairing and canonical signed-request law | ready |
| Greenways OS pairing client | ready |
| `hara.value` contract and Tahto adapter | ready |
| Filesystem `hara.value` provider | pending `greenways-ai/hoplite#80` |
| Installed `hara.value` registration | pending `greenways-ai/hoplite#82` |
| Exact installed specification-validator invocation | pending `#34` follow-up |
| Semantic routes | pending `#65` |
| Required semantic authentication realm | pending `#66` |
| Selected-value response-source integration | pending `#67` |
| Module-aware compiler-free worker boot | pending `greenways-ai/hoplite#22` |
| Full signed two-device restart/recovery fixture | pending `#36` / `#47` |
| Final provider parity and removal of `native/` | pending `#17` |

Kernel availability is deliberately reported separately from installed service
availability. A pure operation law on `main` is not evidence that a production
route, provider or authentication realm is installed.

## Repository layout

```text
src/tahto/node/       Hoplite control-plane application
src/tahto/protocol/   application-neutral records and verification contracts
src/tahto/semantic/   semantic model, admission, indexes, roots and operations
src/tahto/store/      objects, history and generic capability orchestration
src/tahto/sync/       device, pairing and replication laws
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
  application-neutral state custody · authorization · orchestration
  semantic identity · history · divergence · validation evidence · recovery

Generic host capabilities
  canonical value verification · metadata persistence · byte custody
  bounded request and response sources

Applications and exact installed specification packages
  domain fields · invariants · migrations · transforms · merge policy
```

A Hara value cannot select a native ABI, provider package, driver, database path,
storage root, credential, command or remote executable catalogue. A schema
reference pins an exact installed package root and exported validator entry; it
is not authority to install, fetch or execute remote code.

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
provider parity, exact semantic recovery and fault fixtures complete under
[#17](https://github.com/greenways-ai/tahto/issues/17).

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

The remaining semantic-selected-value adapter reuses this path rather than
creating a second response-body protocol. See
[#67](https://github.com/greenways-ai/tahto/issues/67).

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

Semantic roots enter this unchanged history model:

```text
tahto.semantic-root/1
  -> tahto.commit/1
  -> tahto.head/1
  -> divergence
  -> application-authored merge
  -> metadata CAS
```

The sync kernel owns device enrolment and revocation, durable nonce and request
idempotency evidence, exact missing-object offers, bounded push negotiation and
monotonic per-device collection cursors.

PR #38 established the portable signed two-device object law. The production
filesystem/container-restart composition remains tracked by
[#36](https://github.com/greenways-ai/tahto/issues/36) and
[#47](https://github.com/greenways-ai/tahto/issues/47).

See:

- [`protocol/history.md`](protocol/history.md)
- [`protocol/semantic-history.md`](protocol/semantic-history.md)
- [`protocol/sync.md`](protocol/sync.md)
- [`protocol/two-device-object-transfer.md`](protocol/two-device-object-transfer.md)
- [`test/tahto/semantic/history_test.hal`](test/tahto/semantic/history_test.hal)
- [`test/tahto/sync/two_device_object_test.hal`](test/tahto/sync/two_device_object_test.hal)

## Semantic Fabric

The implemented semantic layer is additive over existing `object-graph/1`
collections, objects, commits and heads:

```text
exact installed specification package
  -> tahto.schema-ref/1
  -> canonical application value root
  -> semantic object + typed links
  -> stable-ID index + complete semantic root
  -> ordinary tahto.commit/1
  -> ordinary tahto.head/1
```

Tahto can already prove that an admitted value root, schema reference, semantic
object, index, root, commit and head are exact and internally consistent. The
next value boundary must also prove that the exact locally installed
specification package accepted the decoded canonical application value.

The remaining value/specification sequence is:

```text
greenways-ai/hoplite#80  filesystem hara.value provider
greenways-ai/hoplite#82  installed provider registration
#34 follow-up             exact package-root and validator-entry invocation
```

Runtime validation never calls the public Specs website and a schema reference
cannot install code.

See:

- [`protocol/semantic-values.md`](protocol/semantic-values.md)
- [`protocol/semantic-admission.md`](protocol/semantic-admission.md)
- [`protocol/semantic-index-roots.md`](protocol/semantic-index-roots.md)
- [`protocol/semantic-history.md`](protocol/semantic-history.md)
- [`protocol/canonical-values.md`](protocol/canonical-values.md)

## Semantic operation kernels

The pure-HAL operation laws are complete:

- `semantic.read` preserves every selected divergent branch and performs bounded
  stable-ID lookup independently in each branch;
- `semantic.prepare` constructs deterministic closed signing intents from fully
  explicit device, sequence, timestamp, parent, head and revision evidence;
- `semantic.submit` replays preparation against current state, binds verified
  signed commit/head records exactly, and publishes through one TAHTO-7
  transition and one generic `hara.store` CAS.

These kernels do not imply installed routes. The service completion sequence is:

```text
#65  route ↔ signed-operation mapping
#66  mandatory semantic authentication realm and service registration
#67  selected value ↔ existing response-source path
```

See:

- [`protocol/semantic-read.md`](protocol/semantic-read.md)
- [`protocol/semantic-prepare.md`](protocol/semantic-prepare.md)
- [`protocol/semantic-submit.md`](protocol/semantic-submit.md)
- [`test/tahto/semantic/read_test.hal`](test/tahto/semantic/read_test.hal)
- [`test/tahto/semantic/prepare_test.hal`](test/tahto/semantic/prepare_test.hal)
- [`test/tahto/semantic/submit_test.hal`](test/tahto/semantic/submit_test.hal)

## Services and jobs

Service descriptors are inert, digest-pinned records. Worker implementation,
installation, scheduling and credentials remain outside Tahto core. Durable jobs
retain an application/namespace/collection coordinate, exact input/output roots,
idempotency and terminal state without admitting executable source into the
record.

See [`protocol/services.md`](protocol/services.md).

## Remaining release train

```text
1. greenways-ai/hoplite#80 -> #82 -> Tahto #34 follow-up
   exact stored bytes -> canonical value -> exact installed validator acceptance

2. Tahto #65 -> #66 -> #67
   route mapping -> required auth realm -> selected-value source

3. greenways-ai/hoplite#22 -> Tahto #58 -> #36/#47
   compiler-free boot -> production pairing -> signed A/restart/B proof

4. Tahto #19 -> #17
   pure-HAL manifest interpretation -> recovery/parity -> delete native/
```

Only after those gates should the release train broaden into Hodos or Alumbra
application integration.

## Operate the current node

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
