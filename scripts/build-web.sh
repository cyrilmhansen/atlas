#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

source_commit=${ATLAS_SOURCE_COMMIT:-$(git rev-parse HEAD)}
output_dir=${ATLAS_WEB_OUTPUT:-build/web}
rustc_version=$(rustc --version)
wasm_bindgen_version=$(wasm-bindgen --version)

rm -rf "$output_dir"
mkdir -p "$output_dir/data" "$output_dir/pkg"

cargo run -q -p atlas --locked --offline --example build_web_projection -- \
  "$output_dir/data/atlas.json" "$source_commit" "$rustc_version" "$wasm_bindgen_version"
cargo build -q -p atlas-web-wasm --target wasm32-unknown-unknown --release --locked --offline
wasm-bindgen \
  --target web \
  --out-dir "$output_dir/pkg" \
  --out-name atlas_web \
  target/wasm32-unknown-unknown/release/atlas_web_wasm.wasm

cp web/index.html web/styles.css web/app.js "$output_dir/"

printf 'Built Atlas Web bundle at %s\n' "$output_dir"
