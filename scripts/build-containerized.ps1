#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB ZERO-HOST-INSTALL CONTAINERIZED BUILD PIPELINE        " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

# 1. Build Toolchain
Write-Host "`n[1/3] Building kubelab-toolchain container..." -ForegroundColor Cyan
podman build -f infrastructure/containers/Containerfile.toolchain -t kubelab-toolchain .

# 2. Build Web Application
Write-Host "`n[2/3] Building production Next.js web application container..." -ForegroundColor Cyan
podman build -f infrastructure/containers/Containerfile.web -t kubelab-web .

# 3. Build Backend API Server
Write-Host "`n[3/3] Building production Rust API Gateway container..." -ForegroundColor Cyan
podman build -f infrastructure/containers/Containerfile.api -t kubelab-api .

Write-Host ""
Write-Host "=================================================================" -ForegroundColor Green
Write-Host "  ALL PRODUCTION CONTAINERS BUILT SUCCESSFULLY!                  " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
