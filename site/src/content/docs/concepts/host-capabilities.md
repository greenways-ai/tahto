---
title: Host capabilities
description: Generic storage, blob custody, and request-scoped source handles.
---
# Host capabilities

Tahto interprets closed, application-neutral capability calls. Metadata uses `hoplite.store`; large immutable bodies use `hoplite.blob`.

Request and response source handles are ephemeral host resources bound to an exact request context. Handles never enter objects, commits, heads, receipts, backups, roots, or metadata snapshots.
