# KubeLab — Developer Guide & Contribution Standards

## 1. Local Development Principles
- **No Local Host Tools Required**: You can develop, test, build, and certify KubeLab using only Podman (or Docker).
- **Rust Monorepo**: All backend microservices live under `/services` and shared packages under `/packages`.
- **Next.js Web Frontend**: Located under `/apps/web`, uses React 18, TailwindCSS, Monaco Editor, and xterm.js.
- **Flutter Mobile App**: Located under `/apps/mobile`, cross-platform client for iOS and Android.

---

## 2. Running Services Locally
Start backing services (PostgreSQL, Redis, NATS, OTel):
```powershell
.\scripts\up.ps1
```

Run backend services with Cargo:
```bash
cargo run -p kubelab-api
```

Run containerized test suite:
```powershell
.\scripts\test-containerized.ps1
```

---

## 3. Authoring Declarative Lab Definitions
Every lab is defined as a YAML manifest under `labs/<track>/<lab-id>/lab.yaml`.

### Lab Schema Structure
```yaml
id: k8s-pod-basics
title: Create and Configure Your First Pod
difficulty: beginner
duration_minutes: 15
track: kubernetes
objectives:
  - Deploy pod
scenario: Deploy an nginx web container named web-server on port 80.
tasks:
  - id: task-deploy-pod
    title: Deploy the Pod
    description: Create pod web-server running nginx:alpine
    points: 50
    validation:
      type: k8s_resource
      resource: pods
      name: web-server
      assertions:
        - field: status.phase
          operator: equals
          expected: Running
        - field: metadata.labels.app
          operator: equals
          expected: frontend
hints:
  - text: "Use: kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend"
    penalty_points: 10
solution: "kubectl run web-server --image=nginx:alpine --port=80 --labels=app=frontend --restart=Always"
```

To validate all 145 lab definitions:
```bash
cargo run -p kubelab-validation-engine --bin validate_lab_schema
```
