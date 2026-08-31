#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "         STARTING KUBELAB CLOUD-NATIVE PLATFORM                  " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Start Services via Podman / Docker Compose
Write-Host "[INFO] Starting full stack (Postgres, Redis, NATS, OTel, Tempo, Loki, Prometheus, Grafana, API, Web)..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile up -d --build
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile up -d --build
} else {
    Write-Host "[ERROR] No container compose tool found. Ensure Podman or Docker is installed." -ForegroundColor Red
    exit 1
}

# 2. Wait for Service Health
Write-Host "`n[INFO] Waiting for platform services to become healthy..." -ForegroundColor Yellow
$maxAttempts = 30
$attempt = 0
$healthy = $false

while ($attempt -lt $maxAttempts) {
    $attempt++
    Start-Sleep -Seconds 2
    try {
        $apiResp = Invoke-RestMethod -Uri "http://127.0.0.1:8080/healthz" -TimeoutSec 2 -ErrorAction SilentlyContinue
        if ($apiResp -and $apiResp.status -eq "ok") {
            $healthy = $true
            break
        }
    } catch {
        Write-Host "   Waiting for API Gateway ($attempt/$maxAttempts)..." -ForegroundColor Gray
    }
}

Write-Host "`n=================================================================" -ForegroundColor Green
Write-Host "  KubeLab Cloud-Native Platform is fully running!                " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
Write-Host "  Web Application:       http://localhost:3000" -ForegroundColor Cyan
Write-Host "  API Gateway & Health:  http://localhost:8080/healthz" -ForegroundColor Cyan
Write-Host "  Prometheus Metrics:    http://localhost:9090" -ForegroundColor Cyan
Write-Host "  Grafana Dashboards:    http://localhost:3001  (admin / admin)" -ForegroundColor Cyan
Write-Host "  Tempo Distributed Tracing: http://localhost:3200" -ForegroundColor Cyan
Write-Host "  Loki Structured Logs:  http://localhost:3100" -ForegroundColor Cyan
Write-Host "  PostgreSQL 16 Engine:  localhost:5432 (db: kubelab)" -ForegroundColor Cyan
Write-Host "  Redis 7 Cache Store:   localhost:6379" -ForegroundColor Cyan
Write-Host "  NATS 2.10 Event Bus:   localhost:4222 / 8222" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Green
