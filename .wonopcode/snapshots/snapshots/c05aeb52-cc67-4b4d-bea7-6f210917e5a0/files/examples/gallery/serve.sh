#!/bin/bash
# Serve the WASM app using Python's built-in HTTP server

set -e

# Find the runfiles directory
RUNFILES_DIR="${BASH_SOURCE[0]}.runfiles"
if [[ ! -d "$RUNFILES_DIR" ]]; then
    RUNFILES_DIR="$0.runfiles"
fi

# Create a temporary directory to serve from
SERVE_DIR=$(mktemp -d)
trap "rm -rf $SERVE_DIR" EXIT

# Copy the wasm-bindgen output files
cp -r "$RUNFILES_DIR"/wonopui/examples/gallery/gallery_wasm_bindgen/* "$SERVE_DIR/" 2>/dev/null || true
cp "$RUNFILES_DIR"/wonopui/examples/gallery/index_bazel.html "$SERVE_DIR/index.html" 2>/dev/null || true

# If files aren't in runfiles, try the current directory
if [[ ! -f "$SERVE_DIR/index.html" ]]; then
    # Running from bazel-bin directly
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    cp -r "$SCRIPT_DIR"/gallery_wasm_bindgen/* "$SERVE_DIR/" 2>/dev/null || true
    cp "$SCRIPT_DIR"/index_bazel.html "$SERVE_DIR/index.html" 2>/dev/null || true
fi

cd "$SERVE_DIR"

echo "Serving WASM app at http://localhost:8080"
echo "Press Ctrl+C to stop"
python3 -m http.server 8080
