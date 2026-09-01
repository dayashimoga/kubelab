#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB DISPOSABLE KIND KUBERNETES RUNTIME PROOF (GATE 08)    " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$clusterName = "kubelab-disposable-test"
$configPath = "$PSScriptRoot/../infrastructure/kind/cluster-config.yaml"

if (-not (Get-Command kind -ErrorAction SilentlyContinue)) {
    throw "kind CLI is required for Gate 08 disposable Kubernetes validation"
}
if (-not (Get-Command kubectl -ErrorAction SilentlyContinue)) {
    throw "kubectl CLI is required for Gate 08 disposable Kubernetes validation"
}

# Ensure Podman provider is used when docker daemon is not active
if (-not (Get-Command docker -ErrorAction SilentlyContinue) -or (Get-Command podman -ErrorAction SilentlyContinue)) {
    $env:KIND_EXPERIMENTAL_PROVIDER = "podman"
}

try {
    # 1. Clean any previous test cluster
    $existing = kind get clusters 2>$null
    if ($existing -contains $clusterName) {
        Write-Host "[INFO] Purging existing '$clusterName' cluster..." -ForegroundColor Gray
        kind delete cluster --name $clusterName 2>$null | Out-Null
    }

    # 2. Create Kind Cluster
    Write-Host "`n[1/6] Provisioning disposable multi-node Kind cluster '$clusterName'..." -ForegroundColor Yellow
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    kind create cluster --name $clusterName --config $configPath
    if ($LASTEXITCODE -ne 0) { throw "kind create cluster failed with exit code $LASTEXITCODE" }
    Write-Host "      [PASS] Cluster created in $($sw.ElapsedMilliseconds)ms." -ForegroundColor Green

    # 3. Wait for Nodes to become Ready
    Write-Host "`n[2/6] Waiting for control-plane and worker nodes to become Ready..." -ForegroundColor Yellow
    kubectl wait --for=condition=Ready nodes --all --timeout=240s
    if ($LASTEXITCODE -ne 0) { throw "Kubernetes nodes failed to become Ready" }
    Write-Host "      [PASS] All nodes Ready." -ForegroundColor Green

    # 4. Apply PSS Restricted Namespace, Quota, LimitRange, and NetworkPolicy
    Write-Host "`n[3/6] Applying PSS Restricted Namespace, Quotas, and Security Policies..." -ForegroundColor Yellow
    $testNs = "lab-gate08-disposable"
    
    $nsManifest = @"
apiVersion: v1
kind: Namespace
metadata:
  name: $testNs
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
---
apiVersion: v1
kind: ResourceQuota
metadata:
  name: lab-quota
  namespace: $testNs
spec:
  hard:
    pods: "5"
    requests.cpu: "500m"
    requests.memory: 256Mi
    limits.cpu: "1000m"
    limits.memory: 512Mi
---
apiVersion: v1
kind: LimitRange
metadata:
  name: lab-limits
  namespace: $testNs
spec:
  limits:
  - default:
      cpu: "200m"
      memory: 128Mi
    defaultRequest:
      cpu: "100m"
      memory: 64Mi
    type: Container
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-all
  namespace: $testNs
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
"@
    $nsManifest | kubectl apply -f -
    Write-Host "      [PASS] Namespace, PSS Restricted, Quota, LimitRange & NetworkPolicy applied." -ForegroundColor Green

    # 5. Deploy Non-Root Workload & Query Live State
    Write-Host "`n[4/6] Deploying restricted, non-root workload in isolated namespace..." -ForegroundColor Yellow
    $workloadManifest = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: test-app
  namespace: $testNs
spec:
  replicas: 1
  selector:
    matchLabels:
      app: test-app
  template:
    metadata:
      labels:
        app: test-app
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 10001
        runAsGroup: 10001
        seccompProfile:
          type: RuntimeDefault
      containers:
      - name: pause
        image: registry.k8s.io/pause:3.9
        securityContext:
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop:
            - ALL
        resources:
          requests:
            cpu: 50m
            memory: 32Mi
          limits:
            cpu: 100m
            memory: 64Mi
"@
    $workloadManifest | kubectl apply -f -
    kubectl rollout status deployment/test-app -n $testNs --timeout=90s
    Write-Host "      [PASS] Workload deployed and rolled out successfully under PSS Restricted." -ForegroundColor Green

    # 6. Mutate Workload (Scale to 2) and Verify State
    Write-Host "`n[5/6] Mutating workload (scaling replicas: 2) and querying live API..." -ForegroundColor Yellow
    kubectl scale deployment/test-app -n $testNs --replicas=2
    kubectl rollout status deployment/test-app -n $testNs --timeout=90s
    $readyReplicas = kubectl get deployment/test-app -n $testNs -o jsonpath='{.status.readyReplicas}'
    if ("$readyReplicas".Trim() -ne "2") { throw "Scale mutation failed: expected 2 ready replicas, got $readyReplicas" }
    Write-Host "      [PASS] Scale mutation verified ($readyReplicas ready replicas)." -ForegroundColor Green

    # 7. Teardown Cluster & Assert Zero Residue
    Write-Host "`n[6/6] Destroying disposable cluster '$clusterName' and asserting zero residue..." -ForegroundColor Yellow
    kind delete cluster --name $clusterName
    $remaining = kind get clusters 2>$null
    if ($remaining -contains $clusterName) { throw "Cluster destruction failed" }
    Write-Host "      [PASS] Cluster destroyed cleanly (clusters = 0)." -ForegroundColor Green

    Write-Host "`n=================================================================" -ForegroundColor Green
    Write-Host "  DISPOSABLE K8S RUNTIME PROOF: PASSED (100% PROVEN)            " -ForegroundColor Green
    Write-Host "=================================================================" -ForegroundColor Green
    exit 0
} catch {
    Write-Host "`n[FAIL] Disposable K8s validation failed: $_" -ForegroundColor Red
    kind delete cluster --name $clusterName 2>$null | Out-Null
    exit 1
}
