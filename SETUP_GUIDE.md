# KubeLab — Zero-Host-Install Setup & Deployment Guide

## 1. Prerequisites (Zero Host Dependencies)
The only requirement on the host machine is **Podman** (or Docker):
- **Podman** (version 4.x or 5.x) with Podman Machine running on Windows/macOS/Linux.
- Zero local dependencies on Node.js, npm, Rust, Cargo, Python, Flutter, or PostgreSQL required! Everything runs in isolated OCI containers.

---

## 2. Quick Start (One-Command Platform Startup)

### Starting the Platform
```powershell
# Windows PowerShell
.\scripts\up.ps1

# Linux / macOS Bash
./scripts/up.sh
```

This automatically initializes and starts:
- **PostgreSQL 16** (`localhost:5432`)
- **Redis 7** (`localhost:6379`)
- **NATS 2.10 Event Bus** (`localhost:4222`)
- **OpenTelemetry Collector** (`localhost:4317` / `4318`)
- **Prometheus** (`localhost:9090`)
- **Grafana** (`localhost:3001`)

---

## 3. Provisioning Disposable Kubernetes Sandboxes
To start a disposable Kind cluster for live lab execution:
```powershell
.\scripts\lab-up.ps1 -ClusterName kubelab-cluster
```

---

## 4. Running Containerized Verification & Tests
To run all Rust workspace tests, live backing service integration tests, and lab schema validation inside the containerized toolchain:
```powershell
.\scripts\test-containerized.ps1
```

---

## 5. Clean Shutdown & Zero-Residue Purge

### Normal Teardown
```powershell
.\scripts\down.ps1
```

### Complete Zero-Residue Purge
Removes all containers, volumes, temp networks, artifacts, and disposable Kind clusters:
```powershell
.\scripts\clean.ps1
```
