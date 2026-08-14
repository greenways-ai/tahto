# Greenways Chats semantic profile

`greenways.chats-profile/0-alpha` is the application-owned specification for
installation-local Chats stored through Tahto's existing semantic fabric. It
does not create a second state machine, provider API, authorization realm, or
query language.

## Coordinate and resources

The profile is fixed to the Tahto coordinate:

```text
application  greenways.chats
namespace    local
collection   chats
head         main / primary
```

Greenways clients address the route-independent resources:

```text
greenways:tahto/collection/chats
greenways:tahto/chat/<32-lowercase-hex-id>
```

A resource URI never contains a host, socket, provider, path, credential,
handle, or storage implementation.

## Exact installed specification

Chat and Message semantic objects use exact schema references from:

```text
package          hara:greenways/chats-profile
package version  0.1.0
schema version   1
```

The package root remains mandatory immutable evidence supplied by trusted
installation. The two exported schema entries are:

```text
greenways.chat          -> greenways.chats/chat-spec
greenways.chat-message  -> greenways.chats/message-spec
```

A schema reference does not install or fetch code.

## Application values

A Chat is one closed `greenways.chat/0-alpha` value:

```clojure
{:protocol "greenways.chat/0-alpha"
 :id <32 lowercase hex>
 :title <1..512 characters>
 :source
 {:protocol "greenways.chat-source/0-alpha"
  :provider <bounded identifier>
  :source-id <1..1024 characters>}
 :created-at <UTC timestamp>
 :updated-at <UTC timestamp not before created-at>
 :message-count <0..100000>}
```

A Message is one closed `greenways.chat-message/0-alpha` value:

```clojure
{:protocol "greenways.chat-message/0-alpha"
 :id <32 lowercase hex>
 :role "system" | "user" | "assistant" | "tool"
 :content <0..262144 characters>
 :created-at <UTC timestamp>}
```

Private prompts, credentials, browser cookies, routes, paths, native handles,
and provider configuration are not profile fields. `source-id` is a bounded
opaque external reference, not a reusable authorization value.

## Semantic identity and links

Stable semantic IDs are:

```text
chat/<chat-id>
message/<message-id>
```

A Chat semantic envelope contains exactly `message-count` ordered typed links
with role `greenways.chat/message`. A Message semantic envelope contains one
parent link with role `greenways.message/chat`. Every link retains both the
stable target ID and exact semantic-object root. The ordinary Tahto semantic
index and root continue to prove link closure, stable-ID selection, schema set,
and complete collection identity.

## Closed query algebra

The profile exposes only three selectors through
`greenways.chats-query/0-alpha`:

```text
recent    bounded cursor page of summaries, newest first
chat      exact Chat ID lookup
messages  bounded forward/backward page, chronological within a page
```

Limits are positive and at most 100. Cursors are non-negative offsets. Unknown
selectors, fields, predicates, sort expressions, database clauses, and
out-of-range cursors fail closed.

Recent ordering is deterministic by descending `updated-at` and ascending Chat
ID. Message ordering is deterministic by ascending `created-at` and ascending
Message ID. Query results contain application values and stable logical URIs
only; they contain no Tahto provider or route state.

## Fabric integration

This profile is layered over, rather than duplicated beside, Tahto's existing
boundaries:

```text
verified canonical application value
  -> Chats schema predicate
  -> tahto.semantic-object/index/root admission
  -> tahto.semantic.prepare
  -> signed commit/head records
  -> tahto.semantic.submit
  -> hoplite.store compare-and-swap + receipt
  -> tahto.semantic-change-feed invalidation
```

The semantic-read adapter, profile mutation orchestration, persistent-provider
fixture, deterministic 1,000-Chat corpus, and release benchmark are subsequent
commits on issue #74. The model/query layer in this document is pure and has no
host effects.
