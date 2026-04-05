#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

ensure_solana_path

if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "${SOLANA_BIN_DIR}" >> "${GITHUB_PATH}"
fi

solana --version
cargo-build-sbf --version
