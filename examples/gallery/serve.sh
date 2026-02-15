#!/bin/bash
# Serve the WASM app using Python's built-in HTTP server

set -e

# Find the runfiles directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check various runfiles locations - the runfiles dir is named after the sh_binary target
RUNFILES_BASE=""
for candidate in \
    "${SCRIPT_DIR}/gallery.runfiles/_main/examples/gallery" \
    "${BASH_SOURCE[0]%.sh}.runfiles/_main/examples/gallery" \
    "${SCRIPT_DIR}/../gallery.runfiles/_main/examples/gallery" \
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
    echo "Tried:"
    echo "  ${SCRIPT_DIR}/gallery.runfiles/_main/examples/gallery"
    echo "  ${BASH_SOURCE[0]%.sh}.runfiles/_main/examples/gallery"
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

# Copy the built Tailwind CSS
if [[ -f "$RUNFILES_BASE/styles.css" ]]; then
    cp -L "$RUNFILES_BASE/styles.css" "$SERVE_DIR/styles.css"
    echo "Using built Tailwind CSS ($(du -h "$SERVE_DIR/styles.css" | cut -f1))"
else
    echo "Warning: Built CSS not found at $RUNFILES_BASE/styles.css, using CDN fallback"
    # Modify index.html to use CDN
    if [[ "$(uname)" == "Darwin" ]]; then
        sed -i '' 's|<link rel="stylesheet" href="styles.css" />|<script src="https://cdn.tailwindcss.com"></script>|' "$SERVE_DIR/index.html"
    else
        sed -i 's|<link rel="stylesheet" href="styles.css" />|<script src="https://cdn.tailwindcss.com"></script>|' "$SERVE_DIR/index.html"
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
