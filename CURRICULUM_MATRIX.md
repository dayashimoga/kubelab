# KubeLab Authoritative Curriculum Matrix & Production Coverage

## 15-Track Engineering Curriculum & Competency Matrix

| # | Track ID | Track Name | Level | Modules | Lessons / Labs | Question Bank | Architectural Focus | Alignment |
|---|---|---|---|---|---|---|---|---|
| 01 | `linux-containers` | Linux & Container Fundamentals | Beginner | 2 | 8 | 80 Qs | POSIX permissions, kernel namespaces, cgroups v2, rootless Podman, Quadlets, OCI runc | LFCS, Docker DCA |
| 02 | `kubernetes` | Kubernetes Core Architecture & Workloads | Beginner | 2 | 16 | 160 Qs | Pod lifecycle, InitContainers, native sidecars, ConfigMap/Secrets, rolling Deployments, StatefulSets | CKA, CKAD |
| 03 | `storage` | Storage & Persistent Volumes | Intermediate | 2 | 10 | 100 Qs | emptyDir, hostPath, dynamic CSI StorageClasses, volume expansion, VolumeSnapshots, subPath | CKA |
| 04 | `networking` | Cloud-Native Networking, CNI & Gateway API | Intermediate | 2 | 12 | 120 Qs | ClusterIP, NodePort, LoadBalancer, Ingress, NetworkPolicies, CoreDNS, Cilium eBPF, Gateway API | CKA, CKS |
| 05 | `helm-kustomize` | Packaging with Helm & Kustomize | Intermediate | 2 | 10 | 100 Qs | Helm chart scaffolding, Go templates, helpers, hooks, Kustomize overlays, generators, patches | CKAD |
| 06 | `administration` | Cluster Operations & Administration | Advanced | 2 | 10 | 100 Qs | kubeadm bootstrap, etcd backup/restore, zero-downtime upgrades, node drain, PKI cert rotation | CKA |
| 07 | `security` | Zero-Trust Kubernetes Security & RBAC | Intermediate | 2 | 13 | 130 Qs | RBAC Roles, TokenRequest, Pod Security Standards (Restricted), Falco, Trivy, Cosign, SealedSecrets | CKS |
| 08 | `gitops` | GitOps & Continuous Delivery with Argo CD | Intermediate | 2 | 10 | 100 Qs | Argo CD bootstrap, App-of-Apps, sync waves, drift self-healing, Argo Rollouts canary, Blue-Green | GitOps Certified |
| 09 | `service-mesh` | Service Mesh with Istio & Envoy Proxy | Advanced | 2 | 11 | 110 Qs | IstioOperator, Envoy injection, VirtualService routing, STRICT mTLS, outlier detection, EnvoyFilters | Istio ICA |
| 10 | `observability` | OpenTelemetry, Prometheus & Grafana | Advanced | 2 | 10 | 100 Qs | Prometheus Operator, ServiceMonitors, PromQL alerting, Loki log pipelines, Tempo distributed tracing | Prometheus PCA |
| 11 | `troubleshooting` | Production Troubleshooting & Break-Fix | Advanced | 2 | 10 | 100 Qs | CrashLoopBackOff, ImagePullBackOff, CPU starvation, OOMKill (Exit 137), CoreDNS failure, 503 cascades | CKA, CKAD |
| 12 | `sre-performance` | Site Reliability Engineering & SLOs | Advanced | 2 | 10 | 100 Qs | SLI/SLO formulation, multi-window burn rate alerts, VPA right-sizing, KEDA queue autoscaling, topologySpread | SRE Foundation |
| 13 | `platform-eng` | Platform Engineering & Multi-Cluster | Expert | 2 | 10 | 100 Qs | Custom Resource Definitions (CRDs), Crossplane XRDs, Cluster API (CAPI), vCluster, Backstage, Karpenter | Platform Eng |
| 14 | `incidents` | Production Incident Response & Chaos | Expert | 2 | 10 | 100 Qs | Live SEV-1 break-fix: CoreDNS loop crash, HPA thrashing, Ingress 503 cascade, etcd quorum loss | SRE Lead |
| 15 | `certification` | Real-World Exam & Certification Drills | Expert | 2 | 4 | 40 Qs | High-intensity timed drills for CKA, CKAD, CKS, and KCNA under deterministic state evaluation | CKA / CKAD / CKS |

## Authoritative Catalog Totals
- **Total Tracks**: 15
- **Total Modules**: 30
- **Total Lessons**: 154
- **Total Hands-On Labs**: 154 (100% uniquely authored)
- **Total Architecture & Process Diagrams**: 154 (100% unique Mermaid diagrams)
- **Total Assessment Questions**: 1,540 (≥10 categorized questions per lesson)
- **Full Client Parity**: Web (Monaco + xterm.js PTY) and Mobile (Flutter 7-tab Lab Workspace)
