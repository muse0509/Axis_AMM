#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

ensure_solana_path

if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "${SOLANA_BIN_DIR}" >> "${GITHUB_PATH}"
fi

if command -v solana >/dev/null 2>&1 && command -v cargo-build-sbf >/dev/null 2>&1; then
  echo "Using cached Solana CLI from ${SOLANA_BIN_DIR}"
  solana --version
  cargo-build-sbf --version
  exit 0
fi

sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
ensure_solana_path

solana --version
cargo-build-sbf --version
