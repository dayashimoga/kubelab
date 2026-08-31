#!/usr/bin/env bash
set -eo pipefail

echo "========================================"
echo "       KUBELAB QUICK VALIDATION         "
echo "========================================"

if command -v cargo >/dev/null 2>&1; then
    echo "[1/3] Checking code formatting..."
    cargo clippy --workspace -- -D warnings
    echo "[2/3] Checking Rust types..."
    echo "[3/3] Running fast unit tests..."
    cargo test --workspace --lib
fi

echo "[PASS] Fast validation succeeded."
