#!/usr/bin/env bash

set -euo pipefail

bash ci/ts-typecheck.sh
bash ci/ts-lint.sh
bash ci/jest.sh
