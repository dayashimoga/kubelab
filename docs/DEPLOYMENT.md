# KubeLab Deployment

## Container Images

| Image | Containerfile | Base |
|---|---|---|
| `kubelab/api` | `infrastructure/containers/Containerfile.api` | `rust:latest` → `debian:bookworm-slim` |
| `kubelab/web` | `infrastructure/containers/Containerfile.web` | `node:22-alpine` |
| `kubelab/toolchain` | `infrastructure/containers/Containerfile.toolchain` | `debian:bookworm-slim` |

## Local Deployment (Podman Compose)

```bash
podman-compose -f infrastructure/containers/podman-compose.yml up -d
```

## Kubernetes Deployment

### ArgoCD (GitOps)
Application manifests in `infrastructure/gitops/argocd/`:
- `root-app.yaml` — ArgoCD Application of Applications

### Istio Service Mesh
Manifests in `infrastructure/mesh/istio/`:
- `virtualservice-canary.yaml` — Canary traffic routing

### Kind Cluster (Development)
```bash
./scripts/k8s-up.ps1   # Create disposable Kind cluster
./scripts/k8s-down.ps1 # Teardown with zero residue
```

## Helm (Future)
Helm chart packaging is a planned capability. Currently deployment uses raw manifests and ArgoCD.
