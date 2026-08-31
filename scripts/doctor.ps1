#!/usr/bin/env pwsh
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "       KUBELAB SYSTEM DOCTOR           " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$allOk = $true

# 1. Check Podman
if (Get-Command podman -ErrorAction SilentlyContinue) {
    $podmanVer = (podman --version).Trim()
    Write-Host "[OK] Podman is installed: $podmanVer" -ForegroundColor Green
} else {
    Write-Host "[ERROR] Podman is NOT installed." -ForegroundColor Red
    $allOk = $false
}

# 2. Check Git
if (Get-Command git -ErrorAction SilentlyContinue) {
    $gitVer = (git --version).Trim()
    Write-Host "[OK] Git is installed: $gitVer" -ForegroundColor Green
} else {
    Write-Host "[ERROR] Git is NOT installed." -ForegroundColor Red
    $allOk = $false
}

# 3. Check Compose Provider
if (Get-Command docker-compose, podman-compose -ErrorAction SilentlyContinue) {
    Write-Host "[OK] Container Compose provider is available." -ForegroundColor Green
} else {
    Write-Host "[WARN] Native compose command not found; Podman compose fallback will be used." -ForegroundColor Yellow
}

# 4. Check Optional Native Tools (nice to have for local host dev)
$optTools = @('cargo', 'rustc', 'node', 'kubectl', 'helm', 'flutter', 'python')
foreach ($tool in $optTools) {
    if (Get-Command $tool -ErrorAction SilentlyContinue) {
        Write-Host "[INFO] Host optional tool found: $tool" -ForegroundColor Gray
    }
}

Write-Host "========================================" -ForegroundColor Cyan
if ($allOk) {
    Write-Host "System check PASSED! Ready to run KubeLab." -ForegroundColor Green
    exit 0
} else {
    Write-Host "System check FAILED! Please install required tools." -ForegroundColor Red
    exit 1
}
