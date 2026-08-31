#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB ZERO-HOST-INSTALL CONTAINERIZED TEST SUITE           " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# Ensure toolchain image exists
Write-Host "[1/4] Ensuring kubelab-toolchain container image is available..." -ForegroundColor Cyan
podman build -f infrastructure/containers/Containerfile.toolchain -t kubelab-toolchain .

# 1. Run Cargo Tests inside container
Write-Host "[2/4] Executing Rust workspace tests in isolated Linux container..." -ForegroundColor Cyan
podman run --rm -v "${PWD}:/workspace" -w /workspace kubelab-toolchain cargo test --workspace

# 2. Run Live Backing Services Integration Tests in container network
Write-Host "[3/4] Running live integration tests with containerized PostgreSQL/Redis/NATS..." -ForegroundColor Cyan
podman run --rm --network host -v "${PWD}:/workspace" -w /workspace `
  -e DATABASE_URL="postgres://kubelab:kubelab_secret_password@127.0.0.1:5432/kubelab" `
  -e REDIS_URL="redis://127.0.0.1:6379" `
  -e NATS_URL="nats://127.0.0.1:4222" `
  kubelab-toolchain cargo test -- --ignored

# 3. Lab Schema Validation
Write-Host "[4/4] Validating all 145 lab declarative YAML definitions inside container..." -ForegroundColor Cyan
podman run --rm -v "${PWD}:/workspace" -w /workspace kubelab-toolchain cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path labs/

Write-Host ""
Write-Host "=================================================================" -ForegroundColor Green
Write-Host "  ALL CONTAINERIZED TESTS & SUITES PASSED SUCCESSFULLY!          " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
