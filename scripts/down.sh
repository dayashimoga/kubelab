#!/usr/bin/env bash
set -eo pipefail

COMPOSE_FILE="$(dirname "$0")/../infrastructure/containers/podman-compose.yml"
echo "Stopping KubeLab services..."

if command -v podman >/dev/null 2>&1; then
    podman compose -f "$COMPOSE_FILE" down
elif command -v docker-compose >/dev/null 2>&1; then
    docker-compose -f "$COMPOSE_FILE" down
fi
echo "KubeLab services stopped."
