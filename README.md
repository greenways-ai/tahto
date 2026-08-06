# Tahto

**Tahto** is the Greenways application-state fabric. It stores, synchronizes, backs up, restores, and routes application state while Greenways OS retains consent, installation, private-key, credential, and grant authority.

```text
Greenways OS applications
          │ exact OS grants
          ▼
       Tahto node
  objects · commits · heads
  sync · backup · services
          │
          ├─ local home node
          ├─ trusted compute node
          ├─ sealed replica
          └─ optional hosted relay
```

Applications retain semantic ownership. Historia defines an archive, Hestia defines a room or mandate, Worlds defines a scene, and Ignatius validates canonical execution. Tahto preserves their signed records and divergent heads without inventing generic merge semantics.

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

The compatibility routes describe Tahto; they do not preserve Beacon's former assumption that Greenways Space is the service authority. The optional hosted Space relationship lives under `adapters/greenways-space/` and is not part of core node startup.

## Repository layout

```text
src/tahto/node/       Hoplite control-plane application
src/tahto/protocol/   application-neutral descriptors and wire contracts
src/tahto/store/      Hara object-vault state machine and host boundary
src/tahto/sync/       device and replication boundary (TAHTO-5/8)
src/tahto/backup/     immutable backup and restore boundary (TAHTO-4/9/10)
src/tahto/service/    inert service descriptors and durable jobs (TAHTO-6)
test/                  Hara executable conformance
protocol/              normative protocol documents and schemas
conformance/           repository, route and JSON-Schema guards
adapters/              optional integrations outside Tahto core
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

## Hara object-vault kernel

TAHTO-3 is implemented in Hara under `tahto.store.*`.

Hara owns:

```text
canonical object identity
resumable-upload transitions
logical namespace quotas
verified-install acceptance
namespace references
bounded range plans
ordered chunk manifests
closure verification
root pins
garbage-collection plans
```

Large object bytes are not materialized as ordinary Hara values. Hara emits a closed set of `tahto.store.host-effect/1` operations using opaque request-body handles. A trusted native/Hoplite adapter will provide bounded streaming, SHA-256 verification, seekable reads, fsync and atomic installation under HOPLITE-1.

See [`protocol/object-vault.md`](protocol/object-vault.md) and `test/tahto/store/vault_test.hal`.

## Current implementation status

TAHTO-2 defines the stable application-neutral record envelopes in [`protocol/records.md`](protocol/records.md).

The object-vault state machine is ready in Hara. The HTTP/native byte data plane is deliberately reported as not wired rather than emulated in Python or by moving whole blobs through Hara collections. Atomic heads, immutable backups, device enrolment, incremental sync, workers and durable jobs remain separate implementation PRs.

The Python programs under `conformance/` are static architecture and schema guards only; they are not node runtime components.

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and [`protocol/tahto.md`](protocol/tahto.md) for the authority boundary.
