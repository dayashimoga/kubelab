#!/usr/bin/env bash
set -eo pipefail

echo "Stopping KubeLab services and tearing down containers..."
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if command -v podman &>/dev/null; then
    podman compose -f "$ROOT/infrastructure/containers/podman-compose.yml" down --remove-orphans 2>/dev/null || true
    podman compose -f "$ROOT/infrastructure/containers/podman-compose.test.yml" down --remove-orphans 2>/dev/null || true
elif command -v docker-compose &>/dev/null; then
    docker-compose -f "$ROOT/infrastructure/containers/podman-compose.yml" down --remove-orphans 2>/dev/null || true
    docker-compose -f "$ROOT/infrastructure/containers/podman-compose.test.yml" down --remove-orphans 2>/dev/null || true
fi

echo "KubeLab services stopped."
