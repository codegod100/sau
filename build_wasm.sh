#!/bin/bash
# Build the project using wasm-pack and output to the pkg directory

set -e

wasm-pack build --target web
