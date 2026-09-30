#!/bin/sh
set -eu
cargo build --release --target wasm32-unknown-unknown
for c in inner outer; do
  wasm-tools component new "target/wasm32-unknown-unknown/release/$c.wasm" -o "$c.wasm"
done
wac plug outer.wasm --plug inner.wasm -o composed.wasm
npx jco transpile composed.wasm -o out --map 'example:repro/host=../host.js'
