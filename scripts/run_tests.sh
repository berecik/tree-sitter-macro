#!/bin/bash
set -e

echo "Running tests..."
cargo test --all-features

echo "Running examples..."
cargo run --example parse_c --features proc_macros

echo "All tests and examples completed successfully!"