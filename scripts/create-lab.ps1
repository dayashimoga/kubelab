#!/usr/bin/env pwsh
param (
    [Parameter(Mandatory=$true)]
    [string]$Category,
    [Parameter(Mandatory=$true)]
    [string]$Id,
    [string]$Title = "New Cloud-Native Lab"
)

$targetDir = "$PSScriptRoot/../labs/$Category/$Id"
if (Test-Path $targetDir) {
    Write-Host "[ERROR] Lab $Id already exists in category $Category" -ForegroundColor Red
    exit 1
}

New-Item -ItemType Directory -Path $targetDir -Force | Out-Null

$labYaml = @"
id: "$Id"
title: "$Title"
difficulty: "intermediate"
duration_minutes: 25
track: "$Category"
prerequisites:
  - "k8s-foundations"
environment:
  type: "kubernetes"
  cluster: "disposable"
  namespace_isolation: true
  resources:
    cpu_limit: "1000m"
    memory_limit: "1024Mi"
initial_state:
  manifests: []
scenario: |
  In this lab, you will demonstrate hands-on cloud-native skills by completing the requested task in a live environment.
tasks:
  - id: "task-1"
    title: "Primary Objective"
    description: "Deploy and configure the requested Kubernetes resource."
    points: 100
    validation:
      type: "k8s_resource"
      resource: "pods"
      assertions:
        - field: "status.phase"
          operator: "equals"
          expected: "Running"
hints:
  - text: "Check pod logs and describe events if pod does not enter Running state."
    penalty_points: 10
solution: |
  kubectl run sample-pod --image=nginx:alpine --restart=Always
cleanup:
  auto: true
limits:
  max_attempts: 5
  timeout_minutes: 30
"@

Set-Content -Path "$targetDir/lab.yaml" -Value $labYaml
Write-Host "[OK] Created new declarative lab scaffold at $targetDir/lab.yaml" -ForegroundColor Green
