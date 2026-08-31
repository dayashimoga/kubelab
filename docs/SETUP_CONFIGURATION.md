# KubeLab Setup & Configuration

## Prerequisites

| Tool | Purpose | Install |
|---|---|---|
| **Podman** or **Docker** | Container runtime | [podman.io](https://podman.io/getting-started/installation) |
| **Git** | Source control | [git-scm.com](https://git-scm.com/downloads) |

> **Zero local installation**: You do not need Rust, Node.js, Flutter, kubectl, or databases on your host. Everything runs inside containers.

## Quick Start

```bash
git clone https://github.com/dayashimoga/kubelab.git
cd kubelab
./scripts/up.ps1     # Windows PowerShell
./scripts/up.sh      # Linux/macOS
```

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | `postgres://kubelab:kubelab_secret_password@127.0.0.1:5432/kubelab` | PostgreSQL connection |
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis connection |
| `NATS_URL` | `nats://127.0.0.1:4222` | NATS event bus |
| `JWT_SECRET` | Generated at startup | JWT signing secret (min 32 chars) |
| `RUST_LOG` | `info,kubelab=debug` | Log level filter |
| `PORT` | `8080` | API server port |
| `HOST` | `0.0.0.0` | API bind address |
| `NODE_ENV` | `production` | Next.js environment |
| `NEXT_TELEMETRY_DISABLED` | `1` | Disable Next.js telemetry |

## Ports

| Port | Service | Notes |
|---|---|---|
| 3000 | Web App (Next.js) | Frontend |
| 3001 | Grafana | Observability dashboards |
| 4222 | NATS | Event bus |
| 4317 | OTel Collector (gRPC) | Trace ingestion |
| 5432 | PostgreSQL | Primary database |
| 6379 | Redis | Session cache |
| 8080 | API (Axum) | Backend REST + WebSocket |
| 8222 | NATS Monitor | NATS health/stats |
| 9090 | Prometheus | Metrics |

## Storage

- **PostgreSQL data**: `postgres-data/` (gitignored)
- **Redis data**: `redis-data/` (gitignored)
- **NATS data**: `nats-data/` (gitignored)
- **Lab cluster data**: `.kind/` or `.k3d/` (gitignored)

## Profiles

### Development
```bash
./scripts/dev.ps1   # Hot reload, debug logging
```

### Testing
```bash
./scripts/test.ps1  # Run all tests with backing services
```

### Production Validation
```bash
./scripts/validate-production.ps1  # 13-gate certification
```

## Containerized Development (Podman)

Build and run the full stack without any host-installed toolchains:

```bash
# Build all containers
podman-compose -f infrastructure/containers/podman-compose.yml build

# Start the stack
podman-compose -f infrastructure/containers/podman-compose.yml up -d

# Run tests inside the toolchain container
podman-compose -f infrastructure/containers/podman-compose.test.yml run --rm toolchain cargo test
```
