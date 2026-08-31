#!/usr/bin/env bash
set -e

echo "================================================================="
echo "         STARTING KUBELAB CLOUD-NATIVE PLATFORM                  "
echo "================================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/../infrastructure/containers/podman-compose.yml"

if command -v podman-compose &>/dev/null; then
    podman-compose -f "$COMPOSE_FILE" up -d --build
elif command -v docker-compose &>/dev/null; then
    docker-compose -f "$COMPOSE_FILE" up -d --build
elif command -v podman &>/dev/null; then
    podman compose -f "$COMPOSE_FILE" up -d --build
elif command -v docker &>/dev/null; then
    docker compose -f "$COMPOSE_FILE" up -d --build
else
    echo "[ERROR] No container compose tool found."
    exit 1
fi

echo ""
echo "================================================================="
echo "  KubeLab Cloud-Native Platform is fully running!                "
echo "================================================================="
echo "  Web Application:       http://localhost:3000"
echo "  API Gateway & Health:  http://localhost:8080/healthz"
echo "  Prometheus Metrics:    http://localhost:9090"
echo "  Grafana Dashboards:    http://localhost:3001  (admin / admin)"
echo "  Tempo Distributed Tracing: http://localhost:3200"
echo "  Loki Structured Logs:  http://localhost:3100"
echo "================================================================="
