#!/usr/bin/env bash
set -eo pipefail

echo "================================================================="
echo "   KUBELAB ZERO-HOST-INSTALL CONTAINERIZED BUILD PIPELINE        "
echo "================================================================="

# 1. Build Toolchain
echo ""
echo "[1/3] Building kubelab-toolchain container..."
podman build -f infrastructure/containers/Containerfile.toolchain -t kubelab-toolchain .

# 2. Build Web Application
echo ""
echo "[2/3] Building production Next.js web application container..."
podman build -f infrastructure/containers/Containerfile.web -t kubelab-web .

# 3. Build Backend API Server
echo ""
echo "[3/3] Building production Rust API Gateway container..."
podman build -f infrastructure/containers/Containerfile.api -t kubelab-api .

echo ""
echo "================================================================="
echo "  ALL PRODUCTION CONTAINERS BUILT SUCCESSFULLY!                  "
echo "================================================================="
