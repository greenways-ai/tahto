# Tahto response-source projection

## Status

This document defines the portable output of an authorized Tahto object range.
It is a HAL integration profile over the generic `hara.blob` capability, not a
Tahto wire record and not a Hoplite-specific response type.

## Boundary

The complete path is:

```text
Tahto namespace authority and object metadata
  -> vault/plan-range
  -> tahto.store.host-effect/1 object/read-range
  -> hara.blob object/open-source
  -> validated tahto.store.host-result/1 opened range
  -> hara.response-source/1
```

Tahto decides whether an application and namespace may read an immutable
object, checks that the requested half-open interval is inside the object, and
applies its configured range-size limit. The generic blob provider opens the
already-authorized immutable range and returns an opaque work-scoped source
handle.

`tahto.store.response-source` validates both sides of that operation and then
projects only:

```clojure
{:protocol "hara.response-source/1"
 :source-handle 31
 :offset 2
 :length 7}
```

The descriptor contains no digest, application, namespace, provider, driver,
path, credential, work identifier, HTTP field or Nginx state. It therefore
carries no Tahto semantics into Hoplite or another response host.

## Execution API

Pure and injected hosts use:

```clojure
(response-source/open-with
 state
 {:application "app"
  :namespace "space"
  :digest "sha256:..."
  :start 2
  :end 9}
 invoke)
```

Coroutine-enabled production hosts use:

```clojure
(coroutine/await
 (response-source/open state request))
```

Both paths run `vault/plan-range`, the same exact `hara.blob` request and result
validator, and the same descriptor projection. A successful result is a normal
Tahto `model/ok` whose value is the portable descriptor; the logical store state
is unchanged.

## Validation laws

- The range must first pass Tahto namespace authorization and object bounds.
- Exactly one `object/read-range` effect may be executed.
- The generic provider result must exactly match the requested digest, offset
  and length and may contain no additional fields.
- The translated Tahto opened-range result must pass the existing closed host
  result validator.
- The final value must pass Hara's `hara.response-source/1` validator, including
  positive source handle and length, non-negative offset, safe-integer bounds,
  and a bounded half-open interval.
- A positive native handle above the portable safe-integer maximum is rejected.
- Provider rejection and request/result mismatch remain capability errors; a
  value that is valid only at the native capability layer but not portable is a
  `tahto.response-source/result-invalid` failure.
- Object bytes never cross the HAL event or result value.

## Host lifecycle

The descriptor is response-lifetime state. It must not be stored in Tahto
objects, commits, snapshots or receipts. The response host resolves it using
its own request-scoped authority, streams under its own backpressure rules, and
closes the source before releasing the owning work scope.
