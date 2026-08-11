# Tahto services and durable jobs, version 1

TAHTO-6 adds deterministic state transitions around the stable `tahto.service/0-alpha` and `tahto.job/0-alpha` records defined by the Tahto core schema.

## Authority boundary

A Tahto service is an inert registration for an application-owned worker. Greenways OS owns installation, consent, grants, private keys and credentials. The application repository owns the worker implementation and the meaning of its inputs and outputs. Tahto owns only custody of the descriptor, durable job progress and object-root references.

A service record never installs or transports executable code. Unknown fields are rejected, so a descriptor cannot add a command, URL-selected factory, JavaScript, HTML, HAL source, arbitrary Wasm or a native executable path.

## Service identity

Service identity is scoped by application and service id. A registration contains:

```text
application
service id and protocol
worker version
immutable artifact digest
allowed collections
allowed operation classes
bounded resource policy
registered or disabled status
```

Registering the exact same descriptor is idempotent. Reusing the same identity with a different artifact or policy is a conflict. A reviewed worker upgrade therefore uses a new service identity/version and disables the previous registration; it is never an in-place executable substitution.

Disabling a service blocks new jobs and retries. Already-running work may still settle to `blocked`, `completed`, `failed` or `cancelled`, allowing shutdown without making durable work permanently unaccountable.

## Job coordinates

The stable job envelope intentionally does not contain a namespace or collection. TAHTO-6 stores an internal coordinate beside each job:

```text
application
namespace
collection
service
creating device
```

Every transition must use a pending, operation-exact TAHTO-5 request context for the same coordinate. This prevents a job id from becoming ambient authority across applications, namespaces or collections.

## Enqueue idempotency

A queued job starts with:

```text
state: queued
attempt: 0
output roots: empty
error: absent
```

Its `idempotencyKey` must match the signed request context. Tahto binds that key to the exact job record, internal coordinate and request digest. A retry of the same signed operation returns the existing job. Any different record or coordinate is rejected.

Input roots must be distinct canonical digests, within configured bounds, present in the authorized namespace and complete under the Tahto object graph.

## State machine

The closed transition graph is:

```text
queued   -> running | cancelled
running  -> blocked | completed | failed | cancelled
blocked  -> queued | cancelled
failed   -> queued | cancelled
completed -> terminal
cancelled -> terminal
```

A `queued -> running` claim increments `attempt` by exactly one. Every other transition preserves the current attempt. Retry returns a failed or blocked job to `queued`; the next claim increments again. Configured attempt limits fail closed.

`blocked` and `failed` require a closed error record. Other states reject an error field. Only `completed` may publish output roots. Those roots must also be complete and namespace-authorized.

The following fields are immutable after enqueue:

```text
protocol
job id
application
service
input roots
enqueue idempotency key
creation timestamp
```

Completed and cancelled jobs cannot be rewritten.

## Durability requirement

The Hara functions return a proposed next state. Production acceptance requires one installed durable metadata provider transaction that atomically commits:

```text
TAHTO-5 nonce evidence
TAHTO-5 request idempotency state
TAHTO-6 service or job transition
any resulting receipt record
```

No in-memory map is represented as production durability. Worker execution and scheduling are separate provider concerns.

## Conformance

The executable Hara suite proves:

- descriptors are closed and inert;
- service artifact identity is immutable;
- wrong-operation contexts are rejected;
- disabled services cannot accept new jobs;
- enqueue replay is exact and conflicts fail closed;
- inputs and outputs cannot cross namespaces;
- attempts advance only on claims;
- invalid transitions and terminal rewrites are rejected;
- in-flight work may settle after disablement; and
- configured service and job bounds are enforced.
