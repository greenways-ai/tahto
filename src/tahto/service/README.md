# Service and durable-job boundary

TAHTO-6 implements inert application service descriptors and durable job state in Hara.

A service descriptor records only reviewed metadata:

```text
application and service identity
service protocol and worker version
immutable package or binary digest
allowed collections and operations
bounded memory, CPU and output policy
registered or disabled status
```

The descriptor is not an installer and cannot contain JavaScript, HTML, HAL, arbitrary Wasm, a native command or another executable payload. Greenways OS owns installation, approval, grants and credentials. Worker implementations remain in their application repositories and are pinned by immutable artifact digest.

`tahto.service.state` provides deterministic transitions for:

```text
register-service
disable-service
enqueue-job
transition-job
```

Jobs retain an internal application/namespace/collection coordinate while the stable `tahto.job/1` envelope remains application-neutral. Inputs and completed outputs must already be complete, namespace-authorized Tahto object closures. Enqueue requests are idempotent, attempts are monotonic, and completed or cancelled jobs are terminal.

TAHTO-6 does not execute workers or provide persistence. An installed durable metadata provider must atomically commit the returned state transition together with the request nonce/idempotency evidence produced by TAHTO-5.
