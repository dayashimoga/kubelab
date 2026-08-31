#!/usr/bin/env pwsh
param (
    [string]$ClusterName = "kubelab-cluster",
    [string]$LabId = "k8s-pod-basics"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  PROVISIONING DISPOSABLE K8S LAB CLUSTER" -ForegroundColor Cyan
Write-Host "  Cluster: $ClusterName | Lab: $LabId" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan

# Create dedicated namespace in local cluster or kind
Write-Host "[1/3] Ensuring cluster is accessible..." -ForegroundColor Cyan
if (Get-Command kubectl -ErrorAction SilentlyContinue) {
    $ns = "lab-$LabId"
    Write-Host "[2/3] Initializing sandbox namespace '$ns'..." -ForegroundColor Cyan
    kubectl create namespace $ns --dry-run=client -o yaml | kubectl apply -f -
    
    # Set network policy isolation
    $netPol = @"
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: lab-isolate
  namespace: $ns
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector: {}
"@
    $netPol | kubectl apply -f -
    
    Write-Host "[3/3] Lab environment ready!" -ForegroundColor Green
    Write-Host "Use: kubectl config set-context --current --namespace=$ns" -ForegroundColor Yellow
} else {
    Write-Host "[INFO] Running inside containerized orchestrator." -ForegroundColor Yellow
}
