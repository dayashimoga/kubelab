#!/usr/bin/env bash
set -eo pipefail

PATH_TO_LAB="$1"
if [ -z "$PATH_TO_LAB" ] || [ ! -f "$PATH_TO_LAB" ]; then
    echo "[ERROR] Usage: ./scripts/test-lab.sh <path-to-lab.yaml>"
    exit 1
fi

echo "Validating declarative lab schema: $PATH_TO_LAB"
cargo run -p kubelab-validation-engine --bin validate-lab-schema -- --path "$PATH_TO_LAB"
echo "[PASS] Lab schema is 100% valid."
