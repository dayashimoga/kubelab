# Changelog

All notable changes to the KubeLab Platform will be documented in this file.

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
