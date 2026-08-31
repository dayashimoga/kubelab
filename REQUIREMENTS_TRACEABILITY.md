# KubeLab — Requirements Traceability Matrix

## Traceability Mapping: `Requirement -> Implementation -> Test -> Status`

| Requirement ID | Requirement Description | Code Implementation | Verification Test | Runtime Evidence | Status |
|---|---|---|---|---|---|
| **REQ-AUTH-01** | Argon2id password hashing | `services/auth/src/password.rs` | `auth_flow_test.rs` | `cargo test -p kubelab-auth` | **PROVEN** |
| **REQ-AUTH-02** | JWT Token generation & claims | `services/auth/src/jwt.rs` | `auth_flow_test.rs` | Token expiration & claims match | **PROVEN** |
| **REQ-AUTH-03** | Multi-tenant user isolation | `services/auth/src/service.rs` | `cross_user_isolation_test.rs` | UserA / UserB sandboxes isolated | **PROVEN** |
| **REQ-AUTH-04** | Role-Based Access Control | `services/api/src/routes/auth.rs` | `endpoint_authorization_matrix_test.rs` | Anonymous 401 DENY / Bearer 200 ALLOW | **PROVEN** |
| **REQ-PERSIST-01** | PostgreSQL user & lab persistence | `services/api/src/db/` | `postgres_persistence_test.rs` | Tables created on PostgreSQL 16 | **PROVEN** |
| **REQ-CACHE-01** | Redis token caching & revocation | `services/api/src/cache/` | `redis_session_test.rs` | Keys stored with TTL on Redis 7 | **PROVEN** |
| **REQ-EVENT-01** | NATS domain event bus | `services/api/src/events/` | `nats_event_bus_test.rs` | Pub/sub messages received on NATS 2.10 | **PROVEN** |
| **REQ-LAB-01** | 145 Declarative YAML Labs | `labs/**/*.yaml` | `lab_catalog_test.rs` | 145 YAML files pass schema validation | **PROVEN** |
| **REQ-LAB-02** | State-based deterministic grading | `packages/validation-engine/` | `evaluator_negative_test.rs` | JSONPath field assertions evaluate live state | **PROVEN** |
| **REQ-WEB-01** | Production Next.js web portal | `apps/web/` | `Containerfile.web` | Production build 14 static routes pre-rendered | **PROVEN** |
| **REQ-TOOL-01** | Zero-host-install container toolchain | `Containerfile.toolchain` | `test-containerized.ps1` | Full test suite executes in container | **PROVEN** |
| **REQ-SEC-01** | Adversarial attack resistance | `services/api/tests/` | `security_adversarial_test.rs` | Path traversal & injection blocked | **PROVEN** |
| **REQ-OBS-01** | Prometheus Metrics & OTel | `services/api/src/metrics.rs` | `metrics_test.rs` | `/metrics` endpoint exports Prometheus format | **PROVEN** |
