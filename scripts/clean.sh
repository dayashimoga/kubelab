#!/usr/bin/env bash
set -eo pipefail

COMPOSE_FILE="$(dirname "$0")/../infrastructure/containers/podman-compose.yml"
echo "Cleaning up KubeLab containers, volumes, and temporary caches..."

if command -v podman >/dev/null 2>&1; then
    podman compose -f "$COMPOSE_FILE" down -v --remove-orphans
elif command -v docker-compose >/dev/null 2>&1; then
    docker-compose -f "$COMPOSE_FILE" down -v --remove-orphans
fi

rm -rf "$(dirname "$0")/../target" \
       "$(dirname "$0")/../node_modules" \
       "$(dirname "$0")/../apps/web/.next" \
       "$(dirname "$0")/../coverage"

echo "KubeLab workspace cleaned."
