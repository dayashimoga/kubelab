#!/usr/bin/env python3
"""
Tracks 7 & 8: Security (13 Labs) & GitOps with Argo CD (10 Labs)
"""

def register_sec_and_gitops(reg):
    # Track 7: Security (13 labs)
    sec_labs = [
        ("sec-01-rbac-role-binding", "Namespace RBAC Roles & RoleBindings", "Grant least-privilege API access within a single namespace using Roles and RoleBindings."),
        ("sec-02-clusterrole-clusterbinding", "Cluster-Wide RBAC ClusterRoles & Bindings", "Manage cluster-scoped authorizations for nodes, persistent volumes, and multi-namespace controllers."),
        ("sec-03-serviceaccount-token-projected", "ServiceAccount TokenRequest & Bound Projections", "Eliminate permanent ServiceAccount secret tokens by projecting short-lived, bound TokenRequest credentials."),
        ("sec-04-pod-security-admission", "Pod Security Standards (Privileged, Baseline, Restricted)", "Enforce cluster-wide Pod Security Standards labels: enforce, audit, and warn at Restricted levels."),
        ("sec-05-security-context-non-root", "Hardened SecurityContext & Read-Only Root Filesystems", "Lock down container runtimes with runAsNonRoot, allowPrivilegeEscalation: false, and read-only roots."),
        ("sec-06-seccomp-apparmor-profiles", "Seccomp & AppArmor System Call Filtering", "Restrict dangerous Linux system calls using RuntimeDefault and custom Localhost seccomp profiles."),
        ("sec-07-image-vulnerability-trivy", "Container Image Vulnerability Scanning with Trivy", "Scan OCI container images in CI and block high/critical CVEs before cluster deployment."),
        ("sec-08-opa-gatekeeper-constraint", "Policy-as-Code with OPA Gatekeeper & Rego", "Define declarative admission guardrails using ConstraintTemplates and Rego validation rules."),
        ("sec-09-kyverno-policy-validation", "Kubernetes Native Policy Validation with Kyverno", "Enforce best practices, mutations, and image verification using native Kyverno ClusterPolicies."),
        ("sec-10-falco-runtime-security", "Runtime Threat Detection with Falco & eBPF", "Detect container breakouts, terminal spawning, and privilege escalation using Falco kernel audit rules."),
        ("sec-11-kube-bench-cis-hardening", "CIS Kubernetes Benchmark Hardening with kube-bench", "Audit control plane and worker node configurations against the official CIS Benchmark."),
        ("sec-12-cosign-image-signing", "Cryptographic Image Signing & Verification with Cosign", "Sign container images with Sigstore Cosign and enforce signature validation in admission webhooks."),
        ("sec-13-sealed-secrets-gitops", "Encrypted Secret Management with Bitnami SealedSecrets", "Safely commit encrypted SealedSecret manifests to public Git repositories.")
    ]

    for lab_id, title, summary in sec_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    Subject["User / ServiceAccount"] --> Request["API Server Request"]
    Request --> AuthZ["RBAC / Admission Policy ({lab_id})"]
    AuthZ -->|Allowed| K8sResource["Cluster Resources"]
    AuthZ -.->|Denied 403| Audit["Security Audit Log"]""",
            f"""# {title}

Zero-trust Kubernetes security combines least-privilege authorization, runtime containment, admission policies, and cryptographic verification.

## Architectural Security Model

```mermaid
graph LR
    Subject["User / ServiceAccount"] --> Request["API Server Request"]
    Request --> AuthZ["RBAC / Admission Policy ({lab_id})"]
    AuthZ -->|Allowed| K8sResource["Cluster Resources"]
    AuthZ -.->|Denied 403| Audit["Security Audit Log"]
```

## Key Operational Commands

```bash
# Check if current user can perform an action
kubectl auth can-i create deployments -n production

# Test API permissions as a specific ServiceAccount
kubectl auth can-i list secrets --as=system:serviceaccount:default:my-sa
```

## Common Production Gotchas & Anti-Patterns

1. **Using Wildcard Verbs in RBAC**: Setting `verbs: ["*"]` or `resources: ["*"]` grants dangerous administrative rights.
2. **Mounting ServiceAccount Tokens Unconditionally**: Leaving `automountServiceAccountToken: true` on pods that don't call the Kubernetes API.
3. **Running Containers as Root**: Not specifying `runAsNonRoot: true` in production security contexts.

## Security & Reliability Best Practices

- **Enforce Pod Security Standards 'Restricted'**: Block privilege escalation, root execution, and host namespace mounts.
- **Audit RBAC Roles Quarterly**: Remove unused service accounts and expired role bindings.""",
            ["Granting wildcard '*' verbs or resources in production RBAC roles.",
             "Leaving automountServiceAccountToken: true on workloads that do not interact with the Kubernetes API.",
             "Omitting allowPrivilegeEscalation: false in container security contexts."],
            "Enforce Pod Security Standards 'Restricted' and disable automatic ServiceAccount token mounting.")

    # Track 8: GitOps (10 labs)
    gitops_labs = [
        ("gitops-01-argocd-bootstrap", "Bootstrapping Argo CD in Kubernetes", "Install Argo CD control plane and configure declarative GitOps repositories."),
        ("gitops-02-app-of-apps-pattern", "Enterprise App-of-Apps Architecture", "Manage multi-cluster application fleets hierarchically using the App-of-Apps design pattern."),
        ("gitops-03-automated-sync-prune", "Automated Synchronization, Self-Healing & Pruning", "Enable automated sync policies with self-heal and resource pruning for drift prevention."),
        ("gitops-04-sync-waves-hooks", "Argo CD Sync Waves & Phased Migration Hooks", "Sequence multi-resource deployments in ordered sync waves (-1 to 5) with pre/post-sync hooks."),
        ("gitops-05-drift-detection-remediation", "Cluster Drift Detection & Automated Reconciliation", "Detect manual kubectl modifications and automatically reconcile cluster state to match Git."),
        ("gitops-06-argo-rollouts-canary", "Progressive Delivery with Argo Rollouts Canary", "Deploy zero-downtime canary releases with dynamic traffic shifting and automated promotions."),
        ("gitops-07-analysis-template-prometheus", "Automated Canary Metric Analysis with Prometheus", "Halt and roll back deployments automatically when Prometheus error rate metrics exceed thresholds."),
        ("gitops-08-blue-green-active-preview", "Blue-Green Deployments with Preview Services", "Execute Blue-Green deployments with active/preview service switching and manual gate approval."),
        ("gitops-09-flux-kustomization-controller", "GitOps Automation with Flux CD & Source Controller", "Manage continuous deployment pipelines using Flux GitRepository and Kustomization controllers."),
        ("gitops-10-multi-cluster-argocd", "Multi-Cluster Fleet Management with Argo CD", "Register remote Kubernetes clusters and deploy applications across multiple regions from a central GitOps hub.")
    ]

    for lab_id, title, summary in gitops_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    GitRepo["Git Repository (Single Source of Truth)"] -->|Poll / Webhook| ArgoCD["Argo CD Controller ({lab_id})"]
    ArgoCD -->|Reconciles State| TargetCluster["Target Kubernetes Cluster"]
    TargetCluster -.->|Drift Detected| ArgoCD""",
            f"""# {title}

GitOps establishes Git as the single source of truth for declared infrastructure and application state, automating deployment and reconciliation.

## Architectural GitOps Pipeline

```mermaid
graph LR
    GitRepo["Git Repository (Single Source of Truth)"] -->|Poll / Webhook| ArgoCD["Argo CD Controller ({lab_id})"]
    ArgoCD -->|Reconciles State| TargetCluster["Target Kubernetes Cluster"]
    TargetCluster -.->|Drift Detected| ArgoCD
```

## Key Operational Commands

```bash
# Check Argo CD application sync status
argocd app get my-app

# Trigger manual synchronization
argocd app sync my-app --prune
```

## Common Production Gotchas & Anti-Patterns

1. **Manual Hotfixes in Cluster**: Making manual `kubectl edit` changes that get overwritten by GitOps self-healing.
2. **Missing Sync Waves for Migrations**: Running database migrations concurrently with new web pods before database schemas are ready.
3. **Disabling Self-Heal**: Permitting configuration drift to persist silently in production environments.

## Security & Reliability Best Practices

- **Enable Automated Self-Healing**: Ensure `selfHeal: true` and `prune: true` are enabled on all production applications.
- **Implement Metric-Driven Analysis**: Use Argo Rollouts AnalysisTemplates to guard promotions with real Prometheus SLI metrics.""",
            ["Making manual cluster edits without committing to Git, causing unexpected rollbacks upon sync.",
             "Omitting sync waves, causing web applications to start before database migrations finish.",
             "Hardcoding secrets in Git repositories without sealed secrets or external vault operators."],
            "Adopt the App-of-Apps pattern with automated self-healing and Prometheus analysis templates.")
