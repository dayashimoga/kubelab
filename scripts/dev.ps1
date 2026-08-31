#!/usr/bin/env pwsh
Write-Host "Starting KubeLab in Development Mode..." -ForegroundColor Cyan

# 1. Start background infra (Postgres, Redis, NATS)
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"
if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile up -d postgres redis nats
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile up -d postgres redis nats
}

Write-Host "Infra services online. Starting dev servers..." -ForegroundColor Green

# 2. Check if local cargo / pnpm can run, or fallback to container runner
if ((Get-Command cargo -ErrorAction SilentlyContinue) -and (Get-Command pnpm, npm -ErrorAction SilentlyContinue)) {
    Write-Host "Running local hot-reload dev servers..." -ForegroundColor Cyan
    # Run API and Web
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cargo run -p kubelab-api"
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd apps/web; pnpm dev"
} else {
    Write-Host "Running containerized dev environment..." -ForegroundColor Yellow
    if (Get-Command podman -ErrorAction SilentlyContinue) {
        podman compose -f $composeFile up
    } else {
        docker-compose -f $composeFile up
    }
}
