# Tahto node and fabric boundary

## Product boundary

Tahto is a user-controlled application-state and semantic fabric. Greenways OS
remains the product, browser kernel, suite host, consent authority and
private-key authority. Applications and exact installed specification packages
retain the meaning of their data.

```text
Greenways OS ── exact application grant ──▶ Tahto
     │                                      │
     │ owns consent and keys                │ owns identity, custody,
     ▼                                      │ graph closure and history
application/specification meaning           ▼
Historia · Hestia · Hodos · Alumbra     objects · commits · heads
                                        sync · backup · restore
```

Tahto core contains no dependency on those application repositories.

## Runtime boundary

Tahto domain behavior is Hara code. Generic installed capabilities own operating
system mechanics:

```text
Tahto HAL
  authorization · state transitions · validation · recovery
        |
        v
hara.store · hara.blob
        |
        v
trusted Hoplite providers · SQLite · filesystem · Nginx
```

A portable value cannot select a native ABI, driver, path, provider package,
credential, command or remote executable catalogue.

## Canonical routes

The current control-plane application exposes:

```text
GET /.well-known/tahto
GET /tahto/v1/health
GET /tahto/v1/status
```

The descriptor is public local metadata and contains no private key, bearer
credential, application payload or administrator grant. Authenticated semantic
operations remain a later additive service under #35 and are not advertised by
the current node.

## Beacon compatibility

For one migration release:

```text
GET /.well-known/greenways-beacon
GET /beacon/v1/health
GET /beacon/v1/status
```

The discovery alias points to Tahto. No `/space/` proxy exists in core and
hosted Greenways Space remains optional.

## Implemented fabric records

TAHTO-2 defines closed application-neutral records for:

```text
node · device · application · namespace · collection · object
commit · head · backup · receipt · service · job
```

Application payloads stay in referenced immutable objects. Core records never
grow Hodos-, Alumbra-, Historia-, Hestia- or Ignatius-specific fields.

Existing `object-graph/1` collections and application-owned `schema`/
`schemaVersion` fields provide the envelope for the Semantic Fabric work. No new
core record generation or collection mode is required for the first release.

## Object and metadata capabilities

Metadata uses generic `hara.store` for opaque canonical values, exact revision
CAS and atomic opaque receipts. Large bytes use generic `hara.blob` for staged
upload, digest-verified installation and immutable source opening.

Authorized ranges project only `hara.response-source/1`. Host ownership is exact
request context + work + handle; a copied numeric handle is insufficient
authority and never enters durable Tahto state.

The portable signed two-device law is implemented. The filesystem/container-
restart production proof remains #36.

## Semantic Fabric direction

The next profiles add:

```text
exact immutable schema reference
stable logical semantic identity
typed content-addressed link
bounded stable-ID index
complete semantic collection root
```

A semantic root is referenced by an ordinary `tahto.commit/1` and selected by an
ordinary `tahto.head/1`. Tahto preserves divergent valid roots. An application-
owned merge package may later produce a normal merge commit; Tahto does not
invent or execute a universal merge.

A schema reference pins an exact installed package root and exported validator
entry. It is evidence, not authority to install code. Runtime validation never
calls the public Specs website.

## Security laws

1. Greenways OS owns installation, consent, private keys, provider credentials
   and grants.
2. Tahto owns application-neutral identity, custody, movement, closure, history
   and recovery.
3. Applications and specification packages own payload meaning and
   reconciliation.
4. Divergent valid heads are preserved.
5. Source and rebuildable derived state remain distinct.
6. Service descriptors are inert and cannot install executable content.
7. Device pairing does not grant node administration.
8. No request may select a storage path, driver or remote upstream.
9. Source handles are ephemeral request/work resources and never durable state.
10. Hosted Greenways Space is optional.
11. Tahto core depends on no application repository.
12. Dense engine state is not expanded into one semantic object per voxel,
    sample or byte.

## Ordered work

```text
#23  baseline documentation and status
#30  semantic value profiles
#31  semantic object admission
#32  stable indexes and roots
#33  existing commit/head integration
#34  bounded canonical-value verification
#35  authenticated semantic operations
#36  production two-device transfer proof
#17  recovery proof and transitional native cleanup
```
