# Tahto node protocol bootstrap

## Product boundary

Tahto is a user-controlled application-state fabric. Greenways OS remains the product, browser kernel, suite host, consent authority, and private-key authority. Applications retain the meaning of their data.

```text
Greenways OS ── exact application grant ──▶ Tahto
     │                                      │
     │ owns consent and keys                │ owns custody and movement
     ▼                                      ▼
application semantics                 objects, commits, heads,
Historia · Hestia · Worlds            sync, backup, services
```

## Canonical routes

```text
GET /.well-known/tahto
GET /tahto/v1/health
GET /tahto/v1/status
```

The bootstrap descriptor is public local metadata and contains no private key, bearer credential, browser history, application payload, or administrator grant.

## Beacon compatibility

For one migration release:

```text
GET /.well-known/greenways-beacon
GET /beacon/v1/health
GET /beacon/v1/status
```

The discovery alias explicitly points to Tahto. The health and status aliases return Tahto state. No `/space/` proxy exists in core.

## Optional Greenways Space adapter

A hosted Greenways Space node may later act as relay, rendezvous service, sealed backup destination, public world delivery node, or application-worker host. That relationship is implemented as an explicit adapter and is not required for local application launch or storage.

## Security laws

1. Greenways OS owns installation, consent, private keys, provider credentials, and grants.
2. Tahto owns application-neutral custody and movement.
3. Applications own payload meaning and reconciliation.
4. Divergent valid heads are preserved.
5. Source and rebuildable derived state remain distinct.
6. Service descriptors are inert and cannot install executable content into Greenways OS.
7. Device pairing does not grant node administration.
8. No request may select a storage path or remote upstream.
9. Hosted Greenways Space is optional.
10. Tahto core depends on no application repository.

## Deferred protocol records

TAHTO-2 defines versioned application-neutral records for node, device, application, namespace, collection, object, commit, head, backup, receipt, service, and job. This bootstrap PR does not claim those schemas are stable or implemented.
