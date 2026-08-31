#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB CODE COVERAGE HARNESS (CARGO TARPAULIN)               " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$Threshold = 90
$ReportDir = "$PSScriptRoot/../target/coverage"
New-Item -ItemType Directory -Force -Path $ReportDir | Out-Null

$HasHostTarpaulin = [bool](Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue)

if ($HasHostTarpaulin) {
    Write-Host "[INFO] Running cargo-tarpaulin directly on host..." -ForegroundColor Yellow
    cargo tarpaulin --workspace --timeout 120 --out Html --out Lcov --output-dir $ReportDir --fail-under $Threshold
    $exitCode = $LASTEXITCODE
} else {
    Write-Host "[INFO] cargo-tarpaulin not on host; executing inside Podman container..." -ForegroundColor Yellow
    
    # Run in Podman container with Rust toolchain and cargo-tarpaulin
    podman run --rm `
      -v "${PSScriptRoot}/..:/workspace:z" `
      -w /workspace `
      docker.io/xd009642/tarpaulin:latest `
      cargo tarpaulin --workspace --timeout 180 --out Html --out Lcov --output-dir target/coverage --fail-under $Threshold 2>&1
    
    $exitCode = $LASTEXITCODE
}

if ($exitCode -eq 0) {
    Write-Host "`n=================================================================" -ForegroundColor Green
    Write-Host "  COVERAGE CERTIFICATION PASSED (>= $Threshold% THRESHOLD MET) " -ForegroundColor Green
    Write-Host "  Report saved to: $ReportDir/tarpaulin-report.html              " -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n[WARN] Target coverage threshold ($Threshold%) check completed (Exit code: $exitCode)." -ForegroundColor Yellow
    exit $exitCode
}
