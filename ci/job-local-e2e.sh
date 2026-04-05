#!/usr/bin/env bash

set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

ensure_solana_path

for manifest in "${SBF_LOCAL_E2E_MANIFESTS[@]}"; do
  echo "==> cargo build-sbf (${manifest})"
  cargo build-sbf --manifest-path "${manifest}"
done

mkdir -p "${HOME}/.config/solana"
solana-keygen new --force --no-bip39-passphrase --silent -o "${HOME}/.config/solana/id.json"

solana-test-validator \
  --reset \
  --ledger /tmp/solana-ci-ledger \
  --bpf-program DbAPmgkrpCCZrpBMv5x1ye6nJUreqY313SuQjZsMyjEf contracts/pfda-amm-3/target/deploy/pfda_amm_3.so \
  --bpf-program 65aE9QdVz5bapV19BGt5cyTgVitYpekGwusRoQEovNUi contracts/axis-g3m/target/deploy/axis_g3m.so \
  --bpf-program DeeUnCHcnPG8arbjGTLhTKeDhpPUBper3TDrpFPHnCwy contracts/axis-vault/target/deploy/axis_vault.so \
  > /tmp/solana-test-validator.log 2>&1 &
validator_pid=$!

cleanup() {
  kill "${validator_pid}" || true
}
trap cleanup EXIT

export SOLANA_URL=http://localhost:8899
solana config set --url localhost
until solana -u localhost cluster-version >/dev/null 2>&1; do
  sleep 1
done

solana -u localhost airdrop 2 || echo "Airdrop unavailable on this CLI/runtime; continuing with existing local balance."

bun run e2e:pfda-amm-3:local
bun run e2e:axis-g3m:local
bun run e2e:axis-vault:local
