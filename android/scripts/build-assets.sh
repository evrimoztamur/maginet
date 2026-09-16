#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
if [[ "$("${WASM_BINDGEN:-wasm-bindgen}" --version)" != 'wasm-bindgen 0.2.91' ]]; then
  echo 'Set WASM_BINDGEN to wasm-bindgen-cli 0.2.91 (matching Cargo.lock).' >&2; exit 1
fi
cargo build --locked --release --target wasm32-unknown-unknown --features android
DEST=android/app/src/main/assets/web
mkdir -p "$DEST/static/js" "$DEST/static/png"
"${WASM_BINDGEN:-wasm-bindgen}" --target web --out-dir "$DEST/static/js/pkg" target/wasm32-unknown-unknown/release/maginet.wasm
cp static/js/{load,ai-worker}.js "$DEST/static/js/"
cp static/png/atlas.png "$DEST/static/png/"
cp html/ios.html "$DEST/index.html"
