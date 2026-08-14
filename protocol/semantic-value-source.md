# Selected semantic value response source

Tahto projects one selected immutable value through the existing portable Hoplite response-source boundary. It does not define another streaming protocol, provider catalogue, storage handle, or durable source record.

## Request

`tahto.semantic-value-source-request/0-alpha` is a closed value containing only:

```text
read-result
commit-root
stable ID
offset
length
```

The admitted `semantic.read` request context is supplied separately by the trusted service boundary. A caller cannot choose application, namespace, collection, device, request digest, provider, route, work identity, authority root, native handle, path, URL, or credential.

## Validation order

Before any host callback, Tahto:

1. validates the pending, non-replayed semantic-read context;
2. validates the closed request and non-zero safe range;
3. binds the complete read result to the exact context;
4. reruns the same bounded semantic read against current state;
5. requires byte-for-byte equality with the supplied read result;
6. selects exactly one branch by commit root;
7. selects the exact stable ID within that branch;
8. verifies the semantic-object root, immutable value root, declared size, installed object, and range.

A forged, stale, foreign, ambiguous, absent, overflowed, or out-of-range request fails before the effect.

## Host effect

Only after validation may Tahto call the reviewed source-open capability with:

```text
service   hoplite.blob
operation object/open-source
request   immutable value digest + exact offset + exact length
```

The installed host owns provider composition and private storage details. Provider selection is static trusted deployment configuration, never request data.

## Result

`tahto.semantic-value-source/0-alpha` binds the exact coordinate, branch commit root, stable ID, semantic-object root, value root, value size, range, and one portable `hoplite.response-source/0-alpha` descriptor.

The result is ephemeral request/work transport state. It is not written to Tahto metadata, semantic projections, heads, receipts, or change feeds. Tahto state must remain byte-for-byte unchanged.

## Security laws

- The selected branch and stable ID cannot be substituted after authorisation.
- The source descriptor must match the authorised digest and range.
- Unknown request, result, and provider-result fields fail closed.
- No source handle, provider name, path, URL, credential, or native route becomes durable semantic state.
- Copied, stale, or foreign-work descriptors remain subject to Hoplite's work-owned transport checks.
