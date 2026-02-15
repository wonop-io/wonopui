#!/bin/bash
# Serve the WASM app using Python's built-in HTTP server

set -e

# Find the runfiles directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUNFILES_DIR="${SCRIPT_DIR}/serve.runfiles/wonopui"

# If not found, try the .runfiles pattern
if [[ ! -d "$RUNFILES_DIR" ]]; then
    RUNFILES_DIR="${BASH_SOURCE[0]}.runfiles/wonopui"
fi

# Create a temporary directory to serve from
SERVE_DIR=$(mktemp -d)
trap "rm -rf $SERVE_DIR" EXIT

echo "Setting up serve directory: $SERVE_DIR"

# Copy the wasm-bindgen output files
if [[ -d "$RUNFILES_DIR/examples/gallery/gallery_wasm" ]]; then
    cp -r "$RUNFILES_DIR/examples/gallery/gallery_wasm/"* "$SERVE_DIR/"
    cp "$RUNFILES_DIR/examples/gallery/index_bazel.html" "$SERVE_DIR/index.html"
else
    # Fallback: running from bazel-bin directly
    if [[ -d "$SCRIPT_DIR/gallery_wasm" ]]; then
        cp -r "$SCRIPT_DIR/gallery_wasm/"* "$SERVE_DIR/"
        cp "$SCRIPT_DIR/index_bazel.html" "$SERVE_DIR/index.html"
    else
        echo "Error: Could not find WASM files"
        echo "Searched in: $RUNFILES_DIR/examples/gallery/gallery_wasm"
        echo "And: $SCRIPT_DIR/gallery_wasm"
        exit 1
    fi
fi

cd "$SERVE_DIR"

echo ""
echo "Files being served:"
ls -la
echo ""
echo "==================================="
echo "Serving WASM app at http://localhost:8080"
echo "Press Ctrl+C to stop"
echo "==================================="
echo ""

python3 -m http.server 8080
