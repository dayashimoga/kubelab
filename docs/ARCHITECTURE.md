# KubeLab Architecture

## System Context

```mermaid
C4Context
    title KubeLab System Context
    Person(learner, "Learner", "Cloud-native engineer learning K8s")
    Person(instructor, "Instructor", "Creates labs and curriculum")
    Person(admin, "Admin", "Platform operator")

    System(kubelab, "KubeLab Platform", "Cloud-native learning and lab platform")

    System_Ext(k8s, "Kubernetes Clusters", "Lab sandbox environments")
    System_Ext(argocd, "Argo CD", "GitOps continuous delivery")
    System_Ext(istio, "Istio", "Service mesh for labs")
    System_Ext(otel, "OpenTelemetry", "Distributed tracing and metrics")

    Rel(learner, kubelab, "Uses", "HTTPS/WSS")
    Rel(instructor, kubelab, "Manages", "HTTPS")
    Rel(admin, kubelab, "Operates", "HTTPS/CLI")
    Rel(kubelab, k8s, "Provisions labs", "K8s API")
    Rel(kubelab, argocd, "GitOps sync", "gRPC")
    Rel(kubelab, istio, "Mesh config", "xDS")
    Rel(kubelab, otel, "Telemetry", "OTLP")
```

## Container Diagram

```mermaid
graph TB
    subgraph "Client Tier"
        WEB["Web App<br/>Next.js/TypeScript/PWA<br/>Port 3000"]
        MOBILE["Mobile App<br/>Flutter Android/iOS"]
    end

    subgraph "API Tier"
        API["API Gateway<br/>Rust/Axum<br/>Port 8080"]
    end

    subgraph "Domain Services (Rust Crates)"
        AUTH["kubelab-auth<br/>JWT + Argon2"]
        LEARN["kubelab-learning<br/>Lessons/Tracks"]
        ASSESS["kubelab-assessment<br/>Quizzes/Scoring"]
        PROGRESS["kubelab-progress<br/>XP/Skills/Badges"]
        LABS["kubelab-labs<br/>Lab Service"]
        ORCH["kubelab-lab-orchestrator<br/>K8s Provisioning"]
        AI["kubelab-ai-tutor<br/>AI Assistance"]
        NOTIF["kubelab-notification<br/>Events"]
    end

    subgraph "Data Tier"
        PG["PostgreSQL 16<br/>Primary Store"]
        REDIS["Redis 7<br/>Sessions/Cache"]
        NATS["NATS 2.10<br/>Event Bus"]
    end

    subgraph "Lab Infrastructure"
        K8S["Kubernetes<br/>Kind/k3d"]
        ARGO["Argo CD"]
        ISTIO["Istio/Envoy"]
        OTEL_COL["OTel Collector"]
        PROM["Prometheus"]
        GRAFANA["Grafana"]
        TEMPO["Tempo"]
        LOKI["Loki"]
    end

    subgraph "Validation"
        VALID["kubelab-validation-engine<br/>Schema + Grading"]
    end

    WEB -->|HTTPS/WSS| API
    MOBILE -->|HTTPS| API
    API --> AUTH
    API --> LEARN
    API --> ASSESS
    API --> PROGRESS
    API --> LABS
    API --> AI
    API --> NOTIF
    LABS --> ORCH
    ORCH --> K8S
    LABS --> VALID
    AUTH --> PG
    AUTH --> REDIS
    NOTIF --> NATS
    PROGRESS --> PG
    K8S --> ARGO
    K8S --> ISTIO
    K8S --> OTEL_COL
    OTEL_COL --> PROM
    OTEL_COL --> TEMPO
    OTEL_COL --> LOKI
    PROM --> GRAFANA
    TEMPO --> GRAFANA
    LOKI --> GRAFANA
```

## Component Responsibilities

### API Gateway (`services/api`)
- HTTP/WebSocket routing via Axum
- JWT authentication middleware
- CORS, rate limiting, request validation
- Terminal WebSocket proxy with sandbox isolation
- OpenAPI/Swagger documentation
- Health checks and Prometheus metrics endpoint

### Auth Service (`services/auth`)
- User registration with Argon2 password hashing
- JWT token generation and validation
- Role-based access control (Learner, Instructor, Admin)
- Session management via Redis

### Lab Orchestrator (`services/lab-orchestrator`)
- Kubernetes namespace provisioning with PSS `restricted` profiles
- LimitRange enforcement and resource quotas
- IMDS metadata blocking via NetworkPolicy
- Chaos injection for incident simulation
- Manifest application and cleanup

### Validation Engine (`packages/validation-engine`)
- Declarative lab schema definition and parsing
- State-based grading via Kubernetes API assertions
- JSONPath field extraction and comparison
- Support for 154 lab definitions across 15 tracks

## Data Flow: Lab Lifecycle

```mermaid
sequenceDiagram
    participant L as Learner
    participant W as Web/Mobile
    participant A as API
    participant O as Orchestrator
    participant K as Kubernetes
    participant V as Validator

    L->>W: Start Lab
    W->>A: POST /api/labs/{id}/start
    A->>O: provision_namespace()
    O->>K: Create Namespace + PSS + LimitRange
    K-->>O: Namespace Ready
    O-->>A: Session Created
    A-->>W: Lab Session + Terminal URL

    L->>W: Execute kubectl commands
    W->>A: WebSocket /ws/terminal/{session}
    A->>K: Proxied shell execution

    L->>W: Submit for grading
    W->>A: POST /api/labs/{id}/grade
    A->>V: evaluate(session, assertions)
    V->>K: Query actual state (jsonpath)
    K-->>V: Resource state
    V-->>A: Score + Feedback
    A-->>W: Grade Result + XP
```

## Deployment Architecture

```mermaid
graph TB
    subgraph "Container Runtime (Podman/Docker)"
        API_C["Containerfile.api<br/>rust:latest → debian:bookworm-slim"]
        WEB_C["Containerfile.web<br/>node:22-alpine"]
        TOOL_C["Containerfile.toolchain<br/>Full dev environment"]
    end

    subgraph "Compose Stack"
        PC["podman-compose.yml<br/>API + Web + PG + Redis + NATS"]
        PCT["podman-compose.test.yml<br/>Test overrides"]
    end

    subgraph "Observability Stack"
        OTEL_CFG["otel-collector-config.yaml"]
        GRAF_DS["grafana/datasources.yaml<br/>Prometheus + Tempo + Loki"]
    end

    PC --> API_C
    PC --> WEB_C
    PC --> OTEL_CFG
    PC --> GRAF_DS
```

## Network & Port Map

| Service | Port | Protocol | Description |
|---|---|---|---|
| Web App | 3000 | HTTP | Next.js frontend |
| API | 8080 | HTTP/WS | Axum REST + WebSocket |
| PostgreSQL | 5432 | TCP | Primary database |
| Redis | 6379 | TCP | Session cache |
| NATS | 4222 | TCP | Event bus |
| NATS Monitor | 8222 | HTTP | NATS monitoring |
| Prometheus | 9090 | HTTP | Metrics |
| Grafana | 3001 | HTTP | Dashboards |
| Tempo | 4317 | gRPC | Traces (OTLP) |
| Loki | 3100 | HTTP | Logs |
| OTel Collector | 4318 | HTTP | Telemetry ingestion |

## Security Boundaries

- **API perimeter**: All external traffic terminates at the Axum API gateway with JWT validation
- **Lab isolation**: Each learner gets a dedicated Kubernetes namespace with PSS `restricted`, LimitRange, and IMDS-blocking NetworkPolicy
- **Terminal sandbox**: WebSocket terminal sessions run in isolated shell processes with environment sanitization and 30-minute idle timeout
- **Container runtime**: All containers run as non-root users with dropped capabilities
- **Admission control**: Server-side YAML admission rejects privileged pods, hostNetwork, hostPath, runtime socket mounts, and cluster-admin bindings
