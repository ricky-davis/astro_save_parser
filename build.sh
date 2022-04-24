#!/bin/bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir www target/wasm32-unknown-unknown/debug/astro_save_editor_wasm.wasm
wasm-gc www/astro_save_editor_wasm_bg.wasm
pushd www
python server.py
popd