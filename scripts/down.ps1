#!/usr/bin/env pwsh
Write-Host "Stopping KubeLab services and tearing down containers..." -ForegroundColor Yellow
$composeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.yml"
$testComposeFile = "$PSScriptRoot/../infrastructure/containers/podman-compose.test.yml"

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose -f $composeFile down --remove-orphans 2>$null
    podman compose -f $testComposeFile down --remove-orphans 2>$null
} elseif (Get-Command docker-compose -ErrorAction SilentlyContinue) {
    docker-compose -f $composeFile down --remove-orphans 2>$null
    docker-compose -f $testComposeFile down --remove-orphans 2>$null
}

# Verify Zero-Residue State
Write-Host "Verifying zero running containers for project..." -ForegroundColor Gray
if (Get-Command podman -ErrorAction SilentlyContinue) {
    $active = podman ps --filter "name=kubelab-" --format "{{.Names}}" 2>$null
    if ($active) {
        Write-Host "[WARNING] Remaining containers detected: $active. Force stopping..." -ForegroundColor Yellow
        podman stop -t 2 $active 2>$null
        podman rm -f $active 2>$null
    }
}

Write-Host "KubeLab services cleanly stopped with zero residue." -ForegroundColor Green
