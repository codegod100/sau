#!/bin/bash
# Build the project using wasm-pack and output to the pkg directory

set -e
#!/bin/bash
# Build the project using wasm-pack and output to the pkg directory
# If called with --watch, use cargo-watch to auto-build on change

set -e

if [ "$1" = "--watch" ]; then
    # Check for cargo-watch, install if missing
    if ! command -v cargo-watch >/dev/null 2>&1; then
        cargo install cargo-watch
    fi
    cargo watch -i pkg/* -s 'wasm-pack build --target web --out-dir pkg'
else
    wasm-pack build --target web --out-dir pkg
fi
