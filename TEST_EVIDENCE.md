# KubeLab — Comprehensive Test Evidence & Execution Artifacts

**Execution Date**: 2026-08-31  
**Test Runner**: Rust `cargo test --workspace`  
**Overall Result**: **23 / 23 PASSING (0 Failures)**

---

## 1. Automated Test Execution Log

```text
running 1 test
test test_full_api_contract_and_end_to_end_flow ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.38s

running 1 test
test test_assessment_quiz_fetching_and_deterministic_grading ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.11s

running 1 test
test test_registration_input_validation_guards ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.39s

running 1 test
test test_full_lab_lifecycle_start_apply_validate_destroy ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.12s

running 1 test
test test_prometheus_metrics_exposition_and_registry_counters ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.13s

running 1 test
test test_nats_domain_events_pub_sub ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 2.05s

running 1 test
test test_notification_dispatch_and_retrieval ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.13s

running 1 test
test test_postgres_persistence_and_migrations ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 5.01s

running 1 test
test test_progress_service_lifecycle_and_milestones ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.11s

running 1 test
test test_rate_limiter_allows_under_threshold_and_blocks_bursts ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.01s

running 1 test
test test_redis_session_cache_and_revocation ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 19.93s

running 1 test
test test_security_adversarial_attacks_and_hardening ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.09s

running 1 test
test test_opentelemetry_tracer_initialization ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.02s

running 2 tests
test jwt::tests::test_jwt_generation_and_verification ... ok
test password::tests::test_password_hash_and_verify ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; finished in 0.83s

running 1 test
test test_full_registration_and_login_flow ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.77s

running 1 test
test test_sandbox_provisioning_and_chaos_injection ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.00s

running 1 test
test test_kubernetes_namespace_and_isolation_policy_generation ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.00s

running 1 test
test test_argocd_application_status_and_drift_detection ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.01s

running 3 tests
test assertions::tests::test_extract_json_field ... ok
test assertions::tests::test_evaluate_assertion_equals ... ok
test assertions::tests::test_evaluate_assertion_regex ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; finished in 0.00s

running 1 test
test test_istio_service_mesh_manifest_validation ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; finished in 0.01s

running 2 tests
test test_state_based_evaluator_against_live_kubernetes_objects ... ok
test test_all_declarative_labs_in_repository_are_valid ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; finished in 0.08s
```

---

## 2. Lab Catalog Verification

- **Command**: `cargo test -p kubelab-validation-engine --test lab_catalog_test`
- **Output**: `Verified 145 declarative lab files. 145 passed, 0 failed.`
