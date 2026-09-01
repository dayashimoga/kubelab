# KubeLab Curriculum Matrix

## 15-Track Engineering Curriculum & Competency Matrix

| Track ID | Track Name | Level | Labs Count | Core Competencies & Skills Covered | Key Technologies | Certification Alignment |
|---|---|---|---|---|---|---|
| `linux-containers` | Linux & Container Fundamentals | Beginner | 8 | Namespaces, cgroups, chroot, rootless OCI runtimes, multi-stage builds | Linux, Podman, OCI, seccomp | LFCS, Docker DCA |
| `kubernetes` | Kubernetes Core Architecture | Beginner-Int | 15 | Pods, Deployments, ReplicaSets, Services, ConfigMaps, Secrets, Namespaces, Probes, Ingress | Kubernetes, CoreDNS | CKA, CKAD |
| `storage` | Storage & Persistent Volumes | Intermediate | 8 | StorageClasses, PVCs, PVs, CSI drivers, VolumeSnapshots, dynamic provisioning | CSI, NFS, Local Path, Ceph | CKA |
| `networking` | Cloud-Native Networking | Intermediate | 13 | CNI plugins, ClusterIP, NodePort, LoadBalancer, Ingress, NetworkPolicies, CoreDNS, Calico | Calico, Flannel, CoreDNS, Envoy | CKA, CKS |
| `helm-kustomize` | Package Management & Templates | Intermediate | 8 | Helm charts, values, subcharts, hooks, Kustomize overlays, patches, bases | Helm 3, Kustomize | CKAD |
| `administration` | Cluster Operations & Administration | Advanced | 12 | kubeadm, node cordon/drain, etcd backup/restore, cluster upgrades, PKI/CA cert rotation | kubeadm, etcdctl, kubelet | CKA |
| `security` | Zero-Trust Kubernetes Security | Advanced | 13 | PSS/PSA restricted, RBAC least-privilege, admission controllers, Falco, Trivy, Kyverno | Kyverno, Falco, Trivy, OPA | CKS |
| `gitops` | GitOps & Continuous Delivery | Advanced | 11 | Argo CD, ApplicationSets, AppProject isolation, drift detection, self-healing, sync waves | Argo CD, GitOps | GitOps Certified |
| `service-mesh` | Service Mesh Architecture | Advanced | 11 | Istio, Envoy, STRICT mTLS, canary routing, circuit breaking, fault injection, telemetry | Istio, Envoy, mTLS | Istio ICA |
| `observability` | Observability & Telemetry | Intermediate | 10 | Prometheus metrics, Alertmanager, Grafana dashboards, Loki log aggregation, Tempo OTel tracing | OTel, Prometheus, Grafana, Loki, Tempo | Prometheus Certified |
| `troubleshooting` | Production Troubleshooting | Advanced | 10 | CrashLoopBackOff, OOMKilled, Pending pods, ImagePullBackOff, node NotReady, DNS failures | kubectl-debug, crictl, logs | CKA, CKAD |
| `sre-performance` | SRE, Autoscaling & Performance | Advanced | 8 | HPA, VPA, Cluster Autoscaler, PDB, QoS classes, latency profiling, resource optimization | HPA, VPA, k6, Prometheus | SRE Foundation |
| `platform-multicluster`| Platform & Multi-Cluster Mgmt | Expert | 7 | Cluster API, Submariner, GitOps multi-environment, virtual clusters, developer platforms | vcluster, Submariner, CAPI | Platform Eng |
| `incidents` | Incident Response & Chaos Engineering | Expert | 10 | Live fault injection, CoreDNS outage, PVC deadlock, auth breach, SLO breach, rapid recovery | Chaos Mesh, Litmus, OTel | SRE Lead |
| `certification` | Real-World Exam & Certification Drills | Mixed | 10 | Timed drills, multi-objective tasks, strict rubric grading, complex debugging | Full K8s ecosystem | CKA / CKAD / CKS |

## Total Catalog Statistics
- **Total Tracks**: 15
- **Total Declarative Labs**: 145
- **Total Incident Scenarios**: 10
- **Hands-On Coverage**: 100% Declarative YAML & Real K8s State Assertions
