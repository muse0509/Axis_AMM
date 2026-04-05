#!/usr/bin/env bash

set -euo pipefail

rustup update stable
rustup default stable
rustc --version
cargo --version
