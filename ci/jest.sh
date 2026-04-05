#!/usr/bin/env bash

set -euo pipefail

bunx jest --config jest.config.cjs --runInBand
