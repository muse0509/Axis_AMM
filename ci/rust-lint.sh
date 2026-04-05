#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

for manifest in "${RUST_MANIFESTS[@]}"; do
  echo "==> cargo clippy (${manifest})"
  cargo clippy --manifest-path "${manifest}" --all-targets
done
