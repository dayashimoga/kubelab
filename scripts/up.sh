#!/usr/bin/env bash
set -eo pipefail

echo "========================================"
echo "       STARTING KUBELAB PLATFORM        "
echo "========================================"

# Run doctor
bash "$(dirname "$0")/doctor.sh"

COMPOSE_FILE="$(dirname "$0")/../infrastructure/containers/podman-compose.yml"

echo "[INFO] Starting all services..."
if command -v podman >/dev/null 2>&1; then
    podman compose -f "$COMPOSE_FILE" up -d --build
elif command -v docker-compose >/dev/null 2>&1; then
    docker-compose -f "$COMPOSE_FILE" up -d --build
else
    echo "[ERROR] No container compose found."
    exit 1
fi

echo "========================================"
echo "  KubeLab Platform is running!          "
echo "  Web App:       http://localhost:3000"
echo "  API Docs:      http://localhost:8080/swagger-ui"
echo "  Health Check:  http://localhost:8080/healthz"
echo "========================================"
