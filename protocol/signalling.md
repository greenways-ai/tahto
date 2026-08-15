# Ephemeral peer signalling

Tahto uses Hoplite's bounded Nchan channel only to exchange WebRTC negotiation
records. Nchan is not a session database, authority store, or sync log. Greenways
OS supplies device identity, grants, and signing keys; Tahto validates the
directed signalling record and retains its existing durable sync offers and
cursors.

`tahto.signal/0-alpha` is a closed record with:

- `:session`, `:from`, and `:to` bounded identifiers;
- a positive, monotonically increasing `:sequence`;
- `:kind` equal to `"offer"`, `"answer"`, `"candidate"`, or `"close"`;
- a map-valued `:payload`;
- optional positive `:reply-to`.

The Nchan channel identifier is an unguessable, short-lived bearer capability.
Subscribe and publish requests additionally carry a signed `x-tahto-request`
envelope with operation `signal.subscribe` or `signal.publish`. Publisher
admission returns HTTP 304 so Nchan publishes the original bounded body after
authorization.

Once negotiation succeeds, application traffic belongs on a generic Hoplite
WebRTC data-channel host represented to Hara as a Duplex. Tahto consumes that
transport; it does not own ICE, DTLS, SCTP, socket polling, or WebRTC lifecycle
mechanics.
