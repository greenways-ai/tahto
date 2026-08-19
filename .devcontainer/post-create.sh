#!/usr/bin/env bash
set -Eeuo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[[ -n "$repo_root" ]] || { echo "error: run from a Tahto checkout" >&2; exit 1; }

HARA_REVISION="e1ecbd0f7275a64806ac95c48ac1803b357afcae"
HOPLITE_REVISION="58cc7a471ace6fdcc30307e9f972089f6f027291"
HOPLITE_VALUE_REVISION="d51c5954e427ea84439477135b970a3e1145c190"
DEPENDENCIES="$repo_root/.dependencies/technology"
HARA_CHECKOUT="$DEPENDENCIES/hara"
HOPLITE_CHECKOUT="$DEPENDENCIES/hoplite"
HOPLITE_VALUE_CHECKOUT="$DEPENDENCIES/hoplite-value"

fail() { echo "error: $*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"; }

persist_local_bin() {
  local line='export PATH="$HOME/.local/bin:$PATH"'
  mkdir -p "$HOME/.local/bin"
  touch "$HOME/.bashrc"
  grep -Fqx "$line" "$HOME/.bashrc" || printf '\n%s\n' "$line" >> "$HOME/.bashrc"
  export PATH="$HOME/.local/bin:$PATH"
}

select_node() {
  export NVM_DIR="${NVM_DIR:-$HOME/.nvm}"
  if [[ -s "$NVM_DIR/nvm.sh" ]]; then
    # shellcheck disable=SC1090
    source "$NVM_DIR/nvm.sh"
    nvm install 24
    nvm use 24
  fi
  need node; need npm
  [[ "$(node -p 'process.versions.node.split(".")[0]')" == 24 ]] \
    || fail "Node 24 is required; found $(node --version)"
}

ensure_rust() {
  need rustup
  rustup toolchain install 1.88.0 --profile minimal
  rustup toolchain install 1.85.0 --profile minimal --component rustfmt
  need cargo
}

ensure_checkout() {
  local repository="$1" revision="$2" checkout="$3"
  if [[ -e "$checkout" ]]; then
    [[ -d "$checkout/.git" ]] || fail "$checkout exists but is not a Git checkout"
    [[ -z "$(git -C "$checkout" status --porcelain --untracked-files=all)" ]] \
      || fail "dependency checkout is dirty: $checkout"
    local actual
    actual="$(git -C "$checkout" rev-parse HEAD)"
    [[ "$actual" == "$revision" ]] \
      || fail "dependency revision mismatch at $checkout (expected $revision, found $actual); refusing to reset it"
    return
  fi
  mkdir -p "$(dirname "$checkout")"
  local temporary="${checkout}.tmp.$$"
  rm -rf "$temporary"
  git clone --filter=blob:none --no-checkout "$repository" "$temporary"
  git -C "$temporary" fetch --depth 1 origin "$revision"
  git -C "$temporary" checkout --detach "$revision"
  mv "$temporary" "$checkout"
}

print_version() {
  local label="$1"; shift
  printf '%-18s ' "$label:"
  "$@" --version 2>&1 | head -n 1 || true
}

persist_local_bin
select_node
ensure_rust
need git; need python3

[[ "$(tr -d '[:space:]' < "$repo_root/packaging/hara-revision")" == "$HARA_REVISION" ]] \
  || fail "packaging/hara-revision differs from the reviewed CI pin"
[[ "$(tr -d '[:space:]' < "$repo_root/packaging/hoplite-revision")" == "$HOPLITE_REVISION" ]] \
  || fail "packaging/hoplite-revision differs from the reviewed CI pin"

ensure_checkout "https://github.com/hara-lang/hara.git" "$HARA_REVISION" "$HARA_CHECKOUT"
ensure_checkout "https://github.com/greenways-ai/hoplite.git" "$HOPLITE_REVISION" "$HOPLITE_CHECKOUT"
ensure_checkout "https://github.com/greenways-ai/hoplite.git" "$HOPLITE_VALUE_REVISION" "$HOPLITE_VALUE_CHECKOUT"

hara_manifest="$HARA_CHECKOUT/core/rust/Cargo.toml"
hoplite_manifest="$HOPLITE_CHECKOUT/core/Cargo.toml"
[[ -f "$hara_manifest" ]] || fail "pinned Hara manifest is missing"
[[ -f "$hoplite_manifest" ]] || fail "pinned Hoplite manifest is missing"

cargo +1.88.0 fetch --locked --manifest-path "$hara_manifest"
cargo +1.88.0 build --locked --release --manifest-path "$hara_manifest" --bin hara --bin hara-test
install -m 0755 "$HARA_CHECKOUT/core/rust/target/release/hara" "$HOME/.local/bin/hara"
install -m 0755 "$HARA_CHECKOUT/core/rust/target/release/hara-test" "$HOME/.local/bin/hara-test"

cargo +1.88.0 fetch --locked --manifest-path "$hoplite_manifest"
cargo +1.88.0 build --locked --release --manifest-path "$hoplite_manifest" --bin hoplite
cargo +1.88.0 build --locked --release --manifest-path "$hoplite_manifest" \
  --features application-console --bin hoplite-console-bundle
install -m 0755 "$HOPLITE_CHECKOUT/core/target/release/hoplite" "$HOME/.local/bin/hoplite"
install -m 0755 "$HOPLITE_CHECKOUT/core/target/release/hoplite-console-bundle" \
  "$HOME/.local/bin/hoplite-console-bundle"

mkdir -p "$repo_root/src/hoplite"
cp "$HOPLITE_VALUE_CHECKOUT/core/lib/src/hoplite/value.hal" "$repo_root/src/hoplite/value.hal"
cp "$HOPLITE_CHECKOUT/core/lib/src/hoplite/response_source.hal" "$repo_root/src/hoplite/response_source.hal"

cargo +1.85.0 fetch --locked --manifest-path "$repo_root/native/Cargo.toml"
npm ci --prefix "$repo_root/site"

[[ -z "$(git -C "$repo_root" status --porcelain --untracked-files=all)" ]] \
  || fail "setup changed the Tahto working tree"

printf '\nTahto development environment ready.\n'
print_version "Node" node
print_version "npm" npm
print_version "Rust 1.88" rustc +1.88.0
print_version "Rust 1.85" rustc +1.85.0
print_version "Hara" hara
print_version "hara-test" hara-test
print_version "Hoplite" hoplite
printf 'Hara revision:          %s\n' "$(git -C "$HARA_CHECKOUT" rev-parse HEAD)"
printf 'Hoplite revision:       %s\n' "$(git -C "$HOPLITE_CHECKOUT" rev-parse HEAD)"
printf 'Hoplite value revision: %s\n' "$(git -C "$HOPLITE_VALUE_CHECKOUT" rev-parse HEAD)"
cat <<'CHECKS'

Available checks (dependencies are prepared for offline execution):
  python3 conformance/check-bootstrap.py
  python3 conformance/check-protocol.py
  bash conformance/check-hal-layout.sh
  bash conformance/check-hal-contracts-a.sh
  bash conformance/check-hal-contracts-b.sh
  bash conformance/check-hal-security.sh
  cargo +1.85.0 fmt --manifest-path native/Cargo.toml --all -- --check
  cargo +1.85.0 test --locked --manifest-path native/Cargo.toml --workspace
  npm run build --prefix site
CHECKS
