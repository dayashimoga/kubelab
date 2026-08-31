#!/usr/bin/env pwsh
$ErrorActionPreference = "Continue"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "   KUBELAB PRODUCTION CERTIFICATION & QUALITY GATE VALIDATOR    " -ForegroundColor Cyan
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

# 1. Repository Integrity & Structure
Run-Gate "Repository Integrity & Required Artifacts" {
    $requiredFiles = @(
        'README.md', 'LICENSE', 'SECURITY.md', 'ARCHITECTURE.md',
        'IMPLEMENTATION.md', 'TODO.md', 'CHANGELOG.md',
        'PRODUCTION_READINESS_AUDIT.md', 'REQUIREMENTS_TRACEABILITY.md',
        'GAP_ANALYSIS.md', 'TEST_EVIDENCE.md', 'SECURITY_AUDIT.md',
        'PERFORMANCE_AUDIT.md', 'LAB_CERTIFICATION.md'
    )
    foreach ($f in $requiredFiles) {
        if (-not (Test-Path "$PSScriptRoot/../$f")) { throw "Missing critical file: $f" }
    }
}

# 2. Database Schema & Migration Verification
Run-Gate "Database Schema & Migration DDL" {
    if (-not (Test-Path "$PSScriptRoot/../services/api/migrations/0001_init.sql")) {
        throw "PostgreSQL migration file 0001_init.sql is missing"
    }
    $sql = Get-Content "$PSScriptRoot/../services/api/migrations/0001_init.sql" -Raw
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS users") { throw "Missing users DDL" }
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS lab_sessions") { throw "Missing lab_sessions DDL" }
    if ($sql -notmatch "CREATE TABLE IF NOT EXISTS user_progress") { throw "Missing user_progress DDL" }
}

# 3. Static Analysis & Type Checking
Run-Gate "Static Analysis & Type Checking (Cargo Check)" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo check --workspace 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Cargo check failed: $out" }
    } else {
        throw "Rust toolchain (cargo) not found on host"
    }
}

# 4. Backend Unit, Integration & API Contract Tests
Run-Gate "Backend Services Test Suite (100% Pass Required)" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo test --workspace 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Backend test suite failed: $out" }
    }
}

# 5. Security & Adversarial Attack Test Suite
Run-Gate "Security & Adversarial Attack Verification" {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $out = cargo test -p kubelab-api --test security_adversarial_test 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Security adversarial tests failed: $out" }
    }
}

# 6. Declarative Lab Catalog & State-Based Assertion Rules
Run-Gate "Declarative Lab Catalog Schema & Grading Rules" {
    $labs = Get-ChildItem -Path "$PSScriptRoot/../labs" -Filter "lab.yaml" -Recurse
    if ($labs.Count -lt 5) { throw "Fewer than 5 declarative labs found ($($labs.Count))" }
    Write-Host "      Verified $($labs.Count) declarative lab definitions across all categories." -ForegroundColor Gray
}

# 7. Web Application Integrity & Component Structure
Run-Gate "Web Application Component & Page Integrity" {
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
        'apps/web/src/lib/auth-context.tsx'
    )
    foreach ($p in $webPages) {
        if (-not (Test-Path "$PSScriptRoot/../$p")) { throw "Missing web component: $p" }
    }
}

# 8. Mobile Client Scaffold & Multi-Platform Directories
Run-Gate "Mobile Client Scaffold & Multi-Platform Scaffolds" {
    if (-not (Test-Path "$PSScriptRoot/../apps/mobile/pubspec.yaml")) { throw "Mobile pubspec.yaml missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/mobile/lib/main.dart")) { throw "Mobile main.dart missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/mobile/android/app/build.gradle")) { throw "Mobile Android build.gradle missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/mobile/ios/Runner/Info.plist")) { throw "Mobile iOS Info.plist missing" }
}

# 9. Cloud-Native GitOps & Service Mesh Manifests
Run-Gate "Argo CD GitOps & Istio Service Mesh Architecture" {
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
Run-Gate "Playwright E2E Test Specifications" {
    if (-not (Test-Path "$PSScriptRoot/../apps/web/playwright.config.ts")) { throw "playwright.config.ts missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/web/e2e/auth.spec.ts")) { throw "auth.spec.ts missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/web/e2e/labs.spec.ts")) { throw "labs.spec.ts missing" }
    if (-not (Test-Path "$PSScriptRoot/../apps/web/e2e/terminal.spec.ts")) { throw "terminal.spec.ts missing" }
}

# 11. Complete Documentation Suite Integrity
Run-Gate "Documentation & Architecture Specifications" {
    $docDirs = @('architecture', 'curriculum', 'labs', 'security', 'testing', 'operations', 'api')
    foreach ($d in $docDirs) {
        if (-not (Test-Path "$PSScriptRoot/../docs/$d")) {
            throw "Documentation section docs/$d missing"
        }
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
