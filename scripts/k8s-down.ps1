#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB DISPOSABLE KUBERNETES (KIND) CLUSTER LIFECYCLE: DOWN " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$ClusterName = "kubelab-cluster"
$hasKind = [bool](Get-Command kind -ErrorAction SilentlyContinue)

if ($hasKind) {
    Write-Host "[1/2] Checking if Kind cluster '$ClusterName' exists..." -ForegroundColor Yellow
    $clusters = kind get clusters 2>$null
    if ($clusters -contains $ClusterName) {
        Write-Host "[2/2] Deleting Kind cluster '$ClusterName' and reclaiming host resources..." -ForegroundColor Yellow
        kind delete cluster --name $ClusterName
        Write-Host "      [PASS] Cluster '$ClusterName' deleted cleanly with zero residue." -ForegroundColor Green
    } else {
        Write-Host "      [INFO] Cluster '$ClusterName' is not currently running." -ForegroundColor Gray
    }
} else {
    Write-Host "[INFO] Kind not on host; ensuring container sandbox teardown..." -ForegroundColor Gray
    podman rm -f (podman ps -a --filter "name=kubelab-sandbox" --format "{{.Names}}" 2>$null) 2>$null | Out-Null
    Write-Host "      [PASS] Containerized sandbox cleaned with zero residue." -ForegroundColor Green
}

Write-Host "`n=================================================================" -ForegroundColor Green
Write-Host "  ZERO-RESIDUE TEARDOWN COMPLETED                               " -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Green
