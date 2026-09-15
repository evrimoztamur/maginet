#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# Cargo.lock pins wasm-bindgen 0.2.91; use the matching CLI.
if [[ "$("${WASM_BINDGEN:-wasm-bindgen}" --version)" != "wasm-bindgen 0.2.91" ]]; then
  echo 'Install wasm-bindgen-cli 0.2.91 or set WASM_BINDGEN to that executable.' >&2
  exit 1
fi
cargo build --locked --release --target wasm32-unknown-unknown --features ios
mkdir -p ios/Web/static/js
"${WASM_BINDGEN:-wasm-bindgen}" --target web --out-dir ios/Web/static/js/pkg target/wasm32-unknown-unknown/release/maginet.wasm
cp static/js/{load,ai-worker}.js ios/Web/static/js/
mkdir -p ios/Web/static/png
cp static/png/atlas.png ios/Web/static/png/
cp html/ios.html ios/Web/index.html
