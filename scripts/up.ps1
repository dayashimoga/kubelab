#!/usr/bin/env pwsh
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "       STARTING KUBELAB PLATFORM        " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# 1. Run Doctor check
& "$PSScriptRoot/doctor.ps1"
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] System prerequisites check failed." -ForegroundColor Red
    exit 1
}

# 2. Start Services via Podman / Docker Compose
Write-Host "[INFO] Starting database, cache, message bus, API, and Web frontend..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile up -d --build
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile up -d --build
} else {
    Write-Host "[ERROR] No container compose tool found." -ForegroundColor Red
    exit 1
}

Write-Host "========================================" -ForegroundColor Green
Write-Host "  KubeLab Platform is running!          " -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Web App:       http://localhost:3000" -ForegroundColor Cyan
Write-Host "  API Docs:      http://localhost:8080/swagger-ui" -ForegroundColor Cyan
Write-Host "  Health Check:  http://localhost:8080/healthz" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Green
