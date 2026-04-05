#!/usr/bin/env bash

set -euo pipefail

bunx eslint --max-warnings 0 "scripts/**/*.ts" "test/**/*.ts"
