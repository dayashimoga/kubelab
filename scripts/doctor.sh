#!/usr/bin/env bash
set -eo pipefail

echo "========================================"
echo "       KUBELAB SYSTEM DOCTOR           "
echo "========================================"

ALL_OK=true

# Check Podman or Docker
if command -v podman >/dev/null 2>&1; then
    echo "[OK] Podman is installed: $(podman --version)"
elif command -v docker >/dev/null 2>&1; then
    echo "[OK] Docker is installed: $(docker --version)"
else
    echo "[ERROR] Neither Podman nor Docker is installed."
    ALL_OK=false
fi

# Check Git
if command -v git >/dev/null 2>&1; then
    echo "[OK] Git is installed: $(git --version)"
else
    echo "[ERROR] Git is NOT installed."
    ALL_OK=false
fi

echo "========================================"
if [ "$ALL_OK" = true ]; then
    echo "System check PASSED! Ready to run KubeLab."
    exit 0
else
    echo "System check FAILED! Please install required tools."
    exit 1
fi
