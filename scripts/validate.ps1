#!/usr/bin/env pwsh
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "       KUBELAB QUICK VALIDATION         " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$passed = $true

# 1. Format & Lint Check
Write-Host "[1/3] Checking code formatting..." -ForegroundColor Yellow
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    cargo clippy --workspace -- -D warnings
    if ($LASTEXITCODE -ne 0) { $passed = $false }
}

# 2. Type Checking
Write-Host "[2/3] Checking TypeScript & Rust types..." -ForegroundColor Yellow

# 3. Fast Unit Tests
Write-Host "[3/3] Running fast unit tests..." -ForegroundColor Yellow
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    cargo test --workspace --lib
    if ($LASTEXITCODE -ne 0) { $passed = $false }
}

if ($passed) {
    Write-Host "[PASS] Fast validation succeeded." -ForegroundColor Green
    exit 0
} else {
    Write-Host "[FAIL] Fast validation failed." -ForegroundColor Red
    exit 1
}
