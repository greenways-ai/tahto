# Tahto semantic-head change feed

`tahto.semantic-change-feed/0-alpha` is a bounded invalidation feed over
committed semantic-head identity. The exported package entry is
`tahto.change-feed.facade`; `tahto.change-feed` remains the bounded internal
engine.

An event is accepted only when a valid
`tahto.metadata-commit-receipt/0-alpha` has the same revision as the recoverable
Tahto state supplied to `publish`.

Events contain only the coordinate, previous/current head representations,
metadata revision and cursor. A head representation is exactly one of:

- `none` with no commits or head digests;
- `current` with one current commit and one or more signed head digests;
- `divergent` with every current commit and all signed head digests.

Subscriptions use the same admitted pending `head.read` device context and exact
coordinate as the client facade. Replayed or completed contexts cannot create,
poll, or acknowledge subscriptions. Active subscriptions, queues, retained
event history and serialized event size are bounded. Exact duplicate
publication retains the same cursor. Changed bytes at the same
coordinate/revision collide. Expired or foreign-coordinate cursors and queue
overflow return `tahto.change/resync-required`.

## Restart checkpoint

`checkpoint` emits one closed
`tahto.semantic-change-checkpoint/0-alpha` value containing only:

```text
limits
next cursor
latest emitted metadata revision
exact provider revision observed at checkpoint time
retained canonical events
```

Live subscription IDs, devices, queues, request contexts, routes, provider
configuration, credentials, keys, native handles and application values are
never checkpointed.

A checkpoint is valid only when its event cursors are strictly increasing,
metadata revisions are monotonic, its final event agrees with its next cursor
and latest revision, every event remains within the reviewed size limit, and
the retained history remains within the reviewed count limit.

`restore` accepts only a pristine feed configured with exactly the same limits.
It restores no live subscription. The exact provider revision must still match
the recoverable committed state, and the latest retained event for every
coordinate must still equal the current semantic head. A missed commit, changed
head, stale checkpoint, widened field set or corrupted sequence returns a
bounded failure; committed-state drift returns
`tahto.change/resync-required`.

Recovery after `resync-required` is a fresh `tahto.head/read` followed by the
appropriate `tahto.sync/plan`; the feed is not an object-value stream or an
unbounded replay API. Transport framing, WebSocket lifecycle and durable
checkpoint-file mechanics remain embedding-host responsibilities.
