#!/usr/bin/env pwsh
Write-Host "Stopping KubeLab services..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile down
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile down
}
Write-Host "KubeLab services stopped." -ForegroundColor Green
