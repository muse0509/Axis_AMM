#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

ensure_solana_path

for manifest in "${SBF_BUILD_MANIFESTS[@]}"; do
  echo "==> cargo build-sbf (${manifest})"
  cargo build-sbf --manifest-path "${manifest}"
done
