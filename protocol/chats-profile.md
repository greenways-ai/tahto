# Greenways Chats semantic profile

`greenways.chats-profile/0-alpha` is the application-owned specification for
installation-local Chats stored through Tahto's existing semantic fabric. It
does not create a second state machine, provider API, authorization realm, or
query language.

## Coordinate and resources

The profile is fixed to:

```text
application  greenways.chats
namespace    local
collection   chats
head         main / primary
```

Greenways clients use route-independent resources:

```text
greenways:tahto/collection/chats
greenways:tahto/chat/<32-lowercase-hex-id>
```

A resource URI never contains a host, socket, provider, path, credential,
handle, or storage implementation.

## Exact installed specification

The reviewed package contract is:

```text
package          hara:greenways/chats-profile
package version  0.1.0
schema version   1
```

The exact package root remains mandatory trusted-installation evidence. Exported
schema entries are:

```text
greenways.chat-collection -> greenways.chats/collection-spec
greenways.chat            -> greenways.chats/chat-spec
greenways.chat-message    -> greenways.chats/message-spec
```

A schema reference does not install or fetch code.

## Application values

A Chat is one closed `greenways.chat/0-alpha` value with a 32-lowercase-hex ID,
1–512 character title, bounded opaque source reference, creation/update times,
and a message count bounded at 100,000.

A Message is one closed `greenways.chat-message/0-alpha` value with a
32-lowercase-hex ID, one of `system`, `user`, `assistant`, or `tool`, bounded
content, and a creation time.

Private prompts, credentials, browser cookies, routes, paths, native handles,
and provider configuration are not profile fields. `source-id` is a bounded
opaque external reference, not reusable authorization.

## Semantic identity, materialized collection, and links

Stable semantic IDs are:

```text
collection/chats
chat/<chat-id>
message/<message-id>
```

`greenways.chat-collection/0-alpha` contains at most 1,000 bounded public Chat
summaries ordered by descending `updated-at` and ascending Chat ID. Its semantic
envelope contains one `greenways.chats/chat` link for every summary. The
summary and link ID sets must agree exactly.

A Chat envelope contains exactly `message-count` ordered
`greenways.chat/message` links. A Message envelope contains one
`greenways.message/chat` parent link. Every link retains a stable target ID and
exact semantic-object root. Ordinary Tahto semantic index/root laws continue to
prove link closure, stable-ID selection, schema set, and collection identity.

## Closed query algebra

`greenways.chats-query/0-alpha` exposes only:

```text
recent    bounded cursor page from the materialized collection
chat      exact Chat ID lookup
messages  bounded forward/backward page through the Chat's message links
```

Limits are positive and at most 100. Cursors are non-negative offsets. Unknown
selectors, fields, predicates, sort expressions, database clauses, and
out-of-range cursors fail closed. Message pages are chronological by
`created-at` and Message ID.

## Exact semantic-read projection

The profile issues only the existing `semantic.read` operation against
`main/primary`:

- `recent` selects `collection/chats`;
- `chat` and `messages` select the exact `chat/<id>` stable ID.

Each returned `tahto.semantic-read-result/0-alpha` must match the authenticated
pending context's device, request digest, coordinate, and selected stable ID.
The profile preserves every branch in head order. It never chooses, combines, or
silently merges divergent valid heads.

For each branch it verifies canonical application values through
`hoplite.value`, checks exact value root/size agreement, validates the installed
Chats schema, and then projects a bounded result. Message reads follow only the
exact typed links in that branch's Chat projection. A Chat missing from one
branch remains an explicit `missing` outcome for that branch.

Reads never mutate Tahto state. Changed canonical values, schema references,
link targets, roots, sizes, contexts, or semantic-read envelopes fail before a
Chats result is returned.

## Fabric integration

```text
verified canonical value
  -> Chats schema predicate
  -> semantic object/index/root admission
  -> semantic prepare
  -> signed commit/head records
  -> semantic submit
  -> hoplite.store CAS + receipt
  -> semantic-head invalidation
```

Mutation orchestration, persistent-provider restart evidence, the deterministic
1,000-Chat corpus, and release benchmark remain subsequent #74 slices. Local
application/capability enforcement remains a Greenways OS responsibility.
