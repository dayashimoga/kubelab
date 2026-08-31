# KubeLab Implementation Details & Architecture

## 1. Monorepo Organization

```text
kubelab/
├── apps/
│   ├── web/                     # Next.js 15 PWA frontend (React 18, Tailwind CSS)
│   └── mobile/                  # Flutter Android & iOS companion application
├── services/
│   ├── api/                     # Axum 0.7 API Gateway, Auth middleware, WebSocket PTY
│   ├── auth/                    # Argon2id hashing, JWT token manager
│   ├── learning/                # 12 Track curriculum & lesson service
│   ├── assessment/              # Quiz engine, grading, and breakdown analytics
│   ├── labs/                    # Declarative lab sessions, manifest apply, resource sync
│   ├── lab-orchestrator/        # Sandbox namespace provisioner & chaos engine
│   ├── progress/                # XP tracking, streaks, badges, skill graph DAG
│   ├── notification/            # Event-driven alert dispatcher
│   └── ai-tutor/                # Socratic tutor service
├── packages/
│   ├── validation-engine/       # Rust state assertion engine & schema validator
│   ├── shared-types/            # Shared TypeScript data models
│   ├── ui/                      # Accessible React design system components
│   ├── lab-sdk/                 # Lab schema builder and validator
│   └── curriculum/              # Curriculum track definitions
├── infrastructure/
│   └── containers/              # Containerfiles and podman-compose.yml
├── labs/                        # Declarative lab definitions (YAML)
├── docs/                        # Complete architectural and operational guides
└── scripts/                     # Automation and production quality gate scripts
```

---

## 2. Key Subsystems Implementation

### Real WebSocket Terminal Streaming (`services/api/src/routes/terminal_ws.rs`)
- Implements WebSocket upgrade on `/v1/ws/terminal/:session_id`.
- Spawns subprocess (`/bin/bash` or `powershell.exe`) with `Stdio::piped()` and streams stdout/stderr chunks asynchronously to the client while writing stdin keystrokes.
- Provides fallback interactive sandbox execution when subshells are restricted by host policy.

### Deterministic State Validation (`packages/validation-engine`)
- Extracts nested JSON field paths (`status.phase`, `spec.containers[0].ports[0].containerPort`).
- Compares actual cluster object state against expected values with type-safe operators (`equals`, `contains`, `greater_than`, `regex`).

### API Authorization & Security (`services/api/src/routes/auth.rs`)
- Protects private routes using `AuthClaims` extractor validating `Bearer` JWT tokens.
- Argon2id password hashing with salt and parameter tuning.
- Adversarially tested against forged tokens, SQL injection, and path traversal attempts.
