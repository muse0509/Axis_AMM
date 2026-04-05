#!/usr/bin/env bash

set -euo pipefail

bash ci/rust-lint.sh
bash ci/ts-lint.sh
