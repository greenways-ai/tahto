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

## Bootstrap boundary

This first repository slice establishes the Hara/Hoplite node shell and canonical Tahto discovery surface:

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
src/tahto/store/      object-vault boundary (TAHTO-3)
src/tahto/sync/       device and replication boundary (TAHTO-5/8)
src/tahto/backup/     immutable backup and restore boundary (TAHTO-4/9/10)
src/tahto/service/    inert service descriptors and durable jobs (TAHTO-6)
protocol/             normative protocol documents
conformance/          executable fixtures and architecture checks
adapters/             optional integrations outside Tahto core
```

## Operate the bootstrap node

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

## Current implementation status

The bootstrap serves only discovery, health, and status. Content-addressed objects, signed commits, atomic heads, backups, device enrolment, incremental sync, workers, and durable jobs arrive in separate reviewable PRs. The status document reports those components as deferred rather than claiming they are active.

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and [`protocol/tahto.md`](protocol/tahto.md) for the authority boundary.
