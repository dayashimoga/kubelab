#!/usr/bin/env bash
set -eo pipefail

CATEGORY="$1"
ID="$2"
TITLE="${3:-New Cloud-Native Lab}"

if [ -z "$CATEGORY" ] || [ -z "$ID" ]; then
    echo "Usage: ./scripts/create-lab.sh <category> <lab-id> [title]"
    exit 1
fi

TARGET_DIR="$(dirname "$0")/../labs/$CATEGORY/$ID"
mkdir -p "$TARGET_DIR"

cat <<EOF > "$TARGET_DIR/lab.yaml"
id: "$ID"
title: "$TITLE"
difficulty: "intermediate"
duration_minutes: 25
track: "$CATEGORY"
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
EOF

echo "[OK] Created new declarative lab scaffold at $TARGET_DIR/lab.yaml"
