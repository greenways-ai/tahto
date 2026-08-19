# Tahto development environment

The Ubuntu 24.04 devcontainer and Codex cloud use one idempotent bootstrap.
Codex runs it in the universal image, not by building this Dockerfile, and may
run it again while maintaining a cached environment.

## Setup and maintenance

```sh
bash .devcontainer/post-create.sh
```

Use these Codex environment values:

- **Setup script:** `bash .devcontainer/post-create.sh`
- **Maintenance script:** `bash .devcontainer/post-create.sh`
- **Agent internet access:** not required for the protocol, HAL, Rust, and site checks below after setup
- **Docker integration:** not required by Tahto's documented checks

Setup reproduces CI's exact `.dependencies/technology` layout, builds and
installs Hara plus Hoplite tools, stages the two Hoplite-owned HAL contracts,
prepares the frozen Rust 1.85 workspace, and installs site packages. Existing
dependency checkouts must be clean and exact; they are never reset.

## Representative offline checks

```sh
python3 conformance/check-bootstrap.py
python3 conformance/check-protocol.py
bash conformance/check-hal-layout.sh
bash conformance/check-hal-contracts-a.sh
bash conformance/check-hal-contracts-b.sh
bash conformance/check-hal-security.sh
cargo +1.85.0 fmt --manifest-path native/Cargo.toml --all -- --check
cargo +1.85.0 test --locked --manifest-path native/Cargo.toml --workspace
npm run build --prefix site
```

The full native HAL suites can be run with `hara-test` exactly as shown in
`.github/workflows/bootstrap-conformance.yml`. Setup does not start the Tahto
node. Ports `58100` and `4321` are forwarded for explicit node and Astro runs.
