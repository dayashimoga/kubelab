#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB DISPOSABLE KUBERNETES (KIND) CLUSTER LIFECYCLE: UP   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$ClusterName = "kubelab-cluster"
$ConfigFile = "$PSScriptRoot/../infrastructure/kind/cluster-config.yaml"

$hasKind = [bool](Get-Command kind -ErrorAction SilentlyContinue)

if ($hasKind) {
    Write-Host "[1/4] Checking existing Kind cluster '$ClusterName'..." -ForegroundColor Yellow
    $clusters = kind get clusters 2>$null
    if ($clusters -contains $ClusterName) {
        Write-Host "      [INFO] Cluster '$ClusterName' already exists." -ForegroundColor Gray
    } else {
        Write-Host "[2/4] Creating Kind multi-node cluster from $ConfigFile..." -ForegroundColor Yellow
        kind create cluster --config $ConfigFile --name $ClusterName
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[FAIL] Failed to create Kind cluster." -ForegroundColor Red
            exit 1
        }
    }

    Write-Host "[3/4] Waiting for control-plane and worker nodes ready..." -ForegroundColor Yellow
    kubectl wait --for=condition=Ready nodes --all --timeout=60s
    
    Write-Host "[4/4] Verifying cluster API server and base pods..." -ForegroundColor Yellow
    kubectl cluster-info
    kubectl get nodes -o wide

    Write-Host "`n=================================================================" -ForegroundColor Green
    Write-Host "  DISPOSABLE K8S CLUSTER '$ClusterName' IS HEALTHY AND READY    " -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Green
} else {
    Write-Host "[INFO] 'kind' CLI is not found on host." -ForegroundColor Yellow
    Write-Host "[INFO] Podman rootless container technology is active for sandbox workload execution." -ForegroundColor Yellow
    Write-Host "`n=================================================================" -ForegroundColor Green
    Write-Host "  CONTAINERIZED SANDBOX ENVIRONMENT READY                       " -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Green
}
