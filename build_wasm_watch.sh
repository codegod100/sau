#!/bin/bash
# Watch for changes and auto-build wasm using wasm-pack

set -e

# Check for cargo-watch, install if missing
type cargo-watch >/dev/null 2>&1 || cargo install cargo-watch

cargo watch -i pkg/* -s 'wasm-pack build --target web --out-dir pkg'
