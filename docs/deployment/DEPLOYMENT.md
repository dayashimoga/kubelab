# Deployment Guide: Container & Kubernetes

## 1. Local Container Deployment (Podman)
```bash
./scripts/up.ps1   # Windows
./scripts/up.sh    # Linux / macOS
```

## 2. Production Kubernetes Deployment (Helm & Argo CD)

```bash
# 1. Add KubeLab Helm repository
helm repo add kubelab https://charts.kubelab.io
helm repo update

# 2. Deploy with custom values
helm upgrade --install kubelab kubelab/kubelab \
  --namespace kubelab-system --create-namespace \
  -f infrastructure/helm/values.production.yaml
```
