# Tahto semantic-head change feed

`tahto.semantic-change-feed/0-alpha` is a bounded invalidation feed over
committed semantic-head identity. An event is accepted only when a valid
`tahto.metadata-commit-receipt/0-alpha` has the same revision as the recoverable
Tahto state supplied to `publish`.

Events contain only the coordinate, previous/current head representations,
metadata revision and cursor. A head representation is exactly one of:

- `none` with no commits or head digests;
- `current` with one current commit and one or more signed head digests;
- `divergent` with every current commit and all signed head digests.

Subscriptions use the same verified `head.read` device context and exact
coordinate as the client facade. Active subscriptions, queues, retained event
history and serialized event size are bounded. Exact duplicate publication
retains the same cursor. Changed bytes at the same coordinate/revision collide.
Expired or foreign-coordinate cursors and queue overflow return
`tahto.change/resync-required`.

Recovery is a fresh `tahto.head/read` followed by the appropriate
`tahto.sync/plan`; the feed is not an object-value stream or an unbounded replay
API. Transport framing and WebSocket lifecycle remain adapter concerns.
