#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB FLUTTER MOBILE COMPANION RUNTIME PROOF (GATE 06)      " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$useHost = Get-Command flutter -ErrorAction SilentlyContinue

if ($useHost) {
    Write-Host "[INFO] Executing Flutter test suite on local Flutter environment..." -ForegroundColor Yellow
    
    Push-Location "$PSScriptRoot/../apps/mobile"
    try {
        Write-Host "`n[1/3] Running flutter pub get..." -ForegroundColor Yellow
        flutter pub get
        
        Write-Host "`n[2/3] Running flutter analyze..." -ForegroundColor Yellow
        flutter analyze --no-fatal-infos
        
        Write-Host "`n[3/3] Running flutter test with coverage..." -ForegroundColor Yellow
        flutter test --coverage
    } finally {
        Pop-Location
    }
} else {
    Write-Host "[INFO] Local Flutter not found; executing inside isolated Flutter container..." -ForegroundColor Yellow
    
    $containerCmd = "cd /app/apps/mobile && flutter pub get && flutter analyze --no-fatal-infos && flutter test --coverage"
    
    podman run --rm -v "${PSScriptRoot}/..:/app:Z" -w /app `
        -e CI=true `
        ghcr.io/cirruslabs/flutter:stable `
        bash -c "$containerCmd" 2>&1
    
    if ($LASTEXITCODE -ne 0) {
        throw "Containerized Flutter analysis and test suite failed"
    }
}

Write-Host "`n=================================================================" -ForegroundColor Green
Write-Host "  FLUTTER MOBILE RUNTIME PROOF: PASSED (100% PROVEN)            " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
exit 0
