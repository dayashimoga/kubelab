# KubeLab Declarative Lab Certification

## 1. Certified Lab Definitions

All declarative labs in `labs/` have been validated against the JSON Schema and state-based assertion rules.

| Lab ID | Track | Difficulty | Tasks | Assertions Verified | Certification |
|---|---|---|---|---|---|
| `k8s-pod-basics` | Kubernetes Core | Beginner | 2 | `status.phase == "Running"`, `ports[0].containerPort == 80` | **CERTIFIED** |
| `k8s-deployments-scaling` | Kubernetes Core | Intermediate | 2 | `status.readyReplicas == 3`, `spec.strategy.type == "RollingUpdate"` | **CERTIFIED** |
| `k8s-services-clusterip` | Networking | Beginner | 1 | `spec.type == "ClusterIP"`, `spec.ports[0].port == 80` | **CERTIFIED** |
| `k8s-rbac-role-binding` | Security | Intermediate | 2 | `rules[0].verbs includes "get"`, `roleRef.name == "developer-role"` | **CERTIFIED** |
| `gitops-argocd-drift` | GitOps | Intermediate | 1 | `spec.syncPolicy.automated.selfHeal == true` | **CERTIFIED** |
| `mesh-istio-canary` | Service Mesh | Advanced | 1 | `spec.http[0].route[0].weight == 90` | **CERTIFIED** |
| `incident-coredns-failure` | SRE / Incidents | Expert | 2 | `status.phase == "Running"`, `dns_resolution == "healthy"` | **CERTIFIED** |

---

## 2. Lab Lifecycle Compliance

All labs adhere to the strict 8-stage state machine:
`PROVISION` ➔ `BOOTSTRAP` ➔ `READY` ➔ `LEARN` ➔ `VALIDATE` ➔ `SCORE` ➔ `CLEANUP` ➔ `DESTROY`.
