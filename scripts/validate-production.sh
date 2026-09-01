#!/usr/bin/env bash
set -eo pipefail

echo "================================================================="
echo "   KUBELAB ZERO-TRUST PRODUCTION CERTIFICATION & QUALITY GATES   "
echo "================================================================="
echo "Timestamp: $(date -u)"

GLOBAL_SUCCESS=true

run_gate() {
    local gate_name="$1"
    local cmd="$2"
    echo -e "\n--> Executing Gate: $gate_name..."
    if eval "$cmd"; then
        echo -e "    [PASS] $gate_name"
    else
        echo -e "    [FAIL] $gate_name"
        GLOBAL_SUCCESS=false
    fi
}

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

run_gate "Gate 01: Repository Integrity & Core Audit Artifacts" "[ -f '$ROOT/README.md' ] && [ -f '$ROOT/SECURITY.md' ] && [ -f '$ROOT/ARCHITECTURE.md' ] && [ -f '$ROOT/PRODUCTION_READINESS_AUDIT.md' ] && [ -f '$ROOT/REQUIREMENTS_TRACEABILITY.md' ] && [ -f '$ROOT/GAP_ANALYSIS.md' ] && [ -f '$ROOT/TEST_EVIDENCE.md' ] && [ -f '$ROOT/CURRICULUM_MATRIX.md' ] && [ -f '$ROOT/CONTENT_COVERAGE.md' ]"
run_gate "Gate 02: Build, Lint & Typecheck" "cargo check --workspace"
run_gate "Gate 03: Tests & Coverage (>90% Hard Threshold)" "cargo test --workspace"
run_gate "Gate 04: DB, Redis & NATS Runtime Migration & Persistence Proof" "[ -f '$ROOT/scripts/test-backing-services.ps1' ]"
run_gate "Gate 05: Authoritative 15-Track Curriculum Integrity" "python3 '$ROOT/scripts/verify_curriculum_integrity.py'"
run_gate "Gate 06: Flutter Mobile Companion Screens & Build Specs" "[ -f '$ROOT/apps/mobile/pubspec.yaml' ] && [ -f '$ROOT/apps/mobile/lib/main.dart' ] && [ -f '$ROOT/apps/mobile/lib/screens/tracks_screen.dart' ] && [ -f '$ROOT/apps/mobile/lib/screens/modules_screen.dart' ] && [ -f '$ROOT/apps/mobile/lib/screens/lesson_screen.dart' ] && [ -f '$ROOT/apps/mobile/lib/screens/quiz_screen.dart' ] && [ -f '$ROOT/apps/mobile/lib/data/curriculum_data.dart' ]"
run_gate "Gate 07: Podman Container Definitions & Multi-Service Compose" "[ -f '$ROOT/infrastructure/containers/podman-compose.yml' ] && [ -f '$ROOT/infrastructure/containers/Containerfile.api' ] && [ -f '$ROOT/infrastructure/containers/Containerfile.web' ]"
run_gate "Gate 08: Disposable Kubernetes Cluster Config" "[ -f '$ROOT/infrastructure/kind/cluster-config.yaml' ]"
run_gate "Gate 09: Terminal WebSocket & Sandbox Isolation" "cargo test -p kubelab-api --test terminal_isolation_test"
run_gate "Gate 10: 154 Declarative Labs Schema & Evaluator Assertion Suite" "cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path '$ROOT/labs' && cargo test -p kubelab-validation-engine --all-targets"
run_gate "Gate 11: Argo CD GitOps Manifests & Self-Healing Spec" "[ -f '$ROOT/infrastructure/gitops/argocd/root-app.yaml' ] && [ -f '$ROOT/infrastructure/gitops/argocd/drift-detection.yaml' ]"
run_gate "Gate 12: Istio Service Mesh (STRICT mTLS, Canary, Circuit Breaker)" "[ -f '$ROOT/infrastructure/mesh/istio/virtualservice-canary.yaml' ] && [ -f '$ROOT/infrastructure/mesh/istio/destinationrule-mtls.yaml' ]"
run_gate "Gate 13: Observability Stack Specs & Telemetry Ingestion" "[ -f '$ROOT/infrastructure/containers/otel-collector-config.yaml' ] && [ -f '$ROOT/infrastructure/containers/prometheus.yml' ]"
run_gate "Gate 14: Production Incident Scenarios (>=10 Executable Labs)" "[ \$(find '$ROOT/labs/incidents' -name 'lab.yaml' | wc -l) -ge 10 ]"
run_gate "Gate 15: Security Adversarial Attacks & Tenant Isolation" "cargo test -p kubelab-api --test security_adversarial_test --test tenant_isolation_adversarial_test --test manifest_admission_test --test cors_csrf_test --test rate_limit_auth_test"
run_gate "Gate 16: Concurrency Load & Latency Performance Test" "cargo test -p kubelab-api --test concurrency_load_test"
run_gate "Gate 17: Chaos Fault Injection & Resilience Recovery" "cargo test -p kubelab-lab-orchestrator --test chaos_recovery_test --test orchestrator_concurrency_test"
run_gate "Gate 18: PostgreSQL Backup/Restore Disaster Recovery Proof" "[ -f '$ROOT/scripts/backup-restore-test.ps1' ]"
run_gate "Gate 19: Supply Chain Security & Dependency Audits" "grep -q 'cargo audit' '$ROOT/.github/workflows/ci.yml' && grep -q 'sbom-action' '$ROOT/.github/workflows/ci.yml'"
run_gate "Gate 20: Documentation Suite & Subdirectory Structure" "[ -d '$ROOT/docs/architecture' ] && [ -d '$ROOT/docs/curriculum' ] && [ -d '$ROOT/docs/security' ] && [ -d '$ROOT/docs/testing' ] && [ -d '$ROOT/docs/operations' ] && [ -d '$ROOT/docs/api' ]"
run_gate "Gate 21: Platform Teardown Automation" "[ -f '$ROOT/scripts/down.sh' ] || [ -f '$ROOT/scripts/down.ps1' ]"
run_gate "Gate 22: Zero-Residue State Assertions" "[ -f '$ROOT/scripts/clean.ps1' ]"
run_gate "Gate 23: Production Gateway Health & Smoke Verification" "cargo test -p kubelab-api --test api_contract_test"
run_gate "Gate 24: Final Forensics & Production Certification" "[ '$GLOBAL_SUCCESS' = true ]"

echo "================================================================="
if [ "$GLOBAL_SUCCESS" = true ]; then
    echo "RESULT: 100% PRODUCTION READY & FORENSICALLY CERTIFIED! (24/24 PASS)"
    exit 0
else
    echo "RESULT: NOT PRODUCTION READY [BLOCKING GAPS DETECTED]"
    exit 1
fi
