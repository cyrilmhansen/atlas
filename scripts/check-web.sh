#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

./scripts/build-web.sh

node_output=build/web-node
rm -rf "$node_output"
mkdir -p "$node_output"
wasm-bindgen \
  --target nodejs \
  --out-dir "$node_output" \
  --out-name atlas_web \
  target/wasm32-unknown-unknown/release/atlas_web_wasm.wasm

node web/tests/is_sorted.cjs "$node_output/atlas_web.js"
node web/tests/projection.cjs build/web/data/atlas.json

test -s build/web/index.html
test -s build/web/styles.css
test -s build/web/app.js
test -s build/web/pkg/atlas_web.js
test -s build/web/pkg/atlas_web_bg.wasm

printf 'MVP 5 Web acceptance slice passed.\n'
