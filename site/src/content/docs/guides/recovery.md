---
title: Recovery and diagnostics
description: Inspect readiness and preserve exact recovery evidence.
---
# Recovery and diagnostics

Health and status perform a read-only provider load. They distinguish `ready`, `uninitialized`, and `unavailable` rather than claiming health from configuration alone.

Signed diagnostics return only the metadata revision and aggregate counts. They never expose records, keys, credentials, or application values.

Recovery begins from an immutable backup closure pin and a deterministic restore manifest. Operational queues, leases, cursors, and caches are rebuildable and are not canonical application data.
