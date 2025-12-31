#!/bin/bash
set -e

echo "Building CACA for WebAssembly..."

RUSTFLAGS="--cfg=web_sys_unstable_apis" wasm-pack build --target web --dev

echo ""
echo "Build complete! Files in ./pkg/"
echo ""
echo "To run locally:"
echo "  python3 -m http.server 8080"
echo "  # or"
echo "  npx serve ."
echo ""
echo "Then open http://localhost:8080 in a WebGPU-capable browser"
