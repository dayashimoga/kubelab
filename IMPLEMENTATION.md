# KubeLab — Technical Implementation Guide & Architecture

## 1. Architecture Overview

KubeLab is built on an asynchronous, high-throughput Rust backend, Next.js 15 frontend, and Flutter mobile companion app orchestrated via containerized infrastructure.

```text
                               ┌───────────────────────────┐
                               │  Next.js 15 PWA / Flutter │
                               └─────────────┬─────────────┘
                                             │ HTTP / WebSocket (JWT)
                                             ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                             KUBELAB API GATEWAY                             │
│  ┌───────────────┬───────────────┬────────────────┬──────────────────────┐  │
│  │ Auth & RBAC   │ Lab Service   │ Terminal PTY   │ AI Pedagogical Tutor │  │
│  └───────┬───────┴───────┬───────┴────────┬───────┴──────────┬───────────┘  │
└──────────┼───────────────┼────────────────┼──────────────────┼──────────────┘
           │               │                │                  │
           ▼               ▼                ▼                  ▼
    ┌─────────────┐ ┌─────────────┐  ┌─────────────┐   ┌───────────────┐
    │ PostgreSQL  │ │    Redis    │  │  NATS Event │   │  Kind / K3s   │
    │ (Relational)│ │  (Sessions) │  │     Bus     │   │   Sandboxes   │
    └─────────────┘ └─────────────┘  └─────────────┘   └───────────────┘
```

---

## 2. Key Technology Stack

- **Backend Runtime**: Rust 1.78+ (`tokio`, `axum`, `sqlx`, `redis`, `async-nats`, `opentelemetry`, `prometheus`, `kube-rs`)
- **Web Frontend**: Next.js 15 (App Router, Tailwind CSS, Lucide Icons, Service Worker PWA)
- **Mobile Companion**: Flutter 3.22+ (Material 3 Dark Theme, Dart HTTP Client)
- **Container Infrastructure**: Podman / Docker Compose (`infrastructure/containers/podman-compose.yml`)
- **Kubernetes Sandbox**: Kind v1.30.0 with isolated namespaces, ResourceQuotas, and default-deny NetworkPolicies (`scripts/lab-up.ps1`)
- **Observability**: OpenTelemetry Collector, Prometheus 2.54, Grafana 11.2 (auto-provisioned)
- **Declarative Lab Engine**: `kubelab-validation-engine` evaluating JSONPath, Regex, and live cluster object state across 154 lab definitions.
