"""CI rules for WonopUI."""

load("@rules_rust//rust:defs.bzl", "rust_clippy")

def rust_clippy_all(name, deps, **kwargs):
    """Creates a clippy target that checks all specified deps.
    
    Args:
        name: Name of the target
        deps: List of Rust targets to check
        **kwargs: Additional arguments passed to rust_clippy
    """
    rust_clippy(
        name = name,
        testonly = True,
        deps = deps,
        **kwargs
    )
