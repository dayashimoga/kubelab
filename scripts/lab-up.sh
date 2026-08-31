#!/usr/bin/env bash
set -eo pipefail

CLUSTER_NAME="${1:-kubelab-cluster}"
LAB_ID="${2:-k8s-pod-basics}"

echo "========================================"
echo "  PROVISIONING DISPOSABLE K8S LAB CLUSTER"
echo "  Cluster: $CLUSTER_NAME | Lab: $LAB_ID"
echo "========================================"

if command -v kubectl >/dev/null 2>&1; then
    NS="lab-$LAB_ID"
    echo "[INFO] Creating sandbox namespace '$NS'..."
    kubectl create namespace "$NS" --dry-run=client -o yaml | kubectl apply -f -
    echo "[OK] Lab environment ready."
else
    echo "[INFO] Running inside containerized orchestrator."
fi
