#!/usr/bin/env pwsh
param (
    [string]$LabId = "k8s-pod-basics"
)

$ns = "lab-$LabId"
Write-Host "Tearing down lab sandbox namespace '$ns'..." -ForegroundColor Yellow
if (Get-Command kubectl -ErrorAction SilentlyContinue) {
    kubectl delete namespace $ns --ignore-not-found=true --wait=false
    Write-Host "Sandbox namespace '$ns' deletion requested." -ForegroundColor Green
}
