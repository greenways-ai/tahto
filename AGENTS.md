# Tahto contributor contract

Tahto is application-neutral infrastructure authored as Hara programs.

Before changing core code, preserve these laws:

- Greenways OS owns installation, consent, private keys, credentials, and grants.
- Tahto owns application-neutral identity, custody, movement, immutable history,
  graph closure and recovery—not application field meaning.
- Applications and exact installed specification packages own domain invariants,
  transformations, migrations, execution and merge policy.
- Tahto domain state, effect interpretation, orchestration, recovery policy and
  result validation remain in `.hal`.
- Native operating-system and database mechanics use application-neutral
  `hara.store`, `hara.blob` and related Hara/Hoplite capabilities. Do not add a
  Tahto-specific Rust or C provider.
- A portable call plan contains only `{service, operation, arguments}`. HAL may
  not select an ABI, provider package, driver, database path, storage root,
  credential, command or remote executable catalogue.
- Request and response source handles are ephemeral host resources. A numeric
  handle is never authority by itself; production ownership is exact request
  context + work + handle. Handles never enter objects, commits, heads, receipts,
  backups, semantic roots or metadata snapshots.
- The existing `native/` metadata implementation is frozen migration evidence.
  It may not gain new Tahto semantics and is removed through issue #17 after
  generic-provider parity and semantic recovery are proved.
- Divergent valid heads are preserved; core never applies generic
  last-write-wins or invents an application merge.
- Source and rebuildable derived state remain distinct.
- Semantic profiles may define exact schema references, stable logical identity,
  typed content-addressed links, indexes and roots. They must not contain Hodos,
  Alumbra or another application's domain fields.
- A schema reference pins an exact installed package root and exported entry. It
  is data, not authority to install, fetch or execute remote code.
- Runtime validation never depends on calling `specs.hara-lang.org`; public Specs
  is discovery and documentation, while installed locked packages supply the
  executable validator.
- Application worker implementations remain in application repositories.
- Service descriptors are inert and never install remote JavaScript, HTML, HAL,
  arbitrary Wasm or native commands into Greenways OS.
- Greenways Space is optional.
- Tahto core must not depend on Historia, Hestia, Spaces, Worlds, Ignatius,
  Hodos or Alumbra repositories.

Ordered Semantic Fabric work is tracked by #29:

```text
#23 baseline documentation/status
#30 semantic value profiles
#31 semantic object admission
#32 stable indexes and roots
#33 existing commit/head integration
#34 bounded canonical-value verification
#35 authenticated semantic operations
#17 recovery proof and native cleanup
```

Architectural PR descriptions use: Decision, Authority boundary, Protocol and
storage, Migration, Compatibility, Security laws, Conformance, and Not included.
