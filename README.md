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

The first repository slice establishes the Hara/Hoplite node shell and canonical Tahto discovery surface:

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
src/tahto/store/      control/data-plane object-vault boundary
src-python/tahto/     operational node-local vault implementation
src/tahto/sync/       device and replication boundary (TAHTO-5/8)
src/tahto/backup/     immutable backup and restore boundary (TAHTO-4/9/10)
src/tahto/service/    inert service descriptors and durable jobs (TAHTO-6)
protocol/             normative protocol documents and schemas
conformance/          protocol and architecture checks
test-python/          executable object-vault conformance
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

## Operate the local object vault

The TAHTO-3 data-plane library uses only the Python standard library. Initialize and inspect a local vault with:

```sh
bin/tahto-vault --root ~/.greenways/tahto init
bin/tahto-vault --root ~/.greenways/tahto quota-set app.example profile.primary 1073741824
bin/tahto-vault --root ~/.greenways/tahto put app.example profile.primary ./archive.pack
bin/tahto-vault --root ~/.greenways/tahto gc
```

The object destination is always derived from a validated SHA-256 digest. The `put` path is a local source file and cannot select a destination inside the vault.

## Current implementation status

TAHTO-2 defines the stable application-neutral record envelopes and executable protocol fixtures in [`protocol/records.md`](protocol/records.md).

TAHTO-3 provides a streaming filesystem/SQLite content-addressed vault with resumable uploads, digest verification, atomic installation, existence negotiation, range reads, application/namespace quotas, bounded chunk manifests, closure verification, root pins, and dry-run-first garbage collection. Its normative profile is [`protocol/object-vault.md`](protocol/object-vault.md).

The Hara/Hoplite node still serves only discovery, health, and status. Its status record therefore reports the object-vault library as ready and the native HTTP data-plane binding as not yet wired. Atomic heads, immutable backups, device enrolment, incremental sync, workers, and durable jobs remain separate implementation PRs.

See [`LINEAGE.md`](LINEAGE.md) for the extracted Beacon history and [`protocol/tahto.md`](protocol/tahto.md) for the authority boundary.
