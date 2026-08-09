# Tahto upload integration

## Decision

Tahto commits upload state only after the exact generic `hoplite.blob` result has
passed the same closed HAL validator used by the in-memory capability profile.

The integration remains entirely in HAL:

```text
vault transition
  validates authorization, quota, offset and lifecycle
        |
        | candidate state + one closed upload effect
        v
hoplite.blob capability
  moves bounded bytes, verifies content and owns native resources
        |
        | exact generic result
        v
upload orchestrator
  commits, reconciles or rolls back the candidate state
```

No Tahto-specific native executor is introduced.

## Candidate-state law

`begin-upload`, `append-upload`, `abort-upload` and `request-install` produce
candidate state. The candidate is not returned as authoritative merely because
the domain transition was valid.

The orchestrator executes the one emitted effect through
`tahto.store.capability`. If preparation, provider execution, result closure or
result identity fails, it returns the original state. Optimistic append offsets,
removed uploads and the temporary `verifying` status therefore never leak
through a failed host operation.

## Successful operations

- `upload/open` reconciles the provider's verified resumable offset into the new
  Tahto upload, bounded by its declared size.
- `upload/append` commits the exact offset already validated by the vault only
  after the provider reports the matching consumed length and next offset.
- `upload/abort` removes Tahto state only after the generic staging layer reports
  an exact idempotent abort result.
- `upload/verify-install` translates the exact generic commit result into the
  existing Tahto installation proof and then calls `vault/accept-install`.

The final install records the immutable object and namespace reference, removes
the upload and exposes no generic effect or native source handle in durable
state.

## Execution profiles

Pure tests and alternate hosts use `begin-with`, `append-with`, `abort-with` and
`install-with` with an injected capability function. Production handlers use
the corresponding async functions, which await the installed `hoplite.blob`
service. Both paths share the same candidate-state acceptance functions.

Provider rejection remains a rejected coroutine operation in the production
path. Returned provider values, including successful promises, cannot bypass the
closed result validator.

## Scope

This slice integrates upload ingress and verified installation only. Immutable
response-source transport and Tahto range-read integration remain separate
ordered changes. `object/read-range` effects are explicitly rejected by the
upload orchestrator.
