# KubeLab Requirements

## Functional Requirements

| ID | Requirement | Priority | Status |
|---|---|---|---|
| FR-01 | User registration with email/password and role assignment | P0 | Implemented |
| FR-02 | JWT-based authentication with session management | P0 | Implemented |
| FR-03 | Progressive curriculum with 15 tracks from beginner to advanced | P0 | Implemented |
| FR-04 | 154 declarative labs with real Kubernetes sandbox environments | P0 | Implemented |
| FR-05 | Real xterm.js terminal over WebSocket for lab execution | P0 | Implemented |
| FR-06 | Monaco editor for YAML manifest editing and application | P0 | Implemented |
| FR-07 | State-based grading via Kubernetes API assertions | P0 | Implemented |
| FR-08 | XP/skill graph/badge tracking with level progression | P0 | Implemented |
| FR-09 | Quiz and assessment engine with scoring | P0 | Implemented |
| FR-10 | AI-powered tutoring assistance | P1 | Implemented |
| FR-11 | Production incident simulator with injectable faults | P1 | Implemented |
| FR-12 | Flutter mobile companion app (Android + iOS) | P1 | Implemented |
| FR-13 | PWA with offline support (service worker) | P1 | Implemented |
| FR-14 | GitOps evaluation (Argo CD sync/health) | P1 | Implemented |
| FR-15 | Service mesh lab support (Istio/Envoy) | P1 | Implemented |
| FR-16 | OpenTelemetry observability for labs | P1 | Implemented |
| FR-17 | Notification service via NATS event bus | P2 | Implemented |

## Non-Functional Requirements

| ID | Requirement | Target | Status |
|---|---|---|---|
| NFR-01 | API response latency | < 200ms p95 | Implemented |
| NFR-02 | Lab provisioning time | < 30s | Implemented |
| NFR-03 | Concurrent lab sessions | 100+ per node | Designed |
| NFR-04 | Test coverage | > 90% overall, ≥ 95% critical paths | Tooling in place |
| NFR-05 | Zero local host dependencies | Podman/containerized only | Implemented |
| NFR-06 | Multi-tenant lab isolation | PSS restricted + NetworkPolicy | Implemented |
| NFR-07 | Backup/restore RPO | 0 (zero data loss) | Implemented |
| NFR-08 | Backup/restore RTO | < 5 seconds | Implemented |
| NFR-09 | WCAG 2.2 AA accessibility | All interactive elements | Designed |
| NFR-10 | Container image non-root execution | All containers | Implemented |
| NFR-11 | CORS and rate limiting | All API endpoints | Implemented |
| NFR-12 | CI pipeline duration | < 15 min critical path | Implemented |

## Acceptance Criteria

### FR-04: Declarative Labs
- Labs defined as YAML in `labs/` directory
- Each lab has tasks with `StateAssertion` grading rules
- Validation engine parses and validates all 154 lab schemas
- Labs span 15 tracks: Linux & Containers, Kubernetes Core, Storage, Networking, Helm & Kustomize, Administration, Zero-Trust Security, GitOps, Service Mesh, Observability, Troubleshooting, SRE & Performance, Platform Engineering, Incidents, Certification Drills

### FR-05: Terminal
- Authenticated WebSocket endpoint at `/ws/terminal/{sessionId}`
- Real container shell via `kubectl exec` / `podman exec` inside learner sandbox
- Zero host shell fallback (fails closed if sandbox container is unavailable)
- Dropped environment variables and UID 10001 enforcement
