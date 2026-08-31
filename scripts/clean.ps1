#!/usr/bin/env pwsh
Write-Host "Cleaning up KubeLab containers, volumes, and temporary caches..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile down -v --remove-orphans
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile down -v --remove-orphans
}

# Clean temp directories
$pathsToClean = @(
    "$PSScriptRoot/../target",
    "$PSScriptRoot/../node_modules",
    "$PSScriptRoot/../apps/web/.next",
    "$PSScriptRoot/../apps/web/node_modules",
    "$PSScriptRoot/../coverage"
)

foreach ($p in $pathsToClean) {
    if (Test-Path $p) {
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $p
    }
}

Write-Host "KubeLab workspace cleaned." -ForegroundColor Green
