# KubeLab — Cloud-Native Learning & Lab Platform

[![CI](https://github.com/kubelab/kubelab/actions/workflows/ci.yml/badge.svg)](https://github.com/kubelab/kubelab/actions/workflows/ci.yml)
[![Production Gate](https://github.com/kubelab/kubelab/actions/workflows/production-validation.yml/badge.svg)](https://github.com/kubelab/kubelab/actions/workflows/production-validation.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![WCAG 2.2 AA](https://img.shields.io/badge/WCAG-2.2_AA-green.svg)](https://www.w3.org/WAI/WCAG22/quickref/)

> **Production-grade open-source Cloud-Native Engineering Learning Platform**: responsive Web + Android/iOS mobile app for learning, practicing and proving skills in Linux, containers/Docker/Podman, Kubernetes, Advanced Kubernetes, networking/CNI/eBPF/Gateway API, security, Helm/Kustomize, Istio/Envoy/Service Mesh, OpenTelemetry, Prometheus/Grafana, logs/metrics/traces, Argo CD/GitOps, SRE/SLI/SLO, reliability, performance, platform engineering, multi-cluster, disaster recovery, and production incident response.

---

## ⚡ Zero Local Installation

You do not need to install Node, Rust, Python, Flutter, kubectl, Helm, Docker or databases on your host. Everything runs in containerized environments with **Podman** or disposable local clusters.

```bash
# Clone the repository
git clone https://github.com/kubelab/kubelab.git
cd kubelab

# Start all services with one command (PowerShell or Bash)
./scripts/up.ps1     # On Windows
./scripts/up.sh      # On Linux/macOS
```

Open your browser to:
- **Web App**: [http://localhost:3000](http://localhost:3000)
- **API & OpenAPI Docs**: [http://localhost:8080/swagger-ui](http://localhost:8080/swagger-ui)
- **Admin Portal**: [http://localhost:3001](http://localhost:3001)

---

## 🧭 Architecture

```text
Users
 ├── Web (Next.js/TypeScript/PWA)
 └── Mobile (Flutter Android/iOS)
          │
          ▼
 API Gateway / Auth (Rust/Axum)
          │
 ┌────────┼──────────────────────────────────┐
 ▼        ▼          ▼          ▼            ▼
Learning Assessment Progress   AI Tutor   Notifications
 │        │          │
 └────────┴──────────┴──────► PostgreSQL / Redis / NATS Event Bus
                                  │
                                  ▼
                           Lab Orchestrator
                                  │
                         Lab Scheduler/Controller
                                  │
             ┌────────────────────┴────────────────────┐
             ▼                                         ▼
   Disposable Local/CI Labs                    Remote Lab Pool
 Podman/kind/k3d/etc.                           Kubernetes
             │                                         │
             └───────────────┬─────────────────────────┘
                             ▼
                    Real Lab Environment
        ┌────────────┬────────────┬──────────────┐
        │ Kubernetes │ Argo CD    │ Istio/Envoy  │
        │ OTel       │ Prometheus │ Grafana      │
        └────────────┴────────────┴──────────────┘
                             │
                             ▼
                    Validation & Grading
                             │
                             ▼
                    Score / Skill Graph
```

---

## ✨ Features

- **No Mocks, No Fakes**: Real xterm.js terminal over WebSocket, real Kubernetes namespaces/clusters, real Monaco editor, real state-based grading.
- **Progressive Curriculum**: 12 complete tracks from Linux foundations to multi-cluster & incident response.
- **Production Incident Simulator**: Live multi-tier microservices with injectable faults (DNS failures, crashloops, memory leaks, cert expiry, service mesh routing bugs, GitOps drift).
- **Interactive Visualizations**: Dynamic architecture diagrams, topology explorer, service mesh graph, and D3.js skill tree.
- **Deterministic State Grading**: Validates actual live system state via Kubernetes API & custom assertions — never brittle regex on shell output.
- **Mobile First**: Complete Flutter companion app for quizzes, lessons, flashcards, diagrams, and "Continue on Desktop" lab handoff.
- **Enterprise Security**: Multi-tenant isolation, seccomp profiles, non-root execution, dropped capabilities, network policies, rate-limiting, and RBAC.

---

## 🛠️ CLI & Scripts

| Command (PowerShell / Bash) | Description |
|---|---|
| `./scripts/up.ps1` / `./scripts/up.sh` | Start all services, check health, display URLs |
| `./scripts/down.ps1` / `./scripts/down.sh` | Stop and clean up containers gracefully |
| `./scripts/dev.ps1` / `./scripts/dev.sh` | Start development mode with hot reload |
| `./scripts/test.ps1` / `./scripts/test.sh` | Run all unit, integration, and e2e tests |
| `./scripts/validate.ps1` / `./scripts/validate.sh` | Run fast quality gate |
| `./scripts/validate-production.ps1` / `.sh` | **Authoritative production certification gate** |
| `./scripts/doctor.ps1` / `./scripts/doctor.sh` | System prerequisites & environment check |
| `./scripts/clean.ps1` / `./scripts/clean.sh` | Full clean of volumes, images, and temp files |

---

## 📜 Documentation

- [Requirements Specification](docs/requirements/REQUIREMENTS.md)
- [System Architecture & ADRs](docs/architecture/ARCHITECTURE.md)
- [Curriculum Specification](docs/curriculum/CURRICULUM.md)
- [Lab Authoring SDK](docs/lab-authoring/SDK.md)
- [Security & Threat Model](docs/security/SECURITY.md)
- [Testing & Quality Gates](docs/testing/TESTING.md)
- [Production Readiness Checklist](docs/production-readiness/CHECKLIST.md)

---

## ⚖️ License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.
