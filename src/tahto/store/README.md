# Object vault

TAHTO-3 is implemented as a deterministic Hara state machine in:

```text
tahto.store.model
tahto.store.host
tahto.store.vault
```

Hara owns object identity, quota accounting, resumable-upload transitions, immutable metadata, namespace references, range validation, ordered chunk manifests, closure verification, root pins and garbage-collection planning.

Large bytes stay outside ordinary Hara values. The kernel emits a closed set of `tahto.store.host-effect/1` operations to a trusted native/Hoplite adapter, using opaque request-body handles. That adapter is responsible for bounded streaming, hashing, seeking, fsync and atomic installation.

See [`protocol/object-vault.md`](../../../protocol/object-vault.md) for the normative profile and `test/tahto/store/vault_test.hal` for executable conformance.
