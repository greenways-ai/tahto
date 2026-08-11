# Tahto atomic metadata transactions, version 1

TAHTO-7 defines the deterministic boundary between Tahto's pure Hara state kernels and an installed durable metadata provider.

It does **not** make an in-memory Hara map durable. It produces one proposed next state and the evidence a provider must compare-and-swap in a single storage transaction.

## Why this boundary exists

TAHTO-5 records verified nonces and request idempotency. TAHTO-3 through TAHTO-6 perform object, history, synchronization, service and job state transitions. Persisting those steps independently would permit crash states such as:

```text
nonce recorded, domain mutation absent
mutation committed, completion digest absent
completion recorded, partial domain state visible
```

The atomic coordinator composes those logical steps before any provider commit.

## Protocols

```text
tahto.metadata-transaction/0-alpha
tahto.transaction-result-verification/0-alpha
```

`tahto.metadata-transaction/0-alpha` is the returned commit plan. It records:

```text
expected revision
next revision
request digest
result digest
completed request context
canonical result or error value
whether the request was replayed
```

`tahto.transaction-result-verification/0-alpha` is produced by an installed canonical-result provider. It binds:

```text
operation
request digest
ok or error outcome
canonical result digest
completion timestamp
```

The Hara coordinator validates the closed proof and its coordinate bindings. It does not hash arbitrary values or emulate canonical serialization.

## Execution law

For a new request, the provider invokes the coordinator against one snapshot:

```text
execute
  snapshot state
  expected metadata revision
  verified device request
  reviewed domain transition
  transition input
  verified canonical result identity
```

The coordinator then:

1. rejects a stale expected revision before recording a nonce;
2. runs TAHTO-5 authorization and idempotency admission;
3. invokes exactly one trusted domain transition;
4. rejects any transition that still returns native effects;
5. validates the result-verification proof against the request and actual ok/error outcome;
6. completes the idempotency record with the verified result digest;
7. increments `:metadata-revision`; and
8. returns the proposed next state.

The durable provider must atomically compare the stored revision with `expected-revision` and replace the snapshot with the returned state. A revision conflict restarts from a new snapshot; it must not partially apply the prior plan.

## Successful and rejected outcomes

A successful domain transition contributes its returned state to the commit.

A rejected domain transition contributes **no domain mutation**, even if a malformed implementation returned a changed state in its error result. The authorization evidence and canonical error digest are still completed idempotently, allowing retries to retrieve the same error without executing the operation again.

This distinction prevents both partial mutation and endless retry of a permanently rejected signed request.

## Native effects

The coordinator accepts only transitions whose effect vector is empty.

Native object transfer, signature verification and other external effects must finish before the metadata transaction begins. Their installed providers return closed verified result records that a domain transition can validate. A metadata provider must not execute network, filesystem or worker effects while holding its atomic commit transaction.

## Replay

A completed idempotent request with a fresh nonce:

- records the new nonce;
- does not invoke the domain transition;
- returns the previously committed result digest; and
- advances the metadata revision because the nonce evidence changed.

Repeating an already-recorded nonce remains an error.

An older pending idempotency record may be completed by a fresh verified nonce for the same exact request. This supports migration from pre-TAHTO-7 states without weakening request identity.

## Revision migration

States without `:metadata-revision` are revision `0`. The first committed plan writes revision `1`. The revision belongs to Tahto metadata state and is mirrored by the durable provider's compare-and-swap condition.

## Authority boundary

Greenways OS owns:

```text
installation
consent
application grants
credentials
private keys
```

Installed providers own:

```text
canonical serialization
signature and freshness verification
canonical result verification
durable compare-and-swap storage
```

Tahto owns:

```text
request and replay semantics
deterministic domain transitions
atomic composition rules
closed commit-plan evidence
```

The transition function supplied to the coordinator is reviewed server code. It is not accepted from request data, serialized as a remote service definition or selected by an untrusted catalogue.

## Conformance laws

The executable Hara suite proves:

- success commits nonce, mutation, completion and revision together;
- completed replay does not re-execute domain code;
- stale revisions fail before nonce consumption;
- invalid and mismatched result proofs roll back all logical steps;
- effectful transitions cannot enter a metadata-only commit;
- rejected operations complete without partial mutation;
- repeated nonces remain rejected;
- legacy pending idempotency can be recovered;
- unknown proof fields fail closed; and
- malformed transition results never become commit plans.

## Not provided by this protocol

This protocol does not select a database, open SQLite or Postgres, fsync a file, replicate a transaction, compact replay evidence, execute object transfer, install a signer or close Gate B. Those require concrete installed providers and the complete two-device conformance scenario.
