#!/usr/bin/env bash
set -eo pipefail

LAB_ID="${1:-k8s-pod-basics}"
NS="lab-$LAB_ID"

echo "Tearing down lab sandbox namespace '$NS'..."
if command -v kubectl >/dev/null 2>&1; then
    kubectl delete namespace "$NS" --ignore-not-found=true --wait=false
    echo "Sandbox namespace '$NS' deletion requested."
fi
