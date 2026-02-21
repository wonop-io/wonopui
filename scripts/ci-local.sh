#!/bin/bash
# Run CI checks locally
# Usage: ./scripts/ci-local.sh [check]
# Available checks: all, build, clippy, fmt, test, deny, udeps, machete, wasm

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

run_check() {
    local name="$1"
    local cmd="$2"
    local optional="${3:-false}"
    
    print_header "$name"
    if eval "$cmd"; then
        print_success "$name passed"
        return 0
    else
        if [ "$optional" = "true" ]; then
            print_warning "$name failed (optional)"
            return 0
        else
            print_error "$name failed"
            return 1
        fi
    fi
}

check_build() {
    run_check "Bazel Build" "bazel build //..."
}

check_clippy() {
    run_check "Clippy" "bazel build --config=clippy-strict //crates/..."
}

check_fmt() {
    run_check "Rustfmt" "bazel build --config=rustfmt //crates/..."
}

check_test() {
    run_check "Tests" "bazel test //..."
}

check_wasm() {
    run_check "WASM Build" "bazel build //examples/gallery:gallery_wasm"
}

check_deny() {
    if command -v cargo-deny &> /dev/null; then
        run_check "Cargo Deny" "cargo deny check"
    else
        print_warning "cargo-deny not installed. Install with: cargo install cargo-deny"
    fi
}

check_udeps() {
    if command -v cargo-udeps &> /dev/null; then
        run_check "Unused Dependencies (udeps)" "cargo +nightly udeps --all-targets --all-features" true
    else
        print_warning "cargo-udeps not installed. Install with: cargo install cargo-udeps"
    fi
}

check_machete() {
    if command -v cargo-machete &> /dev/null; then
        run_check "Unused Dependencies (machete)" "cargo machete" true
    else
        print_warning "cargo-machete not installed. Install with: cargo install cargo-machete"
    fi
}

run_all() {
    local failed=0
    
    check_build || failed=1
    check_clippy || failed=1
    check_fmt || failed=1
    check_test || failed=1
    check_wasm || failed=1
    check_deny || failed=1
    check_machete || true  # Optional
    check_udeps || true    # Optional
    
    echo ""
    if [ $failed -eq 0 ]; then
        print_success "All CI checks passed!"
    else
        print_error "Some CI checks failed"
        exit 1
    fi
}

# Main
case "${1:-all}" in
    all)
        run_all
        ;;
    build)
        check_build
        ;;
    clippy)
        check_clippy
        ;;
    fmt|format)
        check_fmt
        ;;
    test)
        check_test
        ;;
    wasm)
        check_wasm
        ;;
    deny)
        check_deny
        ;;
    udeps)
        check_udeps
        ;;
    machete)
        check_machete
        ;;
    help|--help|-h)
        echo "Usage: $0 [check]"
        echo ""
        echo "Available checks:"
        echo "  all      - Run all checks (default)"
        echo "  build    - Bazel build"
        echo "  clippy   - Clippy lints"
        echo "  fmt      - Rustfmt formatting"
        echo "  test     - Run tests"
        echo "  wasm     - Build WASM target"
        echo "  deny     - Cargo deny (licenses/advisories)"
        echo "  udeps    - Unused dependencies (nightly)"
        echo "  machete  - Fast unused deps check"
        ;;
    *)
        print_error "Unknown check: $1"
        echo "Run '$0 help' for available checks"
        exit 1
        ;;
esac
