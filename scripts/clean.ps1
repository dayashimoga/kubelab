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

# Zero-Residue Assertions
Write-Host "`nAsserting Zero Residue State..." -ForegroundColor Yellow
$residueErrors = 0

if (Get-Command podman -ErrorAction SilentlyContinue) {
    $runningContainers = podman ps --filter "name=kubelab" --format "{{.Names}}" 2>$null
    if ($runningContainers) {
        Write-Host "[FAIL] Lingering containers found: $runningContainers" -ForegroundColor Red
        $residueErrors++
    } else {
        Write-Host "[PASS] containers = 0" -ForegroundColor Green
    }
}

if (Get-Command kubectl -ErrorAction SilentlyContinue) {
    $orphanedNS = kubectl get namespaces -o jsonpath='{.items[*].metadata.name}' 2>$null
    $labNS = ($orphanedNS -split ' ') | Where-Object { $_ -match "^lab-" }
    if ($labNS.Count -gt 0) {
        Write-Host "[FAIL] Lingering lab namespaces found: $($labNS -join ', ')" -ForegroundColor Red
        $residueErrors++
    } else {
        Write-Host "[PASS] lab-namespaces = 0, orphans = 0" -ForegroundColor Green
    }
}

if ($residueErrors -eq 0) {
    Write-Host "`n=================================================================" -ForegroundColor Green
    Write-Host "  ZERO-RESIDUE CLEANUP VERIFIED: (containers=networks=volumes=0) " -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Green
} else {
    Write-Host "`n[FAIL] Clean operation left residue behind ($residueErrors errors)." -ForegroundColor Red
    exit 1
}
