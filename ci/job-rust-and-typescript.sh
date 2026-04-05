#!/usr/bin/env bash

set -euo pipefail

bash ci/rust-tests.sh
bash ci/rust-build-sbf.sh
bash ci/rust-lint.sh
bash ci/ts-typecheck.sh
bash ci/ts-lint.sh
bash ci/jest.sh
