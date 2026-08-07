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
src/tahto/store/      Hara object graph, commits, heads and backup transitions
src/tahto/sync/       device and replication boundary (TAHTO-5/8)
src/tahto/backup/     transfer and restore executors (TAHTO-9/10)
src/tahto/service/    inert service descriptors and durable jobs (TAHTO-6)
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
streaming ABI defines the transport contract, while runtime/Nginx wiring and
durable metadata persistence remain explicit follow-up work.

Commit, head, backup and receipt transitions consume
`tahto.record-verification/1` proofs. The installed key provider must bind the
exact canonical bytes, digest, key identity and signature. TAHTO-4 validates and
uses that proof; key enrolment, cryptographic verification and revocation remain
TAHTO-5.

See:

- [`protocol/object-vault.md`](protocol/object-vault.md)
- [`protocol/history.md`](protocol/history.md)
- [`test/tahto/store/vault_test.hal`](test/tahto/store/vault_test.hal)
- [`test/tahto/store/history_test.hal`](test/tahto/store/history_test.hal)

## Current implementation status

TAHTO-2 defines the stable application-neutral record envelopes. TAHTO-3
provides the Hara object-vault kernel, and TAHTO-4 provides immutable commits,
atomic signed heads, backups and restore planning.

The native byte data plane, durable metadata transaction provider, signed-device
verifier, enrolment and incremental replication are not represented as complete.
The status document reports these boundaries directly instead of emulating them
with whole-file Python or in-memory production stores.

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and
[`protocol/tahto.md`](protocol/tahto.md) for the authority boundary.
