#!/usr/bin/env bash

set -euo pipefail

bash ci/rust-tests.sh
bash ci/rust-lint.sh
