#!/bin/bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install cargo-server 
cargo install cargo-watch 