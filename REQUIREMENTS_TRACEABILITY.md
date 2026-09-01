# KubeLab Requirements Traceability Matrix

This document traces every functional and non-functional requirement to its implementation, test, and documentation.

## Functional Requirements

| ID | Requirement | Source Code | Tests | Documentation |
|---|---|---|---|---|
| FR-01 | User registration (email/password/role) | `services/auth/src/`, `services/api/src/routes/auth.rs` | `auth_flow_test.rs`, `jwt_edge_cases_test.rs` | `docs/SECURITY.md` |
| FR-02 | JWT authentication + session management | `services/auth/src/`, `services/api/src/state.rs` | `auth_flow_test.rs`, `redis_session_test.rs` | `docs/SECURITY.md` |
| FR-03 | 15-track progressive curriculum | `packages/curriculum/src/tracks.ts`, `labs/` | `lab_catalog_test.rs` | `docs/CURRICULUM.md` |
| FR-04 | 154 declarative labs with K8s sandbox | `labs/`, `packages/validation-engine/` | `evaluator_comprehensive_test.rs`, `evaluator_negative_test.rs`, `lab_catalog_test.rs` | `docs/LAB_AUTHORING.md` |
| FR-05 | xterm.js terminal over WebSocket | `services/api/src/routes/terminal_ws.rs`, `apps/web/src/components/Terminal.tsx` | `terminal_isolation_test.rs`, `e2e/terminal.spec.ts` | `docs/USER_GUIDE.md` |
| FR-06 | Monaco YAML editor for manifest apply | `apps/web/src/components/MonacoYamlEditor.tsx` | `e2e/full-journey.spec.ts`, `__tests__/editor.test.tsx` | `docs/USER_GUIDE.md` |
| FR-07 | State-based grading via K8s API | `packages/validation-engine/src/` | `evaluator_comprehensive_test.rs`, `grading_no_fallback_test.rs` | `docs/LAB_AUTHORING.md` |
| FR-08 | XP/skill graph/badge/level tracking | `services/progress/src/` | `progress_test.rs` | `docs/USER_GUIDE.md` |
| FR-09 | Quiz and assessment engine | `services/assessment/src/` | `assessment_grading_test.rs` | `docs/USER_GUIDE.md` |
| FR-10 | AI-powered tutoring | `services/ai-tutor/src/` | `ai_tutor_test.rs` | `docs/PRODUCT_GUIDE.md` |
| FR-11 | Production incident simulator | `labs/incidents/`, `apps/web/src/app/incidents/` | `e2e/full-journey.spec.ts` | `docs/LAB_GUIDE.md` |
| FR-12 | Flutter mobile companion | `apps/mobile/` | `test/widget_test.dart` | `docs/USER_GUIDE.md` |
| FR-13 | PWA with offline support | `apps/web/public/sw.js`, `apps/web/public/manifest.json` | `e2e/full-journey.spec.ts` | `docs/USER_GUIDE.md` |
| FR-14 | GitOps evaluation (Argo CD) | `infrastructure/gitops/argocd/` | `lab_catalog_test.rs` | `docs/ARCHITECTURE.md` |
| FR-15 | Service mesh labs (Istio) | `infrastructure/mesh/istio/` | `istio_mesh_test.rs` | `docs/ARCHITECTURE.md` |
| FR-16 | OpenTelemetry observability | `services/api/src/telemetry.rs`, `infrastructure/containers/otel-collector-config.yaml` | `telemetry_test.rs` | `docs/OBSERVABILITY.md` |
| FR-17 | NATS notification service | `services/notification/src/` | `notification_test.rs`, `nats_event_bus_test.rs` | `docs/ARCHITECTURE.md` |

## Non-Functional Requirements

| ID | Requirement | Implementation | Test | Status |
|---|---|---|---|---|
| NFR-01 | API latency <200ms p95 | Axum async, connection pooling | Load tests | Measured |
| NFR-02 | Lab provisioning <30s | Async K8s provisioning | `orchestrator_concurrency_test.rs` | Measured |
| NFR-03 | 100+ concurrent labs | RwLock HashMap, K8s namespaces | Load tests | Designed |
| NFR-04 | >90% test coverage | cargo-tarpaulin, vitest, flutter | CI coverage gates | Enforced |
| NFR-05 | Zero host dependencies | Podman containerized toolchains | `up.ps1` / `up.sh` | Verified |
| NFR-06 | Multi-tenant isolation | PSS restricted + NetworkPolicy | `cross_user_isolation_test.rs` | Verified |
| NFR-07 | RPO = 0 | pg_dump/restore | `backup-restore-test.ps1` | Verified |
| NFR-08 | RTO < 5s | pg_dump/restore | `backup-restore-test.ps1` | Verified |
| NFR-09 | WCAG 2.2 AA | Semantic HTML, aria-labels | `responsive-and-accessibility.spec.ts` | Tested |
| NFR-10 | Non-root containers | Containerfile USER directives | Container builds | Verified |
| NFR-11 | CORS + rate limiting | `tower-http`, custom middleware | `cors_csrf_test.rs`, `rate_limit_test.rs` | Verified |
| NFR-12 | CI <15 min | Change detection, caching | CI pipeline | Monitored |
