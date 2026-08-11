---
title: Operate the node
description: Build, start, discover, and inspect the current Tahto control plane.
---
# Operate the node

The current project is a Hara/Hoplite control-plane application with its server profile defined in `project.edn`.

## Surface

The node exposes discovery, health, status, diagnostics, and pairing operations. Health and status are read-only and never initialize storage as a side effect.

```text
GET  /.well-known/tahto
GET  /tahto/v1/health
GET  /tahto/v1/status
POST /tahto/v1/diagnostics
POST /tahto/v1/pairing/prepare
POST /tahto/v1/pairing/complete
```

Use `bin/tahto invite` from the loopback operator boundary to create a short-lived pairing invitation. Never log or persist the raw invitation token.

## Verify the checkout

Run the repository's focused Hara test suites for the boundary you are changing. The source README and `test/` tree remain authoritative for exact development commands while the production boot path is still being completed.
