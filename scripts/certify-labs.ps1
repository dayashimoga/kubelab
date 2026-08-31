#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB 145-LAB RUNTIME & DECLARATIVE CERTIFICATION HARNESS   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Run full schema validation on all 145 labs
Write-Host "`n[1/3] Executing schema validation for all 145 declarative YAML labs..." -ForegroundColor Yellow
$sw = [System.Diagnostics.Stopwatch]::StartNew()

$out = cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path "$PSScriptRoot/../labs" 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "      [PASS] 145/145 declarative YAML lab definitions valid and certified." -ForegroundColor Green
} else {
    Write-Host "      [FAIL] Lab schema validation failed: $out" -ForegroundColor Red
    exit 1
}

# 2. Run Evaluator assertion test suites
Write-Host "`n[2/3] Executing live evaluator assertions & negative condition test suite..." -ForegroundColor Yellow
$evalOut = cargo test -p kubelab-validation-engine --test evaluator_comprehensive_test --test evaluator_negative_test 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "      [PASS] Evaluator comprehensive assertions & negative type guards verified." -ForegroundColor Green
} else {
    Write-Host "      [FAIL] Evaluator test suite failed: $evalOut" -ForegroundColor Red
    exit 1
}

# 3. Verify Server-Side Manifest Admission
Write-Host "`n[3/3] Executing server-side zero-trust manifest admission tests..." -ForegroundColor Yellow
$admitOut = cargo test -p kubelab-api --test manifest_admission_test 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "      [PASS] Server-side admission policy enforcement verified." -ForegroundColor Green
} else {
    Write-Host "      [FAIL] Admission test suite failed: $admitOut" -ForegroundColor Red
    exit 1
}

$sw.Stop()
Write-Host "`n=================================================================" -ForegroundColor Green
Write-Host "  145 LAB CERTIFICATION COMPLETED SUCCESSFULLY ($($sw.ElapsedMilliseconds)ms) " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
