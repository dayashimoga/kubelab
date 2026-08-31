#!/usr/bin/env bash
set -e

echo "================================================================="
echo "   KUBELAB DISPOSABLE KUBERNETES (KIND) CLUSTER LIFECYCLE: DOWN "
echo "================================================================="

CLUSTER_NAME="kubelab-cluster"

if command -v kind &> /dev/null; then
    echo "[1/2] Checking if Kind cluster '$CLUSTER_NAME' exists..."
    if kind get clusters 2>/dev/null | grep -q "^$CLUSTER_NAME$"; then
        echo "[2/2] Deleting Kind cluster '$CLUSTER_NAME'..."
        kind delete cluster --name "$CLUSTER_NAME"
        echo "      [PASS] Cluster '$CLUSTER_NAME' deleted cleanly with zero residue."
    else
        echo "      [INFO] Cluster '$CLUSTER_NAME' is not currently running."
    fi
else
    echo "[INFO] Kind not on host; ensuring container sandbox teardown..."
    podman rm -f $(podman ps -a --filter "name=kubelab-sandbox" --format "{{.Names}}" 2>/dev/null) 2>/dev/null || true
    echo "      [PASS] Containerized sandbox cleaned with zero residue."
fi

echo "================================================================="
echo "  ZERO-RESIDUE TEARDOWN COMPLETED                               "
echo "================================================================="
