# Tahto device and sync kernel

This directory contains the Hara-owned state transitions for TAHTO-5.

```text
device.hal
  device enrolment and revocation
  signed-request proof admission
  nonce replay rejection
  idempotency reservation and completion

session.hal
  namespace-scoped push negotiation
  missing-object pull offers
  offline acknowledgements
  monotonic per-device collection cursors
```

The kernel does not parse keys, verify signatures, read request bodies, write files or send network traffic. Installed Greenways OS/Hoplite providers perform those effects and return bounded proofs or opaque handles.

Pairing a device stores an identity and public key only. It does not create administrator authority, application grants, namespace access or key-extraction capability.

See [`protocol/sync.md`](../../../protocol/sync.md) for the normative boundary.
