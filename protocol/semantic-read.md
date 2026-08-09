# Bounded semantic reads

`semantic.read` is the first application-facing Semantic Fabric operation. It projects an already-admitted semantic head without selecting a winning branch or decoding application values.

```text
pending verified device context
  + head kind/name
  + maximum commit count
  + optional stable logical ID
        ↓
head digest and bounded head summary
  + ordered branch projections
  + optional selected semantic object per branch
```

## Request

The operation consumes a request-local value:

```clojure
{:kind "main"
 :name "primary"
 :max-commits 4
 :id "entity/tree"}
```

`:id` is optional. `:max-commits` is mandatory, positive and capped at 16. A wider head fails; it is never truncated.

The associated `tahto.device-request-context/1` must:

- use operation `semantic.read`;
- be pending with no result digest;
- identify an enrolled device;
- carry the exact application, namespace and collection coordinate.

## Result

A successful result contains:

```text
request and device identity
head digest and bounded head summary
one branch for every head commit, in exact stored order
```

Each branch contains only:

```text
commit identity summary
selected semantic-root digest and projection
semantic-index identity and entry count
optional semantic-object projection for the requested stable ID
```

The stable ID is resolved independently in every divergent branch. It may be absent from one branch and present in another. Tahto does not classify the difference or select a winner.

## Authority boundary

Tahto owns:

- authenticated request-context checks;
- current device status;
- head, commit, semantic-root and semantic-index consistency;
- divergence preservation;
- exact bounds and closed result fields.

Applications own the meaning of selected objects and branch differences. Hodos or another client may project the result. `hara.value` remains a separate explicit operation when canonical application bytes must be decoded.

## Security and compatibility laws

- reads never mutate metadata state;
- no decoded application value is returned;
- no provider, path, URL, credential, source handle or merge callback is representable;
- missing projections and profile mismatches fail closed;
- ordinary non-semantic heads continue to use existing APIs;
- no commit, head, semantic object, index or root record changes shape;
- HTTP routing, prepare/submit, signing and response-source plans remain later slices.
