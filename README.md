# Wonop UI - Tailwind components for YEW (BETA)

Wonop UI is a parameterized UI framework that leverages Tailwind CSS for use with the Yew framework in Rust. It provides a set of customizable components and utilities to streamline the development of web applications using Yew.

You can find more information in the [documentation](https://docs.wonopui.com/).

## Features

- Seamless integration with Yew framework
- Utilizes Tailwind CSS for rapid and flexible styling
- Parameterized components for easy customization
- Responsive design out of the box
- Feature flags for each component to minimize bundle size
- Bazel build system for fast, reproducible builds

## Project Structure

The project is organized as a Rust workspace with individual crates for each component:

```
crates/
├── wonopui/              # Main umbrella crate with feature-gated re-exports
├── wonopui-core/         # Core utilities and types
├── wonopui-button/       # Button component
├── wonopui-card/         # Card component
├── wonopui-alert/        # Alert component
└── ...                   # 60+ more component crates
```

## Installation

To use Wonop UI in your Yew project, add the following to your `Cargo.toml`:

```toml
[dependencies]
wonopui = { version = "0.1.0", features = ["button", "card", "alert"] }
```

Or enable all components:

```toml
[dependencies]
wonopui = { version = "0.1.0", features = ["everything"] }
```

## Building with Bazel

This project uses [Bazel](https://bazel.build/) as its primary build system for fast, reproducible builds.

### Prerequisites

Install Bazelisk (recommended) which automatically downloads the correct Bazel version:

```bash
# macOS
brew install bazelisk

# Or install directly
# See: https://github.com/bazelbuild/bazelisk
```

### Building

```bash
# Build all crates
bazel build //...

# Build a specific crate
bazel build //crates/wonopui-button:wonopui-button

# Build the main wonopui library
bazel build //crates/wonopui:wonopui

# Build with release optimizations
bazel build --config=release //...
```

### Running the Gallery Demo

The gallery demonstrates all available components:

```bash
# Build and serve the WASM gallery application
bazel run //examples/gallery

# This starts a local server at http://localhost:8080
```

### Running Tests

```bash
# Run all tests
bazel test //...

# Run tests for a specific crate
bazel test //crates/wonopui-core:wonopui-core_test

# Run tests with verbose output
bazel test --test_output=all //...
```

### Code Quality Checks

#### Clippy (Rust Linter)

```bash
# Run clippy on all crates
bazel build --config=clippy //crates/...

# Run clippy with strict mode (fails on warnings)
bazel build --config=clippy-strict //crates/...
```

#### Rustfmt (Code Formatting)

```bash
# Check formatting on all crates
bazel build --config=rustfmt //crates/...
```

#### All CI Checks

```bash
# Run combined CI checks (clippy + rustfmt)
bazel build --config=ci //crates/...
```

### Cargo Deny (License & Security Auditing)

For license compliance and security vulnerability checks:

```bash
# Install cargo-deny
cargo install cargo-deny

# Run all checks
cargo deny check

# Check specific categories
cargo deny check licenses
cargo deny check advisories
cargo deny check bans
cargo deny check sources
```

### Local CI Script

A convenience script is provided to run all CI checks locally:

```bash
# Run all checks
./scripts/ci-local.sh

# Run specific checks
./scripts/ci-local.sh clippy
./scripts/ci-local.sh fmt
./scripts/ci-local.sh test
./scripts/ci-local.sh deny

# See all available checks
./scripts/ci-local.sh help
```

## Building with Cargo

If you prefer using Cargo directly:

```bash
# Install the wasm32 target
rustup target add wasm32-unknown-unknown

# Build for wasm32 target
cargo build --target wasm32-unknown-unknown --release --features everything

# Run tests
cargo test --all-features
```

After building with Cargo, you'll need to process the resulting Wasm binary with `wasm-bindgen` to generate the JavaScript bindings:

```bash
wasm-bindgen --target web --out-dir ./dist ./target/wasm32-unknown-unknown/release/your_app.wasm
```

## Development

### Adding a New Component

1. Create a new crate in `crates/wonopui-<component>/`
2. Add the crate to the workspace in the root `Cargo.toml`
3. Create a `BUILD.bazel` file for the crate:

```python
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

exports_files(["Cargo.toml"])

rust_library(
    name = "wonopui-mycomponent",
    srcs = glob(["src/**/*.rs"]),
    crate_name = "wonopui_mycomponent",
    edition = "2021",
    deps = [
        "//crates/wonopui-core:wonopui-core",
        "@crates//:yew",
    ],
)

rust_test(
    name = "wonopui-mycomponent_test",
    crate = ":wonopui-mycomponent",
)

filegroup(
    name = "srcs",
    srcs = glob(["src/**/*.rs"]),
)
```

4. Add the crate to `MODULE.bazel` manifests list
5. Add feature flag and re-export in `crates/wonopui/Cargo.toml`

### Configuration Files

- `.bazelrc` - Bazel build configuration
- `clippy.toml` - Clippy linter configuration
- `deny.toml` - Cargo deny configuration for license/security checks
- `MODULE.bazel` - Bazel module dependencies

## CI/CD

GitHub Actions automatically runs:
- Bazel build and tests
- Clippy linting
- Rustfmt formatting checks
- Cargo deny (license/security auditing)
- WASM build verification

See `.github/workflows/ci.yaml` for the full CI configuration.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
