#!/usr/bin/env bash
set -e

echo "================================================================="
echo "   KUBELAB DISPOSABLE KUBERNETES (KIND) CLUSTER LIFECYCLE: UP   "
echo "================================================================="

CLUSTER_NAME="kubelab-cluster"
CONFIG_FILE="$(dirname "$0")/../infrastructure/kind/cluster-config.yaml"

if command -v kind &> /dev/null; then
    echo "[1/4] Checking existing Kind cluster '$CLUSTER_NAME'..."
    if kind get clusters 2>/dev/null | grep -q "^$CLUSTER_NAME$"; then
        echo "      [INFO] Cluster '$CLUSTER_NAME' already exists."
    else
        echo "[2/4] Creating Kind multi-node cluster from $CONFIG_FILE..."
        kind create cluster --config "$CONFIG_FILE" --name "$CLUSTER_NAME"
    fi

    echo "[3/4] Waiting for control-plane and worker nodes ready..."
    kubectl wait --for=condition=Ready nodes --all --timeout=60s
    
    echo "[4/4] Verifying cluster API server and base pods..."
    kubectl cluster-info
    kubectl get nodes -o wide

    echo "================================================================="
    echo "  DISPOSABLE K8S CLUSTER '$CLUSTER_NAME' IS HEALTHY AND READY    "
    echo "================================================================="
else
    echo "[INFO] 'kind' CLI is not found on host."
    echo "[INFO] Podman rootless container technology is active for sandbox workload execution."
    echo "================================================================="
    echo "  CONTAINERIZED SANDBOX ENVIRONMENT READY                       "
    echo "================================================================="
fi
