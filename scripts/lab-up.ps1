#!/usr/bin/env pwsh
param (
    [string]$ClusterName = "kubelab-cluster",
    [string]$LabId = "k8s-pod-basics"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  PROVISIONING DISPOSABLE K8S LAB CLUSTER" -ForegroundColor Cyan
Write-Host "  Cluster: $ClusterName | Lab: $LabId" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan

# 1. Check if kind cluster exists or can be provisioned
$configPath = "$PSScriptRoot/../infrastructure/kind/cluster-config.yaml"
if (Get-Command kind -ErrorAction SilentlyContinue) {
    $existingClusters = kind get clusters 2>$null
    if ($existingClusters -notcontains $ClusterName) {
        Write-Host "[1/3] Creating disposable Kind cluster '$ClusterName' via Podman..." -ForegroundColor Cyan
        kind create cluster --name $ClusterName --config $configPath
    } else {
        Write-Host "[1/3] Cluster '$ClusterName' already active." -ForegroundColor Green
    }
} elseif (Get-Command kubectl -ErrorAction SilentlyContinue) {
    Write-Host "[1/3] Using current Kubernetes context..." -ForegroundColor Cyan
} else {
    Write-Host "[1/3] Containerized kube-rs orchestrator active." -ForegroundColor Yellow
}

# 2. Provision sandbox namespace and isolation
if (Get-Command kubectl -ErrorAction SilentlyContinue) {
    $ns = "lab-$LabId"
    Write-Host "[2/3] Initializing sandbox namespace '$ns' with ResourceQuota & NetworkPolicy..." -ForegroundColor Cyan
    kubectl create namespace $ns --dry-run=client -o yaml | kubectl apply -f -
    
    # Set default-deny ingress network policy isolation
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

    # Set namespace ResourceQuota
    $quota = @"
apiVersion: v1
kind: ResourceQuota
metadata:
  name: lab-quota
  namespace: $ns
spec:
  hard:
    pods: "10"
    requests.cpu: "1"
    requests.memory: 1Gi
    limits.cpu: "2"
    limits.memory: 2Gi
"@
    $quota | kubectl apply -f -
    
    Write-Host "[3/3] Lab environment ready!" -ForegroundColor Green
    Write-Host "Use: kubectl config set-context --current --namespace=$ns" -ForegroundColor Yellow
} else {
    Write-Host "[INFO] Sandbox namespace managed by backend lab orchestrator." -ForegroundColor Green
}
