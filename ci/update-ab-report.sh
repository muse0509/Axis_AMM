#!/usr/bin/env bash

set -euo pipefail

HISTORY_DIR="${1:-reports/ab/history}"
LATEST_DIR="reports/ab"

mkdir -p "${HISTORY_DIR}" "${LATEST_DIR}"

bun run ab:rehearsal -- --export markdown --out-dir "${HISTORY_DIR}"

latest_json="$(ls -1t "${HISTORY_DIR}"/ab-rehearsal-*.json | head -n 1)"
latest_md="$(ls -1t "${HISTORY_DIR}"/ab-rehearsal-*.md | head -n 1)"

cp "${latest_json}" "${LATEST_DIR}/latest.json"
cp "${latest_md}" "${LATEST_DIR}/latest.md"

generated_at="$(node -e "const fs=require('fs');const p=process.argv[1];const j=JSON.parse(fs.readFileSync(p,'utf8'));process.stdout.write(j.generatedAt||'');" "${LATEST_DIR}/latest.json")"

cat > "${LATEST_DIR}/README.md" <<README
# Axis A/B Rehearsal Report

- Latest generated: ${generated_at}
- Source JSON: [latest.json](./latest.json)
- Source Markdown: [latest.md](./latest.md)

Historical artifacts are stored under [history/](./history/).
README
