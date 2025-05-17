#!/usr/bin/env bash
set -euo pipefail

# Install and run cargo-audit
if ! command -v cargo-audit &>/dev/null; then
    cargo install cargo-audit --locked
fi

# Install and run cargo-deny
if ! command -v cargo-deny &>/dev/null; then
    cargo install cargo-deny --locked
fi

# Build
cargo build --verbose

# Check formatting
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all --verbose

# Audit dependencies for security vulnerabilities
cargo audit

# Check dependency policy and minimalism
cargo deny check
