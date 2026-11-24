#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT/builds/sbpf-v3"
ANCHOR_VERSION="1.2.2"
CLI_RUST_VERSION="1.98.1"
PLATFORM_TOOLS_VERSION="v1.57"
ANCHOR_ROOT="$ROOT/.tools/anchor-$ANCHOR_VERSION"
ANCHOR="$ANCHOR_ROOT/bin/anchor"

if [[ ! -x "$ANCHOR" ]] || [[ "$($ANCHOR --version)" != "anchor-cli $ANCHOR_VERSION" ]]; then
  rustup toolchain install "$CLI_RUST_VERSION" --profile minimal
  cargo "+$CLI_RUST_VERSION" install anchor-cli \
    --version "$ANCHOR_VERSION" \
    --locked \
    --root "$ANCHOR_ROOT"
fi

BUILD_SBF="$(command -v cargo-build-sbf || true)"
if [[ -z "$BUILD_SBF" ]]; then
  echo "error: cargo-build-sbf is not available in PATH" >&2
  exit 1
fi

echo "Building sBPF v3 with $($ANCHOR --version)"
echo "Using $BUILD_SBF ($($BUILD_SBF --version | head -n 1))"
cd "$BUILD_DIR"
NO_DNA=1 "$ANCHOR" build \
  --no-idl \
  --arch v3 \
  --tools-version "$PLATFORM_TOOLS_VERSION" \
  -- \
  --sbf-out-dir "$ROOT/target/sbpf-v3/deploy"
