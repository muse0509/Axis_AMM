#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

for project in "${TS_PROJECTS[@]}"; do
  echo "==> tsc --noEmit (${project})"
  bunx tsc --noEmit -p "${project}"
done
