# KubeLab — Repository Structure & Codebase Map

## Workspace Layout

```
kubelab/
├── apps/
│   ├── web/                     # Next.js 14 Web Frontend (React, TypeScript, Tailwind, Monaco, xterm.js)
│   │   ├── src/app/             # App Router pages (auth, learn, labs, incidents, profile)
│   │   ├── src/components/      # UI components (terminal, editor, quiz, visualizers)
│   │   └── e2e/                 # Playwright E2E test specs
│   └── mobile/                  # Flutter 3.24 Cross-Platform Mobile Client (Dart)
│       └── lib/                 # Mobile screens (learning, labs, progress, auth)
├── services/                    # Rust Backend Microservices (Axum, Tokio, Kube-rs)
│   ├── api/                     # Core API Gateway & HTTP Routing & Backing Service Wiring
│   ├── auth/                    # Argon2id Authentication, Password Policy & JWT Token Engine
│   ├── labs/                    # Declarative Lab Engine & Session Management
│   ├── lab-orchestrator/        # Kubernetes Provisioner, Quotas, NetPol, Chaos Engine
│   ├── learning/                # Curriculum & Track Navigation Services
│   ├── assessment/              # Quiz Assessment & Deterministic Grading Engine
│   ├── ai-tutor/                # AI Contextual Assistant (5 Socratic Modes)
│   ├── progress/                # Learner XP, Skill Tree, & Progress Tracking
│   └── notification/            # In-App & Email Notification Dispatcher
├── packages/                    # Shared Libraries & Validation Engines
│   └── validation-engine/       # State-Based JSONPath Evaluator & Lab Catalog Verifier
├── labs/                        # 145 Declarative YAML Lab Definitions across 14 Tracks
├── infrastructure/              # Platform & Cloud-Native Infrastructure
│   ├── containers/              # Containerfiles (API, Web, Flutter, Toolchain) & Compose files
│   ├── kind/                    # Disposable Kind Cluster Configuration
│   ├── k8s/                     # Production Kubernetes Manifests & Base Overlays
│   └── observability/           # OTel Collector, Prometheus, Grafana Dashboards
└── scripts/                     # Platform Automation & Zero-Trust Certification
    ├── up.ps1 / up.sh           # One-command platform spin-up
    ├── down.ps1 / down.sh       # Clean shutdown
    ├── clean.ps1 / clean.sh     # Zero-residue purge
    ├── lab-up.ps1 / lab-up.sh   # Disposable cluster provisioning
    ├── test-containerized.ps1   # Zero-host-install containerized test suite
    └── validate-production.ps1  # 13-gate production certification runner
```

## Component Responsibilities

| Subsystem | Tech Stack | Primary Responsibility |
|---|---|---|
| `apps/web` | Next.js 14, TypeScript | Interactive web portal, Monaco editor, xterm.js terminal |
| `services/api` | Rust (Axum, Tokio) | API Gateway, routing, rate limiting, metrics, health |
| `services/auth` | Rust (Argon2, JWT) | Zero-trust authentication, token generation, password verification |
| `services/labs` | Rust (Kube-rs, YAML) | Lab lifecycle, manifest apply, live state fetching |
| `services/lab-orchestrator` | Rust (Kube-rs, Tokio) | Sandbox namespace provisioning, quotas, network policies, chaos |
| `packages/validation-engine` | Rust (JSONPath, Serde) | Deterministic evaluation of live Kubernetes state |
| `labs/` | Declarative YAML | 145 lab definitions covering beginner to advanced scenarios |
