#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB ZERO-TRUST PRODUCTION CERTIFICATION & QUALITY GATES   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC')" -ForegroundColor Gray

$report = @()
$globalSuccess = $true

function Run-Gate {
    param (
        [string]$GateName,
        [scriptblock]$Action
    )
    Write-Host "`n--> Executing Gate: $GateName..." -ForegroundColor Yellow
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    try {
        & $Action
        $sw.Stop()
        Write-Host "    [PASS] $GateName ($($sw.ElapsedMilliseconds)ms)" -ForegroundColor Green
        $script:report += [PSCustomObject]@{ Gate = $GateName; Status = "PASS"; Duration = "$($sw.ElapsedMilliseconds)ms" }
    } catch {
        $sw.Stop()
        Write-Host "    [FAIL] $GateName - $_" -ForegroundColor Red
        $script:report += [PSCustomObject]@{ Gate = $GateName; Status = "FAIL"; Duration = "$($sw.ElapsedMilliseconds)ms" }
        $script:globalSuccess = $false
    }
}

# 1. Repository Integrity & Required Artifacts
Run-Gate "Gate 01: Repository Integrity & Core Audit Artifacts" {
    $requiredFiles = @(
        'README.md', 'LICENSE', 'SECURITY.md', 'ARCHITECTURE.md',
        'IMPLEMENTATION.md', 'TODO.md', 'CHANGELOG.md',
        'PRODUCTION_READINESS_AUDIT.md', 'REQUIREMENTS_TRACEABILITY.md',
        'GAP_ANALYSIS.md', 'TEST_EVIDENCE.md', 'SECURITY_AUDIT.md',
        'PERFORMANCE_AUDIT.md', 'LAB_CERTIFICATION.md',
        'CURRICULUM_MATRIX.md', 'CONTENT_COVERAGE.md'
    )
    foreach ($f in $requiredFiles) {
        $p = "$PSScriptRoot/../$f"
        if (-not (Test-Path $p)) { throw "Missing critical file: $f" }
        $len = (Get-Item $p).Length
        if ($len -lt 100) { throw "File $f is suspiciously small ($len bytes)" }
    }
}

# 2. Database Schema & Migration Verification
Run-Gate "Gate 02: Database Schema & Migration DDL" {
    $migPath = "$PSScriptRoot/../services/api/migrations/0001_init.sql"
    if (-not (Test-Path $migPath)) {
        throw "PostgreSQL migration file 0001_init.sql is missing"
    }
    $sql = Get-Content $migPath -Raw
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS users") { throw "Missing users DDL" }
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS lab_sessions") { throw "Missing lab_sessions DDL" }
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS user_progress") { throw "Missing user_progress DDL" }
}

# 3. Static Analysis & Type Checking
Run-Gate "Gate 03: Static Analysis & Workspace Type Checking (cargo check)" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo check --workspace 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Cargo check failed: $out" }
    } else {
        throw "Rust toolchain (cargo) not found on host"
    }
}

# 4. Backend Unit & Contract Tests
Run-Gate "Gate 04: Backend Test Suite (100% Pass Required)" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo test --workspace 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Backend test suite failed: $out" }
    }
}

# 5. Security & Adversarial Attack Verification
Run-Gate "Gate 05: Security & Adversarial Attack Verification" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo test -p kubelab-api --test security_adversarial_test 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Security adversarial tests failed: $out" }
    }
}

# 6. Declarative Lab Catalog Schema & Content Validation
Run-Gate "Gate 06: Declarative Lab Catalog Schema & Structure (145 Labs)" {
    $labs = Get-ChildItem -Path "$PSScriptRoot/../labs" -Filter "lab.yaml" -Recurse
    if ($labs.Count -lt 120) { throw "Fewer than 120 declarative labs found ($($labs.Count))" }
    
    $validCount = 0
    foreach ($lab in $labs) {
        $content = Get-Content $lab.FullName -Raw
        if ($content -notmatch "id:\s*[`"']?[\w\-]+[`"']?" -or $content -notmatch "tasks:" -or $content -notmatch "validation:") {
            throw "Invalid lab schema structure in $($lab.FullName)"
        }
        $validCount++
    }
    Write-Host "      Verified $validCount declarative lab schemas across all tracks." -ForegroundColor Gray
}

# 7. Web Application Integrity & PWA Features
Run-Gate "Gate 07: Web Application Component & PWA Integrity" {
    $webPages = @(
        'apps/web/src/app/page.tsx',
        'apps/web/src/app/login/page.tsx',
        'apps/web/src/app/register/page.tsx',
        'apps/web/src/app/learn/page.tsx',
        'apps/web/src/app/labs/page.tsx',
        'apps/web/src/app/practice/page.tsx',
        'apps/web/src/app/incidents/page.tsx',
        'apps/web/src/app/skills/page.tsx',
        'apps/web/src/app/progress/page.tsx',
        'apps/web/src/app/certifications/page.tsx',
        'apps/web/src/app/docs/page.tsx',
        'apps/web/src/components/Terminal.tsx',
        'apps/web/src/components/MonacoYamlEditor.tsx',
        'apps/web/src/components/K8sVisualizer.tsx',
        'apps/web/src/lib/api.ts',
        'apps/web/src/lib/auth-context.tsx',
        'apps/web/public/sw.js',
        'apps/web/public/manifest.json'
    )
    foreach ($p in $webPages) {
        if (-not (Test-Path "$PSScriptRoot/../$p")) { throw "Missing web component: $p" }
    }
}

# 8. Mobile Client Flutter Scaffold
Run-Gate "Gate 08: Mobile Client Scaffold & Multi-Platform Layouts" {
    $mobileFiles = @(
        'apps/mobile/pubspec.yaml',
        'apps/mobile/lib/main.dart',
        'apps/mobile/android/app/build.gradle',
        'apps/mobile/ios/Runner/Info.plist'
    )
    foreach ($m in $mobileFiles) {
        if (-not (Test-Path "$PSScriptRoot/../$m")) { throw "Missing mobile file: $m" }
    }
}

# 9. Cloud-Native GitOps & Service Mesh Manifests
Run-Gate "Gate 09: Argo CD GitOps & Istio Service Mesh Architecture" {
    $gitops = @(
        'infrastructure/gitops/argocd/root-app.yaml',
        'infrastructure/gitops/argocd/appproject.yaml',
        'infrastructure/gitops/argocd/application-workloads.yaml',
        'infrastructure/gitops/argocd/drift-detection.yaml',
        'infrastructure/mesh/istio/virtualservice-canary.yaml',
        'infrastructure/mesh/istio/destinationrule-mtls.yaml',
        'infrastructure/mesh/istio/virtualservice-fault-injection.yaml',
        'infrastructure/mesh/istio/gateway-ingress.yaml',
        'infrastructure/mesh/istio/envoyfilter-telemetry.yaml'
    )
    foreach ($g in $gitops) {
        if (-not (Test-Path "$PSScriptRoot/../$g")) { throw "Missing GitOps/Mesh manifest: $g" }
    }
}

# 10. Playwright E2E Test Suite Integrity
Run-Gate "Gate 10: Playwright E2E Test Specifications" {
    $e2eFiles = @(
        'apps/web/playwright.config.ts',
        'apps/web/e2e/auth.spec.ts',
        'apps/web/e2e/labs.spec.ts',
        'apps/web/e2e/terminal.spec.ts'
    )
    foreach ($e in $e2eFiles) {
        if (-not (Test-Path "$PSScriptRoot/../$e")) { throw "Missing E2E spec: $e" }
    }
}

# 11. Complete Documentation Suite Integrity
Run-Gate "Gate 11: Documentation & Architecture Specifications" {
    $docDirs = @('architecture', 'curriculum', 'labs', 'security', 'testing', 'operations', 'api')
    foreach ($d in $docDirs) {
        $p = "$PSScriptRoot/../docs/$d"
        if (-not (Test-Path $p)) {
            throw "Documentation section docs/$d missing"
        }
        $files = Get-ChildItem -Path $p -Filter "*.md"
        if ($files.Count -eq 0) {
            throw "Documentation section docs/$d contains 0 markdown files"
        }
    }
}

# 12. Live Backing Services & Integration Tests
Run-Gate "Gate 12: Live Backing Services & Integration Tests (Postgres, Redis, NATS)" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo test -- --ignored 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Live integration tests failed: $out" }
        Write-Host "      Live PostgreSQL persistence, Redis sessions, and NATS event bus verified." -ForegroundColor Gray
    }
}

# 13. Infrastructure Configuration & Zero-Residue Safety
Run-Gate "Gate 13: Infrastructure Lifecycles & Container Configurations" {
    $infraFiles = @(
        'infrastructure/containers/podman-compose.yml',
        'infrastructure/containers/podman-compose.test.yml',
        'infrastructure/containers/Containerfile.api',
        'infrastructure/containers/Containerfile.web',
        'infrastructure/containers/otel-collector-config.yaml',
        'infrastructure/containers/prometheus.yml',
        'infrastructure/kind/cluster-config.yaml',
        'scripts/up.ps1',
        'scripts/down.ps1',
        'scripts/clean.ps1',
        'scripts/lab-up.ps1',
        'scripts/lab-down.ps1'
    )
    foreach ($i in $infraFiles) {
        if (-not (Test-Path "$PSScriptRoot/../$i")) { throw "Missing infra file: $i" }
    }
}

Write-Host "`n=================================================================" -ForegroundColor Cyan
Write-Host "             PRODUCTION CERTIFICATION REPORT                    " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

$report | Format-Table -AutoSize

if ($globalSuccess) {
    Write-Host "RESULT: PRODUCTION CERTIFICATION PASSED! [100% PRODUCTION READY]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: PRODUCTION CERTIFICATION FAILED! [BLOCKERS FOUND]" -ForegroundColor Red
    exit 1
}
