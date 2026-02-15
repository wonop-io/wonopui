#!/bin/bash
# Serve the WASM app using Python's built-in HTTP server

set -e

# Find the runfiles directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check various runfiles locations
RUNFILES_BASE=""
for candidate in \
    "${SCRIPT_DIR}/serve.runfiles/_main/examples/gallery" \
    "${BASH_SOURCE[0]}.runfiles/_main/examples/gallery" \
    "${RUNFILES_DIR:-}_main/examples/gallery" \
    "$SCRIPT_DIR"
do
    if [[ -d "$candidate/gallery_wasm" ]]; then
        RUNFILES_BASE="$candidate"
        break
    fi
done

if [[ -z "$RUNFILES_BASE" ]]; then
    echo "Error: Could not find WASM files in runfiles"
    echo "Script dir: $SCRIPT_DIR"
    exit 1
fi

# Create a temporary directory to serve from
SERVE_DIR=$(mktemp -d)
trap "rm -rf $SERVE_DIR" EXIT

echo "Setting up serve directory: $SERVE_DIR"
echo "Source: $RUNFILES_BASE"

# Copy the wasm-bindgen output files (dereference symlinks)
cp -rL "$RUNFILES_BASE/gallery_wasm/"* "$SERVE_DIR/"
cp -L "$RUNFILES_BASE/index_bazel.html" "$SERVE_DIR/index.html"

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
