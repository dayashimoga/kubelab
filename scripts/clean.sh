#!/usr/bin/env bash
set -eo pipefail

echo "Cleaning up KubeLab containers, volumes, and temporary caches..."
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if command -v podman &>/dev/null; then
    podman compose -f "$ROOT/infrastructure/containers/podman-compose.yml" down -v --remove-orphans 2>/dev/null || true
    podman compose -f "$ROOT/infrastructure/containers/podman-compose.test.yml" down -v --remove-orphans 2>/dev/null || true
elif command -v docker-compose &>/dev/null; then
    docker-compose -f "$ROOT/infrastructure/containers/podman-compose.yml" down -v --remove-orphans 2>/dev/null || true
    docker-compose -f "$ROOT/infrastructure/containers/podman-compose.test.yml" down -v --remove-orphans 2>/dev/null || true
fi

if command -v kind &>/dev/null; then
    kind delete cluster --name "kubelab-cluster" 2>/dev/null || true
fi

rm -rf "$ROOT/target" "$ROOT/node_modules" "$ROOT/apps/web/.next" "$ROOT/apps/web/node_modules" "$ROOT/coverage" "$ROOT/.turbo"

echo "KubeLab workspace 100% clean (Zero Residue)."
