# Tahto

**Tahto** is the Greenways application-state fabric. It stores, synchronizes,
backs up, restores and routes application state while Greenways OS retains
consent, installation, private-key, credential and grant authority.

```text
Greenways OS applications
          │ exact OS grants and signed record verification
          ▼
       Tahto node
  objects · commits · heads
  backup · restore · services
          │
          ├─ local home node
          ├─ trusted compute node
          ├─ sealed replica
          └─ optional hosted relay
```

Applications retain semantic ownership. Historia defines an archive, Hestia
defines a room or mandate, Worlds defines a scene, and Ignatius validates
canonical execution. Tahto preserves signed records and divergent heads without
inventing generic merge semantics.

## Node surface

The Hara/Hoplite node exposes:

```text
GET /.well-known/tahto
GET /tahto/v1/health
GET /tahto/v1/status
```

One bounded compatibility release also exposes:

```text
GET /.well-known/greenways-beacon
GET /beacon/v1/health
GET /beacon/v1/status
```

Compatibility routes describe Tahto; they do not restore Beacon's former
assumption that Greenways Space is the service authority. Hosted Space remains
an optional adapter under `adapters/greenways-space/`.

## Repository layout

```text
src/tahto/node/       Hoplite control-plane application
src/tahto/protocol/   application-neutral records and verification contracts
src/tahto/store/      objects, history and atomic metadata provider contracts
src/tahto/sync/       device and replication boundary
src/tahto/backup/     transfer and restore executors (future provider work)
src/tahto/service/    inert service descriptors and durable jobs
native/               closed native ABIs and installed provider implementations
protocol/             normative protocol documents and schemas
test/                 executable Hara conformance suites
conformance/          dependency-free protocol and architecture guards
adapters/             optional integrations outside Tahto core
```

## Operate the node

With a compatible Hoplite executable installed:

```sh
bin/tahto check
bin/tahto run
```

Then inspect:

```sh
curl http://127.0.0.1:58100/.well-known/tahto
curl http://127.0.0.1:58100/tahto/v1/status
```

`bin/greenways-beacon` is a warning compatibility wrapper that invokes `tahto`.

## Hara object and history kernels

TAHTO-3 and TAHTO-4 are deterministic Hara state machines under
`src/tahto/store/`.

The object kernel owns:

```text
canonical digest identity
resumable upload transitions
logical namespace quotas
verified installation acceptance
bounded range plans
verified ordered chunk manifests
object closure and GC roots
```

The history kernel owns:

```text
verified immutable commits
strict device sequence slots
signed compare-and-swap heads
divergent-head preservation
immutable backup closure pins
deterministic restore manifests
verified receipt evidence
```

Large object bodies never become ordinary Hara values. The kernel emits a closed
set of native effects using opaque body handles. Hoplite's merged bounded
streaming ABI and Nginx request-body binding define the transport contract,
while object-transfer execution and native response-source streaming remain
explicit follow-up work.

Commit, head, backup and receipt transitions consume
`tahto.record-verification/1` proofs. The installed key provider must bind the
exact canonical bytes, digest, key identity and signature. The Hara kernel
validates and uses that proof; it does not emulate cryptography.

See:

- [`protocol/object-vault.md`](protocol/object-vault.md)
- [`protocol/history.md`](protocol/history.md)
- [`test/tahto/store/vault_test.hal`](test/tahto/store/vault_test.hal)
- [`test/tahto/store/history_test.hal`](test/tahto/store/history_test.hal)

## Device, synchronization and durable-job kernels

TAHTO-5 adds device enrolment and revocation identity, durable nonce and request
idempotency evidence, namespace-scoped pull offers, bounded push negotiation and
monotonic per-device collection cursors.

TAHTO-6 adds inert service registrations and durable job transitions. Service
records pin immutable package or binary digests and contain no executable code.
Jobs retain an internal application/namespace/collection coordinate, require
complete authorized input and output closures, use exact enqueue idempotency,
advance attempts only when work is claimed and preserve terminal states.

Worker implementation and scheduling remain in application repositories and
installed providers. Greenways OS retains worker installation, approval,
credentials, grants and private keys.

See:

- [`protocol/sync.md`](protocol/sync.md)
- [`protocol/services.md`](protocol/services.md)
- [`test/tahto/sync/device_test.hal`](test/tahto/sync/device_test.hal)
- [`test/tahto/sync/session_test.hal`](test/tahto/sync/session_test.hal)
- [`test/tahto/service/state_test.hal`](test/tahto/service/state_test.hal)

## Atomic metadata transaction plans

TAHTO-7 composes one verified request into one deterministic metadata commit
plan. It checks the expected state revision, records nonce and idempotency
evidence, executes one reviewed effect-free domain transition, validates an
installed-provider result digest, completes the request and advances the
metadata revision.

A completed idempotent replay records its fresh nonce and returns the prior
result digest without re-running domain code. Rejected domain operations are
also completed idempotently, but any partial state returned by the rejected
transition is discarded.

This is the semantic boundary required by a durable provider; it is not itself
a database. The installed provider must compare-and-swap the expected revision
and returned state in one durable transaction. Signature, freshness and
canonical-result verification remain provider responsibilities.

See:

- [`protocol/transactions.md`](protocol/transactions.md)
- [`test/tahto/store/transaction_test.hal`](test/tahto/store/transaction_test.hal)

## Metadata provider and host contracts

The dependency-free native `tahto-metadata-store/1` ABI and the bundled SQLite
provider define durable snapshot, exact revision compare-and-swap and commit
receipt behavior. The SQLite implementation recomputes state SHA-256 identity,
uses an immediate transaction, rejects stale writers and records the snapshot
and receipt atomically.

TAHTO-8 adds the Hara-side bridge. It accepts only canonical state and commit
verification proofs, validates them against the exact TAHTO-7 state and plan,
and exposes one fixed installed host service:

```text
tahto.metadata
```

The closed methods are `load`, `initialize`, `compare-and-swap` and `receipt`.
Requests cannot select a database path, provider package, credential, upstream,
callback or native command. Provider snapshots and receipts are revalidated as
closed records before their state or evidence is accepted.

This contract does not claim that Hoplite currently registers the service or
encodes Hara state into canonical HTA bytes. That runtime bridge is the next
integration slice.

See:

- [`protocol/metadata-store.md`](protocol/metadata-store.md)
- [`protocol/metadata-host.md`](protocol/metadata-host.md)
- [`test/tahto/store/provider_test.hal`](test/tahto/store/provider_test.hal)

## Current implementation status

TAHTO-2 defines the stable application-neutral record envelopes. TAHTO-3
provides the Hara object-vault kernel, TAHTO-4 provides immutable commits,
atomic signed heads, backups and restore planning, TAHTO-5 provides device,
replay and incremental-sync planning, TAHTO-6 provides inert service and
durable-job state, TAHTO-7 provides atomic metadata transaction plans, and
TAHTO-8 defines the fixed Hara-to-native metadata host contract.

The native metadata ABI and SQLite provider are implemented. The Hoplite host
registry and canonical HTA state bridge, canonical signature/freshness
provider, nonce/idempotency retention compaction, native object-transfer and
response-streaming executors, worker executor, pairing UX and complete
two-device conformance scenario are not represented as complete. The status
document reports these boundaries directly instead of emulating them with
whole-file Python or in-memory production stores.

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and
[`protocol/tahto.md`](protocol/tahto.md) for the authority boundary.
