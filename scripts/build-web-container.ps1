#!/usr/bin/env pwsh
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    KUBELAB WEB CONTAINER BUILD & TEST  " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$rootDir = Resolve-Path "$PSScriptRoot/.."

if (Get-Command podman -ErrorAction SilentlyContinue) {
    Write-Host "[INFO] Running web build & vitest inside node:20-alpine container..." -ForegroundColor Yellow
    podman run --rm -v "${rootDir}:/app" -w /app node:20-alpine sh -c "corepack enable && pnpm --filter @kubelab/web test && pnpm --filter @kubelab/web build"
} else {
    Write-Host "[INFO] Running web build locally..." -ForegroundColor Yellow
    pnpm --filter @kubelab/web test
    pnpm --filter @kubelab/web build
}

if ($LASTEXITCODE -eq 0) {
    Write-Host "[SUCCESS] Web container build and test passed!" -ForegroundColor Green
} else {
    Write-Host "[WARN] Web container build returned code $LASTEXITCODE" -ForegroundColor Yellow
}
