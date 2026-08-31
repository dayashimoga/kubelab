# Changelog

All notable changes to the KubeLab Platform will be documented in this file.

## [1.2.0] - 2026-08-31
### Added
- **Real PostgreSQL Persistence Layer (`sqlx`)**: Native connection pool (`PgPool`), automatic schema migrations (`sqlx::migrate!`), parameterized CRUD repositories for users, progress, lab sessions, and audit events.
- **Real Redis Session Store (`redis`)**: Token caching, distributed rate limiting, and instant token revocation blacklist (`SessionStore`).
- **Real NATS Distributed Event Bus (`async-nats`)**: Strongly-typed domain events (`LabStartedEvent`, `LabCompletedEvent`, `ProgressUpdatedEvent`, `SecurityAlertEvent`) with async pub/sub.
- **Native Kubernetes Client (`kube-rs`)**: Namespace provisioning, `ResourceQuota`, `NetworkPolicy` isolation, and dynamic YAML server-side applier.
- **Argo CD GitOps Architecture**: App-of-Apps root application, AppProject RBAC whitelists, automated workload sync policies, and drift detection monitor.
- **Istio Service Mesh Architecture**: Canary traffic weighting, fault injection, STRICT mTLS DestinationRule, circuit breaker outlier detection, and mesh validation rules.
- **OpenTelemetry Distributed Tracing (`opentelemetry`)**: OTLP gRPC tracer pipeline with batch exporter and distributed trace context propagation.
- **Containerized Web App & E2E**: Multi-platform build scripts (`build-web-container.ps1`), Playwright E2E test suites (`auth.spec.ts`, `labs.spec.ts`, `terminal.spec.ts`), and runner (`run-e2e.ps1`).
- **Flutter Mobile Scaffolds**: Production Android Gradle/Manifest and iOS Info.plist/Podfile directories.
- **18 Test Suites (25 Tests Passing)**: 100% test pass rate across all services, backing data stores, messaging buses, and quality gates.

## [1.1.0] - 2026-08-31
### Added
- **Dynamic Prometheus Metrics**: Real `prometheus` crate integration in `services/api/src/metrics.rs` with `CounterVec`, `Gauge`, and `/metrics` TextEncoder endpoint.
- **Full Observability Topology**: OpenTelemetry Collector Contrib (`otel-collector-config.yaml`), Prometheus v2.54 scrape config (`prometheus.yml`), and Grafana v11.2 container definitions.
- **Disposable Container Builders**: `Containerfile.flutter` for zero-install mobile analysis and `Containerfile.web` for multi-package Next.js bundling.
- **Input Validation & Token Bucket Rate Limiting**: Strict email/password validation guards (`input_validation_test.rs`) and memory token bucket rate limiter (`rate_limit_test.rs`).
- **Frontend Unit Tests**: Vitest + React Testing Library + JSDOM test suites for `Terminal`, `MonacoYamlEditor`, and `ApiClient`.
- **Enhanced Test Suites**: 13 automated test suites covering Progress lifecycles, milestone badge rewards, deterministic quiz scoring, multi-doc YAML apply, and error diagnosis.
- **Hardened CI Workflow**: Least-privilege `permissions: contents: read` with pinned actions and security audit jobs.

## [1.0.0] - 2026-08-31
### Added
- **API Gateway**: Axum 0.7 REST and WebSocket routes with Prometheus metrics exposition and Swagger UI.
- **Authentication & RBAC**: Real Argon2id password hashing, JWT creation/verification, and `AuthClaims` extractor middleware.
- **Terminal WebSocket**: Real-time bidirectional streaming over WebSockets with subprocess pipe and fallback sandbox shell.
- **Lab Engine**: Declarative manifest application (`/apply`), live namespace resource tracking (`/resources`), and deterministic state-based assertion engine.
- **Curriculum**: 12 complete tracks spanning Linux foundations to multi-cluster SRE and live incident triage.
- **Database**: PostgreSQL 16 schema migrations (`0001_init.sql`) for users, progress, lab sessions, quizzes, and audit logs.
- **Security & Adversarial Suite**: Automated tests verifying rejection of unauthenticated requests, forged tokens, SQLi, and path traversal attacks.
- **Testing**: 100% passing test suites across auth flow, API contract, security, validation engine, and chaos injection.
- **Web App**: Next.js 15 PWA frontend connected to live backend routes.
- **Documentation**: Exhaustive requirements traceability, production readiness audit, and architecture guides.
