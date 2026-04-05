#!/usr/bin/env bash

set -euo pipefail

bash ci/job-rust.sh
bash ci/job-typescript.sh
