#!/usr/bin/env pwsh
param (
    [Parameter(Mandatory=$true)]
    [string]$Path
)

Write-Host "Validating declarative lab schema: $Path" -ForegroundColor Cyan
if (-not (Test-Path $Path)) {
    Write-Host "[ERROR] Lab file not found: $Path" -ForegroundColor Red
    exit 1
}

# Run validator tool
cargo run -p kubelab-validation-engine --bin validate-lab-schema -- --path "$Path"
if ($LASTEXITCODE -eq 0) {
    Write-Host "[PASS] Lab schema is 100% valid." -ForegroundColor Green
} else {
    Write-Host "[FAIL] Lab schema validation failed." -ForegroundColor Red
    exit 1
}
