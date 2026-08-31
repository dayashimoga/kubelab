# KubeLab Requirements Traceability

## Requirement → Implementation → Test Mapping

| Req ID | Requirement | Implementation | Test |
|---|---|---|---|
| FR-01 | User registration | `services/auth/src/password.rs`, `services/api/src/routes/auth.rs` | `auth_flow_test.rs` |
| FR-02 | JWT authentication | `services/auth/src/jwt.rs`, `services/api/src/routes/auth.rs` | `jwt_edge_cases_test.rs` |
| FR-03 | Progressive curriculum | `services/learning/src/service.rs`, `packages/curriculum/` | `api_contract_test.rs` |
| FR-04 | 145 declarative labs | `labs/`, `packages/validation-engine/` | `validate_lab_schema` binary, `evaluator_negative_test.rs` |
| FR-05 | Terminal over WebSocket | `services/api/src/routes/terminal_ws.rs` | `terminal_isolation_test.rs` |
| FR-06 | Monaco editor | `apps/web/src/components/` | Playwright E2E |
| FR-07 | State-based grading | `packages/validation-engine/src/assertions.rs` | `grading_no_fallback_test.rs` |
| FR-08 | XP/skill tracking | `services/progress/src/service.rs`, `skill_graph.rs` | `progress_test.rs` |
| FR-09 | Quiz engine | `services/assessment/src/service.rs` | `ai_tutor_test.rs` |
| FR-10 | AI tutoring | `services/ai-tutor/src/service.rs` | `ai_tutor_test.rs` |
| FR-11 | Incident simulation | `services/lab-orchestrator/src/chaos.rs` | `chaos_recovery_test.rs` |
| FR-12 | Flutter mobile app | `apps/mobile/lib/` | `widget_test.dart` |
| FR-13 | PWA | `apps/web/public/sw.js` | `ci.yml` web job |
| FR-14 | GitOps evaluation | `services/labs/src/gitops.rs` | `gitops_argocd_test.rs` |
| FR-15 | Service mesh labs | `packages/validation-engine/src/rules/mesh.rs` | `istio_mesh_test.rs` |
| NFR-06 | Lab isolation | `services/lab-orchestrator/src/k8s/namespace_provisioner.rs` | `terminal_isolation_test.rs` |
| NFR-11 | CORS + rate limiting | `services/api/src/routes/mod.rs` | `cors_csrf_test.rs`, `rate_limit_auth_test.rs` |
