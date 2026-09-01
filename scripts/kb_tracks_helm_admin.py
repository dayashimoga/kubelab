#!/usr/bin/env python3
"""
Tracks 5 & 6: Helm/Kustomize (10 Labs) & Cluster Administration (10 Labs)
"""

def register_helm_and_admin(reg):
    # Track 5: Helm & Kustomize (10 labs)
    helm_labs = [
        ("helm-01-chart-structure-scaffold", "Helm Chart Layout & Chart.yaml Metadata", "Create production Helm charts with Chart.yaml, values.yaml, and template structures."),
        ("helm-02-values-override-templating", "Go Templating & Values Overrides in Helm", "Master Helm Go template syntax, default values, quote functions, and conditional blocks."),
        ("helm-03-named-templates-helpers", "Named Template Partials (_helpers.tpl) & Indentation", "Write reusable partial templates with define and include while maintaining strict YAML indentation."),
        ("helm-04-chart-dependencies-subcharts", "Subcharts, Chart Dependencies & Value Overrides", "Manage multi-tier microservice dependencies and pass global values to subcharts."),
        ("helm-05-hooks-lifecycle-post-install", "Helm Lifecycle Hooks (Pre-Install & Post-Upgrade)", "Execute database migration jobs and cleanup routines using helm.sh/hook annotations."),
        ("helm-06-kustomize-base-overlays", "Kustomize Bases & Multi-Environment Overlays", "Structure dry declarative Kubernetes manifests for dev, staging, and prod using Kustomize."),
        ("helm-07-kustomize-configmap-secret-generator", "ConfigMap & Secret Generators with Hash Suffixes", "Trigger automatic zero-downtime rolling updates when config values change using content hash suffixes."),
        ("helm-08-kustomize-json6902-patches", "JSON 6902 Strategic Merge Patches", "Apply granular surgical modifications to Kubernetes resources using JSON 6902 operations (add/replace/remove)."),
        ("helm-09-kustomize-components-composition", "Kustomize Components for Cross-Cutting Features", "Compose optional features (e.g. Istio sidecar injection, Prometheus monitoring) across overlays."),
        ("helm-10-helm-kustomize-post-renderer", "Combining Helm with Kustomize Post-Renderers", "Patch third-party Helm charts dynamically using kustomize as a post-renderer.")
    ]

    for lab_id, title, summary in helm_labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Source["Declarative Source: {title}"] --> Engine["Rendering Engine ({lab_id})"]
    Engine --> Manifests["Standard K8s Manifests ({lab_id}.yaml)"]
    Manifests --> APIServer["Kubernetes API Server (Applied State)"]""",
            f"""# {title}

Declarative packaging and templating enables repeatable, secure multi-environment application delivery across Kubernetes clusters.

## Architectural Rendering Pipeline

```mermaid
graph TD
    Source["Declarative Source (Helm Chart / Kustomize Base)"] --> Engine["Rendering Engine (helm template / kustomize build)"]
    Engine --> Manifests["Standard Kubernetes Manifests (YAML)"]
    Manifests --> APIServer["Kubernetes API Server (Applied State)"]
```

## Key Operational Commands

```bash
# Render and lint Helm templates locally
helm lint ./mychart && helm template myrelease ./mychart

# Build and preview Kustomize overlay manifests
kustomize build overlays/production
```

## Common Production Gotchas & Anti-Patterns

1. **YAML Indentation Errors in Helpers**: Using `template` instead of `include` with `nindent` causes broken YAML alignment.
2. **Missing Hook Delete Policies**: Omitting `helm.sh/hook-delete-policy: hook-succeeded` leaves completed migration pods cluttering namespaces.
3. **Hardcoded Overrides**: Modifying base manifests directly rather than applying overlays.

## Security & Reliability Best Practices

- **Validate Schema**: Add `values.schema.json` to Helm charts to validate configuration parameters on install.
- **Pin Chart Versions**: Always lock dependency chart versions with exact version pins in `Chart.yaml`.""",
            ["Using 'template' instead of 'include' with 'nindent' leading to invalid YAML indentation.",
             "Hardcoding environment secrets in values.yaml rather than using external secrets or sealed secrets.",
             "Modifying base resources directly instead of applying environment overlays in Kustomize."],
            "Always run `helm lint` and `kustomize build` in pre-commit CI hooks to catch templating syntax errors.")

    # Track 6: Administration (10 labs)
    admin_labs = [
        ("admin-01-kubeadm-bootstrap", "Bootstrapping Multi-Node Clusters with kubeadm", "Initialize control plane nodes with kubeadm init and join worker nodes with discovery tokens."),
        ("admin-02-etcd-backup-restore", "etcd Snapshot Backup & Disaster Recovery", "Perform live etcd database backups with etcdctl and restore snapshots during control-plane outages."),
        ("admin-03-control-plane-upgrade", "Zero-Downtime Control Plane & Kubelet Upgrades", "Execute step-by-step kubeadm, kubelet, and kubectl version upgrades following Kubernetes skew policies."),
        ("admin-04-node-drain-cordon", "Node Maintenance, Cordoning & Graceful Drain", "Safely evict workloads before hardware maintenance with kubectl cordon and drain."),
        ("admin-05-certificate-rotation", "Kubernetes PKI Certificate Expiration & Renewal", "Check API server and kubelet TLS certificate expiration and execute zero-downtime certificate renewal."),
        ("admin-06-static-pods-debug", "Static Pod Architecture & Kubelet Manifest Watcher", "Debug and recover control plane static pods in /etc/kubernetes/manifests."),
        ("admin-07-audit-logging-policy", "Kubernetes API Server Audit Policy Logging", "Configure structured security audit logging rules for sensitive API requests and authentication events."),
        ("admin-08-custom-scheduler", "Deploying & Using Custom Secondary Schedulers", "Deploy custom scheduler algorithms and route specialized workloads via schedulerName."),
        ("admin-09-api-priority-fairness", "API Priority and Fairness (APF) Rate Limiting", "Prevent API server starvation using FlowSchemas and PriorityLevelConfigurations."),
        ("admin-10-mutating-webhook", "Dynamic Admission Control & Mutating Webhooks", "Implement admission webhooks to intercept, mutate, and validate Kubernetes API requests.")
    ]

    for lab_id, title, summary in admin_labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Admin["Cluster Administrator: {title}"] --> APIServer["kube-apiserver ({lab_id})"]
    APIServer --> ETCD["etcd Clustered DB ({lab_id})"]
    APIServer --> Kubelet["Node Kubelet Daemon ({lab_id})"]
    Kubelet --> StaticPods["Static Manifests (/etc/kubernetes/manifests/{lab_id})"]""",
            f"""# {title}

Cluster administration encompasses core control-plane maintenance, disaster recovery, security auditing, and zero-downtime lifecycle operations.

## Architectural Overview

```mermaid
graph TD
    Admin["Cluster Administrator"] --> APIServer["kube-apiserver (TLS Protected)"]
    APIServer --> ETCD["etcd Clustered Key-Value Database"]
    APIServer --> Kubelet["Node Kubelet Daemon"]
    Kubelet --> StaticPods["Static Manifests (/etc/kubernetes/manifests)"]
```

## Key Operational Commands

```bash
# Save etcd snapshot
ETCDCTL_API=3 etcdctl snapshot save /var/backups/etcd-snapshot.db \\
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \\
  --cert=/etc/kubernetes/pki/etcd/server.crt \\
  --key=/etc/kubernetes/pki/etcd/server.key

# Check certificate expiration
kubeadm certs check-expiration
```

## Common Production Gotchas & Anti-Patterns

1. **Skipping Minor Version Steps in Upgrades**: Upgrading more than one minor version at a time (e.g. 1.28 -> 1.30 directly) causes control plane crashes.
2. **Draining Nodes without Disruption Budget Checks**: Force draining nodes when PDBs are violated, causing production outages.
3. **Restoring etcd Without Stopping API Server**: Attempting to restore etcd data-dir while kube-apiserver is active causes split-brain data corruption.

## Security & Reliability Best Practices

- **Automate Daily etcd Snapshots**: Schedule cron jobs to take encrypted etcd snapshots and store them offsite.
- **Audit TLS Certs Monthly**: Monitor certificate expiration metrics with Prometheus alerts.""",
            ["Attempting to skip minor versions during kubeadm upgrade (violating version skew policy).",
             "Forgetting to pass --ignore-daemonsets during node drain operations.",
             "Restoring etcd snapshot without shutting down kube-apiserver leading to data corruption."],
            "Always back up etcd before performing any cluster upgrade or control plane maintenance operation.")
