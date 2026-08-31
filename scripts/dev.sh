#!/usr/bin/env bash
set -eo pipefail

echo "Starting KubeLab in Development Mode..."

COMPOSE_FILE="$(dirname "$0")/../infrastructure/containers/podman-compose.yml"
if command -v podman >/dev/null 2>&1; then
    podman compose -f "$COMPOSE_FILE" up -d postgres redis nats
elif command -v docker-compose >/dev/null 2>&1; then
    docker-compose -f "$COMPOSE_FILE" up -d postgres redis nats
fi

if command -v cargo >/dev/null 2>&1 && command -v pnpm >/dev/null 2>&1; then
    echo "Running local dev servers..."
    (cd "$(dirname "$0")/.." && cargo run -p kubelab-api) &
    (cd "$(dirname "$0")/../apps/web" && pnpm dev) &
    wait
else
    echo "Running containerized dev environment..."
    if command -v podman >/dev/null 2>&1; then
        podman compose -f "$COMPOSE_FILE" up
    else
        docker-compose -f "$COMPOSE_FILE" up
    fi
fi
