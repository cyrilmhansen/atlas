#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

./scripts/build-web.sh
output_dir=${ATLAS_WEB_OUTPUT:-build/web}

node_output=build/web-node
rm -rf "$node_output"
mkdir -p "$node_output"
wasm-bindgen \
  --target nodejs \
  --out-dir "$node_output" \
  --out-name atlas_web \
  target/wasm32-unknown-unknown/release/atlas_web_wasm.wasm

node web/tests/is_sorted.cjs "$node_output/atlas_web.js"
node web/tests/is_sorted_trace.cjs "$node_output/atlas_web.js"
node web/tests/is_sorted_stepper.cjs "$node_output/atlas_web.js"
node web/tests/is_sorted_visual_machine.cjs "$node_output/atlas_web.js" "$output_dir/data/atlas.json"
node web/tests/insertion_sort.cjs "$node_output/atlas_web.js"
node web/tests/insertion_stepper.cjs "$node_output/atlas_web.js"
node web/tests/reverse.cjs "$node_output/atlas_web.js"
node web/tests/reverse_stepper.cjs "$node_output/atlas_web.js"
node web/tests/minimum_visual_machine.cjs "$node_output/atlas_web.js" "$output_dir/data/atlas.json"
node web/tests/partition_visual_machine.cjs "$node_output/atlas_web.js" "$output_dir/data/atlas.json"
node web/tests/generator.mjs
node web/tests/playback.mjs
node web/tests/scale_growth.mjs "$node_output/atlas_web.js"
node web/tests/projection.cjs "$output_dir/data/atlas.json"

test -s "$output_dir/index.html"
test -s "$output_dir/styles.css"
test -s "$output_dir/app.js"
test -s "$output_dir/generator.mjs"
test -s "$output_dir/playback.mjs"
test -s "$output_dir/pkg/atlas_web.js"
test -s "$output_dir/pkg/atlas_web_bg.wasm"

printf 'Atlas Web acceptance slice passed.\n'
