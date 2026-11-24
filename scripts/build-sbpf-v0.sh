#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT/builds/sbpf-v0"
ANCHOR_VERSION="0.30.1"
PLATFORM_TOOLS_VERSION="v1.43"
ANCHOR_ROOT="$ROOT/.tools/anchor-$ANCHOR_VERSION"
ANCHOR="$ANCHOR_ROOT/bin/anchor"

if [[ ! -x "$ANCHOR" ]] || [[ "$($ANCHOR --version)" != "anchor-cli $ANCHOR_VERSION" ]]; then
  rustup toolchain install 1.79.0 --profile minimal
  cargo +1.79.0 install anchor-cli \
    --version "$ANCHOR_VERSION" \
    --locked \
    --root "$ANCHOR_ROOT"
fi

command -v cargo-build-sbf >/dev/null || {
  echo "error: cargo-build-sbf is not available in PATH" >&2
  exit 1
}

echo "Building sBPF v0 with $($ANCHOR --version) and platform-tools $PLATFORM_TOOLS_VERSION"
cd "$BUILD_DIR"
NO_DNA=1 "$ANCHOR" build --no-idl -- \
  --arch v0 \
  --tools-version "$PLATFORM_TOOLS_VERSION" \
  --sbf-out-dir "$ROOT/target/sbpf-v0/deploy"
