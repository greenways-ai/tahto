# Tahto console sandbox bundle

`tahto-console.hcb` is the deterministic HCB0 envelope for the fixed
`tahto.console` client namespace. Its source of truth remains
`src/tahto/console.hal`; the bundle does not contain the server dispatcher,
grants, credentials, executable selection, paths, descriptors, or handles.

`manifest.hta` is inert immutable data. Its `:digest` is SHA-256 over the exact
886 bundle bytes and is directly usable in a Hara `SandboxSpec` bundle
reference together with `:format`.

Reproduce the artifact with the reviewed Hoplite revision recorded in
`packaging/hoplite-revision`:

```text
hoplite-console-bundle \
  --namespace tahto.console \
  --source src/tahto/console.hal \
  --output tahto-console.hcb
```

The builder creates a read-only file and refuses to overwrite an existing
artifact. CI rebuilds into a fresh directory, compares every byte, verifies the
declared length and digest, and runs the HAL call and rejection fixtures.
