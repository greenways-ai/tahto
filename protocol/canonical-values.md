# Bounded canonical values

Tahto uses Hara's application-neutral canonical-value capability to read small immutable HTA values without moving byte custody, filesystem paths or decoder selection into semantic state.

```text
namespace-authorized immutable object
  -> hoplite.value / object/verify-hta
  -> exact digest + byte length + portable value
  -> Tahto schema-bound verification evidence
```

## Portable request

```clojure
{:protocol "hoplite.value-request/1"
 :operation "object/verify-hta"
 :digest "sha256:..."
 :max-bytes 1048576}
```

The request contains no provider, driver, path, URL, credential, source handle, callback or command. Trusted installation binds `hoplite.value` to a provider.

## Authority split

Tahto owns:

- object installation and namespace authorization checks before dispatch;
- the semantic 1 MiB maximum;
- exact request/result identity matching;
- agreement between verified byte length and the installed object descriptor;
- translation into `tahto.canonical-value-verification/1`;
- all later schema validation and semantic mutation.

Hara owns the closed generic request/result vocabulary and canonical portable-value profile.

The installed provider owns bounded reads, SHA-256 over actual bytes, canonical HTA decoding and stable generic failure evidence.

Specification packages own application value meaning. Provider success alone never admits a semantic object.

## Tahto result

A successful schema-bound read returns ephemeral composition data:

```clojure
{:value decoded-portable-value
 :verification
 {:protocol "tahto.canonical-value-verification/1"
  :verified true
  :profile "hara.hta/1"
  :schema-ref exact-installed-schema-ref
  :value-root "sha256:..."
  :value-size 512}}
```

The decoded value is not added to durable metadata by this adapter. Semantic admission retains only its existing bounded projection.

## Failure laws

The adapter fails without state mutation when:

- the input is not closed;
- the object is missing or outside the authorized namespace;
- the installed object exceeds the requested maximum;
- the host call or result is widened or malformed;
- digest or maximum identity does not match;
- verified byte length differs from the installed descriptor;
- the provider reports canonicality, digest, object or provider failure;
- a schema reference is not exact and immutable.

Large media, chunks and response bodies remain on `hoplite.blob` and `hoplite.response-source/1` paths.
