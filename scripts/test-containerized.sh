#!/usr/bin/env bash
set -eo pipefail

echo "================================================================="
echo "   KUBELAB ZERO-HOST-INSTALL CONTAINERIZED TEST SUITE           "
echo "================================================================="

# Ensure toolchain image exists
echo "[1/4] Ensuring kubelab-toolchain container image is available..."
podman build -f infrastructure/containers/Containerfile.toolchain -t kubelab-toolchain .

# 1. Run Cargo Tests inside container
echo "[2/4] Executing Rust workspace tests in isolated Linux container..."
podman run --rm -v "$(pwd):/workspace" -w /workspace kubelab-toolchain cargo test --workspace

# 2. Run Live Backing Services Integration Tests in container network
echo "[3/4] Running live integration tests with containerized PostgreSQL/Redis/NATS..."
podman run --rm --network host -v "$(pwd):/workspace" -w /workspace \
  -e DATABASE_URL="postgres://kubelab:kubelab_secret_password@127.0.0.1:5432/kubelab" \
  -e REDIS_URL="redis://127.0.0.1:6379" \
  -e NATS_URL="nats://127.0.0.1:4222" \
  kubelab-toolchain cargo test -- --ignored

# 3. Lab Schema Validation
echo "[4/4] Validating all 145 lab declarative YAML definitions inside container..."
podman run --rm -v "$(pwd):/workspace" -w /workspace kubelab-toolchain cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path labs/

echo ""
echo "================================================================="
echo "  ALL CONTAINERIZED TESTS & SUITES PASSED SUCCESSFULLY!          "
echo "================================================================="
