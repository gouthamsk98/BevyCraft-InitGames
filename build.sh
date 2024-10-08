#!/bin/bash

# Run cargo build for the wasm32 target
cargo build --target wasm32-unknown-unknown --release

# Run wasm-bindgen to generate the JS bindings
wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/bevycraft.wasm

# Run the server
cargo server --path ./out &

# Wait a moment to ensure the server is running
sleep 5

# Refresh the browser at port 800
# open http://localhost:8000