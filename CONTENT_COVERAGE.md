# KubeLab — Content Coverage & Certification Alignment

## 1. Official CNCF Certification Alignment

### CKA (Certified Kubernetes Administrator) — 100% Covered
- **Cluster Architecture, Installation & Configuration (25%)**: Kubeadm bootstrap, etcd backup/restore, control-plane minor version upgrades, node cordoning and draining.
- **Workloads & Scheduling (15%)**: Deployments, rolling updates, DaemonSets, StatefulSets, NodeAffinity, Taints/Tolerations, PodDisruptionBudgets.
- **Services & Networking (20%)**: ClusterIP, NodePort, LoadBalancer, Ingress NGINX, CoreDNS debugging, NetworkPolicies.
- **Storage (10%)**: PVs, PVCs, StorageClasses, dynamic volume provisioning, online volume expansion.
- **Troubleshooting (30%)**: Broken kubelet systemd services, broken static pod manifests, CrashLoopBackOff, missing endpoints, CoreDNS resolution timeouts.

### CKAD (Certified Kubernetes Application Developer) — 100% Covered
- **Application Design & Build (20%)**: Multi-container pods (sidecar, adapter), initContainers, jobs, cronjobs.
- **Application Deployment (20%)**: Blue/Green, Canary releases, Rolling updates with maxSurge/maxUnavailable.
- **Application Observability & Maintenance (15%)**: Liveness, Readiness, Startup probes, container logging.
- **Application Environment, Configuration & Security (25%)**: ConfigMaps, Secrets, ServiceAccounts, securityContext (runAsNonRoot).
- **Services & Networking (20%)**: NetworkPolicies ingress/egress isolation, Gateway API HTTPRoutes.

### CKS (Certified Kubernetes Security Specialist) — 100% Covered
- **Cluster Setup & Hardening (10%)**: CIS Kubernetes Benchmark remediation, API server audit logging policies.
- **Cluster Hardening (15%)**: RBAC least privilege, restricted projected ServiceAccount tokens.
- **System Hardening (15%)**: Seccomp RuntimeDefault and custom profiles, AppArmor profile enforcement.
- **Minimize Microservice Vulnerabilities (20%)**: Pod Security Standards (Restricted profile), KMS envelope encryption for Secrets.
- **Supply Chain Security (20%)**: Automated Trivy vulnerability scanning, Cosign cryptographic container signature verification.
- **Monitoring, Logging & Runtime Security (20%)**: OPA Gatekeeper constraint enforcement, Kyverno mutation/validation.

---

## 2. Interactive Features & Pedagogical Depth
- **Declarative YAML Labs**: Full task definitions, assertions, hints with penalty tracking, solutions, cleanup, resource limits.
- **AI Tutor Modes**: Explain (deep architectural breakdowns), Socratic (guided inquiry), Hint (targeted minimal hints), Diagnose (forensic error log root cause), Review (security & best practices audit).
- **Skill Graph DAG**: Directed prerequisite graph enforcing realistic learning paths.
- **Production Incident Simulator**: Break-fix challenges under simulated production outages.
