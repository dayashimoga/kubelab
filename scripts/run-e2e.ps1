#!/usr/bin/env pwsh
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "     KUBELAB PLAYWRIGHT E2E RUNNER      " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$webDir = Resolve-Path "$PSScriptRoot/../apps/web"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    Write-Host "[INFO] Executing Playwright E2E tests inside mcr.microsoft.com/playwright:v1.45.0 container..." -ForegroundColor Yellow
    podman run --rm -v "${webDir}:/work" -w /work -e CI=true mcr.microsoft.com/playwright:v1.45.0 sh -c "npm install -g pnpm && pnpm install && npx playwright test"
} else {
    Write-Host "[INFO] Running Playwright locally..." -ForegroundColor Yellow
    Set-Location $webDir
    npx playwright test
}

if ($LASTEXITCODE -eq 0) {
    Write-Host "[SUCCESS] All Playwright E2E tests passed!" -ForegroundColor Green
} else {
    Write-Host "[WARN] Playwright E2E tests finished with exit code $LASTEXITCODE" -ForegroundColor Yellow
}
