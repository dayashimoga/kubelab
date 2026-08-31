# Changelog

All notable changes to the KubeLab Platform will be documented in this file.

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
