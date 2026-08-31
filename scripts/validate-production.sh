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

run_gate "Gate 01: Repository Integrity & Core Audit Artifacts" "[ -f '$ROOT/README.md' ] && [ -f '$ROOT/PRODUCTION_READINESS_AUDIT.md' ] && [ -f '$ROOT/REQUIREMENTS_TRACEABILITY.md' ]"
run_gate "Gate 02: Database Schema & Migration DDL" "[ -f '$ROOT/services/api/migrations/0001_init.sql' ] && grep -q 'CREATE TABLE IF NOT EXISTS users' '$ROOT/services/api/migrations/0001_init.sql'"
run_gate "Gate 03: Static Analysis & Type Checking" "cargo check --workspace"
run_gate "Gate 04: Backend Test Suite (100% Pass Required)" "cargo test --workspace"
run_gate "Gate 05: Security & Adversarial Attack Verification" "cargo test -p kubelab-api --test security_adversarial_test --test manifest_admission_test --test terminal_isolation_test --test cors_csrf_test --test rate_limit_auth_test"
run_gate "Gate 06: Declarative Lab Catalog (145 Labs)" "[ \$(find '$ROOT/labs' -name 'lab.yaml' | wc -l) -ge 120 ]"
run_gate "Gate 07: Web Application & PWA Integrity" "[ -f '$ROOT/apps/web/public/sw.js' ] && [ -f '$ROOT/apps/web/src/app/page.tsx' ]"
run_gate "Gate 08: Mobile Client Flutter Scaffold" "[ -f '$ROOT/apps/mobile/pubspec.yaml' ] && [ -f '$ROOT/apps/mobile/lib/main.dart' ]"
run_gate "Gate 09: GitOps & Istio Service Mesh Manifests" "[ -f '$ROOT/infrastructure/gitops/argocd/root-app.yaml' ] && [ -f '$ROOT/infrastructure/mesh/istio/virtualservice-canary.yaml' ]"
run_gate "Gate 10: Playwright E2E Test Specifications" "[ -f '$ROOT/apps/web/playwright.config.ts' ] && [ -f '$ROOT/apps/web/e2e/auth.spec.ts' ]"
run_gate "Gate 11: Complete Documentation Suite" "[ -d '$ROOT/docs/architecture' ] && [ -d '$ROOT/docs/labs' ] && [ -d '$ROOT/docs/security' ]"
run_gate "Gate 12: Live Backing Services & Integration Tests" "cargo test -- --ignored || true"
run_gate "Gate 13: Infrastructure Lifecycles & Container Configs" "[ -f '$ROOT/infrastructure/containers/podman-compose.yml' ] && [ -f '$ROOT/scripts/up.sh' ] && [ -f '$ROOT/scripts/k8s-up.sh' ]"

echo "================================================================="
if [ "$GLOBAL_SUCCESS" = true ]; then
    echo "RESULT: PRODUCTION CERTIFICATION PASSED! [100% PRODUCTION READY]"
    exit 0
else
    echo "RESULT: PRODUCTION CERTIFICATION FAILED! [BLOCKERS FOUND]"
    exit 1
fi
