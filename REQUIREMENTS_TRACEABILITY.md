# KubeLab Requirements Traceability Matrix

## 1. Traceability Overview

Every functional requirement is mapped to its expected behavior, code location, automated test, execution command, observable evidence, status, identified gap, and applied fix.

---

## 2. Requirements Matrix

### REQ-001: Authentication & Password Security
- **Expected Behavior**: Learner registers with email/password; password hashed with Argon2id; secure JWT issued.
- **Code Location**: `services/auth/src/password.rs`, `services/auth/src/jwt.rs`, `services/auth/src/service.rs`
- **Automated Test**: `services/auth/tests/auth_flow_test.rs`
- **Command**: `cargo test -p kubelab-auth --test auth_flow_test`
- **Evidence**: `test test_full_registration_and_login_flow ... ok`
- **Status**: `PROVEN`
- **Gap**: None.
- **Fix**: Argon2id with 19MB memory cost, 2 iterations, 1 parallelism.

---

### REQ-002: API Authorization & JWT Verification
- **Expected Behavior**: Protected endpoints extract and verify `Bearer <jwt>` in `Authorization` header, reject forged/expired tokens with 401.
- **Code Location**: `services/api/src/routes/auth.rs` (`AuthClaims` extractor)
- **Automated Test**: `services/api/tests/security_adversarial_test.rs`
- **Command**: `cargo test -p kubelab-api --test security_adversarial_test`
- **Evidence**: `test test_security_adversarial_attacks_and_hardening ... ok`
- **Status**: `PROVEN`
- **Gap**: Missing route protection middleware.
- **Fix**: Added `AuthClaims` struct implementing `axum::extract::FromRequestParts`.

---

### REQ-003: Real Interactive Terminal over WebSocket
- **Expected Behavior**: Client opens WebSocket connection; backend streams interactive shell output and pipes client keystrokes.
- **Code Location**: `services/api/src/routes/terminal_ws.rs`, `apps/web/src/components/Terminal.tsx`
- **Automated Test**: `services/api/tests/api_contract_test.rs`
- **Command**: `cargo test -p kubelab-api --test api_contract_test`
- **Evidence**: `test test_full_api_contract_and_end_to_end_flow ... ok`
- **Status**: `PROVEN`
- **Gap**: Hardcoded static terminal simulator in frontend.
- **Fix**: Rebuilt with live WebSocket client, command history (up/down arrows), ANSI support, and backend `tokio::process` pipe.

---

### REQ-004: Declarative Lab Manifest Apply & Live Resources
- **Expected Behavior**: Monaco Editor allows applying YAML manifests directly to sandbox namespace; resources appear in live visualizer.
- **Code Location**: `services/labs/src/service.rs`, `services/api/src/routes/labs.rs`, `apps/web/src/components/MonacoYamlEditor.tsx`
- **Automated Test**: `services/api/tests/api_contract_test.rs`
- **Command**: `cargo test -p kubelab-api --test api_contract_test`
- **Evidence**: `HTTP POST /v1/labs/sessions/:id/apply` returns `200 OK` with configured resources.
- **Status**: `PROVEN`
- **Gap**: Manifest apply did not update backend resource state.
- **Fix**: Added `apply_manifest` and `get_namespace_resources` endpoints and session state tracking.

---

### REQ-005: Deterministic State-Based Lab Grading
- **Expected Behavior**: Evaluates JSON object path assertions (e.g. `status.phase == "Running"`, numeric bounds, regex) against live K8s objects without regex string matching.
- **Code Location**: `packages/validation-engine/src/evaluator.rs`, `packages/validation-engine/src/assertions.rs`
- **Automated Test**: `packages/validation-engine/tests/lab_catalog_test.rs`
- **Command**: `cargo test -p kubelab-validation-engine`
- **Evidence**: `test test_state_based_evaluator_against_live_kubernetes_objects ... ok`
- **Status**: `PROVEN`
- **Gap**: None.
- **Fix**: Recursive JSON path extraction and type-safe operator evaluation.

---

### REQ-006: 12 Curriculum Tracks & Knowledge Checks
- **Expected Behavior**: REST API serves 12 complete tracks, progressive lessons, and interactive quizzes.
- **Code Location**: `services/learning/src/data.rs`, `services/assessment/src/service.rs`
- **Automated Test**: `services/api/tests/api_contract_test.rs`
- **Command**: `cargo test -p kubelab-api --test api_contract_test`
- **Evidence**: `tracks.len() == 12` verified in automated test.
- **Status**: `PROVEN`
- **Gap**: None.
- **Fix**: Added track metadata, lesson summaries, concepts, and scoring formulas.

---

### REQ-007: Observability & Production Health Probes
- **Expected Behavior**: Exposes `/healthz`, `/readyz`, and Prometheus `/metrics` endpoints.
- **Code Location**: `services/api/src/routes/mod.rs`
- **Automated Test**: `services/api/tests/api_contract_test.rs`
- **Command**: `cargo test -p kubelab-api --test api_contract_test`
- **Evidence**: `/metrics` returns Prometheus text format (`kubelab_http_requests_total`).
- **Status**: `PROVEN`
- **Gap**: Missing Prometheus `/metrics` route.
- **Fix**: Implemented `prometheus_metrics_handler` exporting counters and gauges.

---

### REQ-008: Ephemeral Sandbox Isolation & Chaos Injection
- **Expected Behavior**: Sandbox provisioner allocates isolated namespace with NetworkPolicy limits; ChaosEngine injects reproducible faults.
- **Code Location**: `services/lab-orchestrator/src/provisioner.rs`, `services/lab-orchestrator/src/chaos.rs`
- **Automated Test**: `services/lab-orchestrator/tests/chaos_recovery_test.rs`
- **Command**: `cargo test -p kubelab-lab-orchestrator --test chaos_recovery_test`
- **Evidence**: `test test_sandbox_provisioning_and_chaos_injection ... ok`
- **Status**: `PROVEN`
- **Gap**: None.
- **Fix**: Chaos fault injection with DNS failure, Pod kill, and Network latency schemas.
