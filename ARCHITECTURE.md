# KubeLab — System Architecture & Design

## 1. High-Level Architecture Overview

KubeLab is built as a cloud-native, microservices-oriented distributed platform engineered in Rust, TypeScript, and Dart.

```text
                               ┌──────────────────────────────────────────────┐
                               │                 Clients                      │
                               │  - Web App (Next.js / TypeScript / PWA)      │
                               │  - Mobile App (Flutter Android / iOS)        │
                               │  - Admin Portal (Next.js)                    │
                               └──────────────────────┬───────────────────────┘
                                                      │ HTTPS / WSS
                                                      ▼
                               ┌──────────────────────────────────────────────┐
                               │           API Gateway (Rust / Axum)          │
                               │  - Auth Middleware & JWT Verification        │
                               │  - Rate Limiting & Audit Logging             │
                               │  - OpenTelemetry Tracing & Metrics           │
                               │  - WebSocket Terminal & Event Router         │
                               └──────┬───────────────┬───────────────┬───────┘
                                      │               │               │
            ┌─────────────────────────┼───────────────┴───────────────┼─────────────────────────┐
            ▼                         ▼                               ▼                         ▼
 ┌─────────────────────┐   ┌─────────────────────┐         ┌─────────────────────┐   ┌─────────────────────┐
 │    Auth Service     │   │  Learning Service   │         │ Assessment Service  │   │  Progress Service   │
 │ - OIDC / OAuth2     │   │ - Tracks & Lessons  │         │ - Quiz Bank & Engine│   │ - Skill Graph DAG   │
 │ - Sessions (Redis)  │   │ - Content Search    │         │ - Exam Timer        │   │ - XP & Streaks      │
 │ - RBAC Policies     │   │ - Visualizations    │         │ - Score Aggregator  │   │ - Badges & Certs    │
 └──────────┬──────────┘   └──────────┬──────────┘         └──────────┬──────────┘   └──────────┬──────────┘
            │                         │                               │                         │
            └─────────────────────────┼───────────────────────────────┼─────────────────────────┘
                                      ▼                               ▼
                      ┌─────────────────────────────────────────────────────────┐
                      │              Data & Messaging Foundation                │
                      │  - PostgreSQL 16 (Relational state, ACID, migrations)   │
                      │  - Redis 7 (Sessions, rate limits, caching)             │
                      │  - NATS Core/JetStream (Event bus, async notifications) │
                      └────────────────────────────┬────────────────────────────┘
                                                   │
                                                   ▼
                      ┌─────────────────────────────────────────────────────────┐
                      │          Lab Orchestrator (Rust / Tokio)                │
                      │  - Cluster Pool Manager (kind / k3d / k8s)              │
                      │  - Namespace Lifecycle Controller                       │
                      │  - Real-time State Validation Engine                    │
                      │  - Chaos & Fault Injection Controller                   │
                      │  - Orphan Recovery & Resource Garbage Collection        │
                      └────────────────────────────┬────────────────────────────┘
                                                   │
                                                   ▼
                      ┌─────────────────────────────────────────────────────────┐
                      │              Disposable Lab Environments                │
                      │  - Dedicated learner namespaces                         │
                      │  - Ephemeral learner shell containers (sandboxed)       │
                      │  - Live Kubernetes API (RBAC restricted)                │
                      │  - Argo CD / Istio / OpenTelemetry / Prometheus stack    │
                      └─────────────────────────────────────────────────────────┘
```

## 2. Core Subsystems

### 2.1 Backend Services (Rust / Axum)
- **`services/api`**: Public-facing API gateway routing requests, handling authentication, CORS, rate-limiting, and managing streaming WebSocket connections for interactive shells.
- **`services/auth`**: User registration, password authentication (Argon2id), session management, and RBAC token generation.
- **`services/learning`**: Course catalog, hierarchical curriculum structure, Markdown/MDX content parsing, and full-text search.
- **`services/assessment`**: Interactive quizzes, randomized option shuffling, timed assessments, and automated scoring calculation.
- **`services/labs`**: Lab metadata API, user lab session lifecycle (`start`, `stop`, `extend`, `validate`), and logs inspection.
- **`services/lab-orchestrator`**: Background engine managing Podman/kind containers, deploying initial manifest configurations, executing state-based assertions, and pruning expired environments.
- **`services/progress`**: User progression engine computing XP, level thresholds, learning streaks, unlockable achievements, and skill graph DAG mastery.
- **`services/notification`**: Event-driven notification dispatcher listening to NATS events for lab completion, achievements, and system alerts.
- **`services/ai-tutor`**: Contextual LLM assistant with prompt guards providing Socratic coaching and troubleshooting guidance without providing answers.

### 2.2 Frontend Applications
- **`apps/web`**: Ultra-responsive Next.js 15 application utilizing CSS custom properties, WCAG 2.2 AA standards, xterm.js terminal, Monaco YAML editor, and dynamic SVG/Canvas architecture visualizers.
- **`apps/mobile`**: Flutter cross-platform mobile app targeting Android and iOS with offline lesson support, quiz practice, and skill tracking.
- **`apps/admin`**: Instructor and administrator portal for managing curriculum content, monitoring active lab clusters, and analyzing learner performance metrics.

### 2.3 Shared Packages
- **`packages/shared-types`**: Core TypeScript models and API contracts.
- **`packages/ui`**: Reusable accessible UI components, theme tokens, and layout primitives.
- **`packages/lab-sdk`**: Declarative lab authoring SDK and JSON Schema validator.
- **`packages/curriculum`**: Versioned curriculum data files across 12 cloud-native tracks.
- **`packages/validation-engine`**: Deterministic state assertions for Kubernetes objects, HTTP endpoints, and log patterns.

## 3. Data Model Architecture

```text
 users 1──* user_sessions
   │
   ├──* user_progress ──* lessons ──* modules ──* courses
   │
   ├──* lab_sessions ──* labs ──* lab_tasks
   │
   ├──* quiz_attempts ──* questions
   │
   └──* user_skills ──* skills
```
