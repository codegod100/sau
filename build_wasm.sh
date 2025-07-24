#!/bin/bash
# Build the project using wasm-pack and output to the pkg directory
# Also build Tailwind CSS
# If called with --watch, use cargo-watch to auto-build on change

set -e

# Function to build CSS
build_css() {
    echo "Building Tailwind CSS..."
    if [ "$1" = "--watch" ]; then
        npx tailwindcss -i ./src/input.css -o ./tailwind/output.css --watch &
        CSS_PID=$!
    else
        npx tailwindcss -i ./src/input.css -o ./tailwind/output.css --minify
    fi
}

# Check if npm dependencies are installed
if [ ! -d "node_modules" ]; then
    echo "Installing npm dependencies..."
    npm install
fi

if [ "$1" = "--watch" ]; then
    # Check for cargo-watch, install if missing
    if ! command -v cargo-watch >/dev/null 2>&1; then
        cargo install cargo-watch
    fi
    
    # Start CSS build in watch mode
    build_css --watch
    
    # Start WASM build in watch mode
    cargo watch -x 'build --target wasm32-unknown-unknown' -s 'wasm-pack build --target web --out-dir pkg'
    
    # Kill CSS watch process when script exits
    trap "kill $CSS_PID 2>/dev/null" EXIT
else
    # Build CSS first
    build_css
    
    # Then build WASM
    wasm-pack build --target web --out-dir pkg
fi
