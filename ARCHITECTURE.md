# KubeLab Architecture

## Overview

KubeLab is a production-grade cloud-native learning and lab platform built with:

- **Backend**: Rust (Axum) workspace — 10 domain crates
- **Frontend**: Next.js 14 (TypeScript) with PWA support
- **Mobile**: Flutter (Android + iOS)
- **Data**: PostgreSQL 16, Redis 7, NATS 2.10 (JetStream)
- **Infrastructure**: Podman/Kind, Argo CD, Istio
- **Observability**: OpenTelemetry → Tempo, Prometheus, Loki, Grafana

## System Context

```
┌─────────────┐    ┌──────────────┐    ┌──────────────┐
│   Learner   │    │  Instructor  │    │    Admin     │
│  (Web/PWA)  │    │  (Web/API)   │    │  (Web/CLI)  │
└──────┬──────┘    └──────┬───────┘    └──────┬───────┘
       │                  │                   │
       └─────────┬────────┴───────┬───────────┘
                 │                │
         ┌───────▼───────┐  ┌────▼──────┐
         │  API Gateway  │  │  Mobile   │
         │  (Axum:8080)  │  │ (Flutter) │
         └───────┬───────┘  └───────────┘
                 │
    ┌────────────┼────────────────────────────┐
    │            │     Domain Services        │
    │  ┌────────┴────────┐  ┌──────────────┐ │
    │  │ kubelab-auth    │  │ kubelab-labs │ │
    │  │ kubelab-learning│  │ kubelab-orch │ │
    │  │ kubelab-assess  │  │ kubelab-valid│ │
    │  │ kubelab-progress│  │ kubelab-ai   │ │
    │  │ kubelab-notif   │  │              │ │
    │  └────────┬────────┘  └──────┬───────┘ │
    └───────────┼──────────────────┼─────────┘
                │                  │
    ┌───────────▼──────┐  ┌───────▼──────────┐
    │   Data Layer     │  │  Lab Infra       │
    │ PG │ Redis │NATS │  │ K8s│Argo│Istio  │
    └──────────────────┘  │ OTel│Prom│Grafana│
                          └──────────────────┘
```

## Workspace Structure

| Path | Type | Description |
|---|---|---|
| `services/api` | Rust crate | API gateway (Axum, routes, middleware) |
| `services/auth` | Rust crate | Authentication (JWT, Argon2, RBAC) |
| `services/learning` | Rust crate | Curriculum, tracks, lessons |
| `services/assessment` | Rust crate | Quizzes, scoring, evaluations |
| `services/labs` | Rust crate | Lab session management |
| `services/lab-orchestrator` | Rust crate | K8s namespace provisioning, chaos |
| `services/progress` | Rust crate | XP, skills, badges, levels |
| `services/notification` | Rust crate | NATS event bus notifications |
| `services/ai-tutor` | Rust crate | AI-powered tutoring |
| `packages/validation-engine` | Rust crate | Lab schema validation & grading |
| `packages/curriculum` | TypeScript | Track/lesson data for web |
| `packages/shared-types` | TypeScript | Shared type definitions |
| `packages/lab-sdk` | TypeScript | Lab client SDK |
| `packages/ui` | TypeScript | Shared UI components |
| `apps/web` | Next.js | Web application (PWA) |
| `apps/mobile` | Flutter | Mobile companion app |
| `labs/` | YAML | 154 declarative lab definitions (15 tracks) |
| `infrastructure/` | YAML/Config | Containers, Kind, GitOps, Istio |

## Data Flow

### Lab Lifecycle
1. Learner starts lab → `POST /api/labs/{id}/start`
2. Orchestrator provisions K8s namespace with PSS/LimitRange/NetworkPolicy
3. Terminal WebSocket connects to sandbox shell
4. Learner executes kubectl / applies YAML via Monaco editor
5. Grading: `POST /api/labs/{id}/grade` → validation engine queries K8s state via JSONPath
6. Score + XP + progress update → cleanup namespace

### Authentication Flow
1. Register: email/password → Argon2 hash → PostgreSQL
2. Login: verify hash → mint JWT (HS256) → Redis session
3. All API calls: `Authorization: Bearer <JWT>` → middleware validation
4. WebSocket: JWT via query parameter → verify + session ownership check

## Network & Port Map

| Service | Port | Protocol |
|---|---|---|
| Web App | 3000 | HTTP |
| API Gateway | 8080 | HTTP/WS |
| PostgreSQL | 5432 | TCP |
| Redis | 6379 | TCP |
| NATS | 4222/8222 | TCP/HTTP |
| Prometheus | 9090 | HTTP |
| Grafana | 3001 | HTTP |
| Tempo | 3200/4317 | HTTP/gRPC |
| Loki | 3100 | HTTP |
| OTel Collector | 4317/4318 | gRPC/HTTP |

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for complete C4 diagrams, sequence flows, and deployment architecture.
