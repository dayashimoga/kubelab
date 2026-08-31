#!/usr/bin/env bash
set -e

echo "================================================================="
echo "   KUBELAB CODE COVERAGE HARNESS (CARGO TARPAULIN)               "
echo "================================================================="

THRESHOLD=90
REPORT_DIR="$(dirname "$0")/../target/coverage"
mkdir -p "$REPORT_DIR"

if command -v cargo-tarpaulin &> /dev/null; then
    echo "[INFO] Running cargo-tarpaulin on host..."
    cargo tarpaulin --workspace --timeout 120 --out Html --out Lcov --output-dir "$REPORT_DIR" --fail-under "$THRESHOLD"
else
    echo "[INFO] cargo-tarpaulin not on host; executing inside Podman container..."
    podman run --rm \
      -v "$(dirname "$0")/..:/workspace:z" \
      -w /workspace \
      docker.io/xd009642/tarpaulin:latest \
      cargo tarpaulin --workspace --timeout 180 --out Html --out Lcov --output-dir target/coverage --fail-under "$THRESHOLD"
fi

echo "================================================================="
echo "  COVERAGE CERTIFICATION COMPLETED                               "
echo "================================================================="
