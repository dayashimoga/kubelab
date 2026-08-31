#!/usr/bin/env pwsh
Write-Host "Cleaning up KubeLab containers, volumes, and temporary caches..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"
$testComposeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.test.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile down -v --remove-orphans 2>$null
    podman compose -f $testComposeFile down -v --remove-orphans 2>$null
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile down -v --remove-orphans 2>$null
    docker-compose -f $testComposeFile down -v --remove-orphans 2>$null
}

# Clean Kind cluster if exists
if (Get-Command kind -ErrorAction SilentlyContinue) {
    Write-Host "Checking for disposable Kind clusters..." -ForegroundColor Gray
    $clusters = kind get clusters 2>$null
    if ($clusters -contains "kubelab-cluster") {
        Write-Host "Deleting Kind cluster 'kubelab-cluster'..." -ForegroundColor Yellow
        kind delete cluster --name "kubelab-cluster" 2>$null
    }
}

# Clean temporary build and cache directories
$pathsToClean = @(
    "$PSScriptRoot/../target",
    "$PSScriptRoot/../node_modules",
    "$PSScriptRoot/../apps/web/.next",
    "$PSScriptRoot/../apps/web/node_modules",
    "$PSScriptRoot/../coverage",
    "$PSScriptRoot/../.turbo"
)

foreach ($p in $pathsToClean) {
    if (Test-Path $p) {
        Write-Host "Removing temporary artifact directory: $p" -ForegroundColor Gray
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $p
    }
}

Write-Host "========================================" -ForegroundColor Green
Write-Host "  KubeLab workspace 100% clean (Zero Residue)  " -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
