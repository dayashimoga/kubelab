#!/usr/bin/env python3
"""
Tracks 13 & 14: Platform Engineering (10 Labs) & Incident Response (10 Labs)
"""

def register_platform_and_incidents(reg):
    # Track 13: Platform Engineering (10 labs)
    platform_labs = [
        ("platform-01-custom-resource-definition", "Designing Custom Resource Definitions (CRDs)", "Author OpenAPI v3 validated CRDs with status subresources and kubectl printer columns."),
        ("platform-02-crossplane-xrd-composition", "Cloud Infrastructure Composition with Crossplane", "Compose AWS/GCP managed databases and networks into clean declarative Kubernetes API abstractions."),
        ("platform-03-cluster-api-aws-provision", "Declarative Fleet Provisioning with Cluster API (CAPI)", "Manage the lifecycle of Kubernetes clusters declaratively as custom resources."),
        ("platform-04-vcluster-virtual-tenancy", "Multi-Tenancy Isolation with vCluster", "Spin up lightweight, fully isolated virtual Kubernetes clusters inside shared host namespaces."),
        ("platform-05-backstage-software-template", "Internal Developer Platforms & Backstage Templates", "Create self-service golden paths for engineering teams using Backstage software templates."),
        ("platform-06-kargo-stage-promotion", "Multi-Stage Artifact Promotion with Kargo", "Automate progressive GitOps stage promotion across development, staging, and production."),
        ("platform-07-external-secrets-operator", "Synchronizing Cloud Vaults with External Secrets Operator", "Sync secrets automatically from AWS Secrets Manager and HashiCorp Vault into Kubernetes."),
        ("platform-08-karpenter-node-autoscaling", "Fast Just-in-Time Node Autoscaling with Karpenter", "Bin-pack and provision right-sized cloud instances in seconds based on pending pod requirements."),
        ("platform-09-telepresence-local-intercept", "Rapid Local Development Loops with Telepresence", "Intercept remote cluster microservice traffic and route it to local developer workstations."),
        ("platform-10-devspace-rapid-inner-loop", "Hot-Reload Container Workflows with DevSpace", "Sync source code and hot-reload code directly inside running Kubernetes pods without rebuilding images.")
    ]

    for lab_id, title, summary in platform_labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Developer["Product Engineering Team"] --> IDP["Internal Developer Platform ({lab_id})"]
    IDP --> CRD["Declarative API Abstraction (XRD / CRD)"]
    CRD --> CloudInfra["Cloud Infrastructure (K8s / Databases / VPCs)"]""",
            f"""# {title}

Platform engineering builds self-service internal developer platforms (IDPs) and custom control planes that accelerate product delivery while enforcing compliance.

## Architectural Platform Abstraction

```mermaid
graph TD
    Developer["Product Engineering Team"] --> IDP["Internal Developer Platform ({lab_id})"]
    IDP --> CRD["Declarative API Abstraction (XRD / CRD)"]
    CRD --> CloudInfra["Cloud Infrastructure (K8s / Databases / VPCs)"]
```

## Key Operational Commands

```bash
# Verify custom resource definitions and status
kubectl get crd -l app.kubernetes.io/name={lab_id}

# Inspect composite resource composition
kubectl describe xrd
```

## Common Production Gotchas & Anti-Patterns

1. **Unvalidated CRD Schemas**: Deploying CRDs without complete OpenAPI v3 validation, allowing invalid fields to crash controllers.
2. **Missing Controller Leader Election**: Running multiple controller replicas without leader election, causing split-brain resource mutations.
3. **Hardcoding Cloud Account IDs**: Leaking cloud credentials or environment IDs into reusable platform templates.

## Security & Reliability Best Practices

- **Adopt Least-Privilege Controller Roles**: Ensure custom operators only hold RBAC permissions for the resources they reconcile.
- **Implement Finalizers Gracefully**: Handle resource deletion finalizers to avoid stuck namespaces during teardown.""",
            ["Deploying CRDs without OpenAPI v3 validation schemas, allowing corrupted states in etcd.",
             "Running multiple operator replicas without enabling leader election.",
             "Failing to clean up custom resource finalizers, causing namespaces to hang in Terminating state."],
            "Design platform abstractions around clear ownership, self-service golden paths, and automated policy validation.")

    # Track 14: Incidents (10 labs)
    incident_labs = [
        ("incident-coredns-failure", "Production Incident: CoreDNS Outage & Cascading 503 Errors", "SEV-1 War Room: Diagnose CoreDNS loop crashback, repair upstream forwarders, and restore cluster DNS."),
        ("incident-hpa-thrashing", "Production Incident: HPA Thrashing & Cascade Scaling", "SEV-1 War Room: Resolve flapping pod autoscaling caused by unbuffered memory threshold spikes."),
        ("incident-ingress-503-outage", "Production Incident: Ingress Gateway 503 Outage", "SEV-1 War Room: Troubleshoot Ingress controller proxy timeout cascade and unready backend pods."),
        ("incident-pvc-deadlock-multiattach", "Production Incident: Multi-Attach Storage Deadlock", "SEV-1 War Room: Break volume attachment locks after sudden worker node kernel panic."),
        ("incident-etcd-quorum-loss", "Production Incident: etcd Split-Brain & Quorum Loss", "SEV-1 War Room: Recover from etcd network partition and restore cluster consensus."),
        ("incident-cert-expiration-outage", "Production Incident: Expired Control Plane TLS Certificates", "SEV-1 War Room: Renew expired kube-apiserver certificates and restore API connectivity."),
        ("incident-oom-cascade-starvation", "Production Incident: Unbounded OOM Cascade Node Collapse", "SEV-1 War Room: Mitigate noisy-neighbor memory leak taking down shared worker nodes."),
        ("incident-gitops-sync-jam", "Production Incident: GitOps Sync Deadlock & Immutable Fields", "SEV-1 War Room: Resolve blocked Argo CD sync storms caused by immutable field modification."),
        ("incident-istio-mtls-breakage", "Production Incident: Broken STRICT mTLS Communication", "SEV-1 War Room: Triage broken inter-service communication following misapplied PeerAuthentication."),
        ("incident-cni-ip-exhaustion", "Production Incident: CNI IPAM Pool Exhaustion Outage", "SEV-1 War Room: Fix pods stuck in ContainerCreating caused by exhausted Calico IPAM subnets.")
    ]

    for lab_id, title, summary in incident_labs:
        reg(lab_id, title, summary,
            f"""sequenceDiagram
    participant OnCall as On-Call Incident Commander
    participant Cluster as Production Cluster (SEV-1 Outage)
    participant Postmortem as Incident Postmortem & Blameless RCA
    OnCall->>Cluster: Triage Alerts, Identify Root Cause ({lab_id})
    OnCall->>Cluster: Execute Surgical Remediation
    Cluster-->>OnCall: Cluster Health Restored (Metrics Nominal)
    OnCall->>Postmortem: Document Prevention Guardrails & Tests""",
            f"""# {title}

**SEV-1 Production Incident Simulation**: This high-severity scenario tests your live diagnostic and remediation skills under time pressure.

## Incident Workflow & Recovery

```mermaid
sequenceDiagram
    participant OnCall as On-Call Incident Commander
    participant Cluster as Production Cluster (SEV-1 Outage)
    participant Postmortem as Incident Postmortem & Blameless RCA
    OnCall->>Cluster: Triage Alerts, Identify Root Cause ({lab_id})
    OnCall->>Cluster: Execute Surgical Remediation
    Cluster-->>OnCall: Cluster Health Restored (Metrics Nominal)
    OnCall->>Postmortem: Document Prevention Guardrails & Tests
```

## Incident Triage Protocol

1. **Assess Impact**: Check user-facing error rates (`rate(http_requests_total[5m])`).
2. **Isolate Component**: Inspect cluster events and component logs (`kubectl get events -n kube-system`).
3. **Execute Remediation**: Apply the minimal surgical fix to restore cluster state convergence.
4. **Verify Convergence**: Confirm all readiness probes pass and error rates drop to zero.

## Key Operational Commands

```bash
# Check critical system pods status
kubectl get pods -n kube-system -o wide

# Review live system error logs
kubectl logs -n kube-system -l k8s-app=kube-dns --tail=100
```

## Common Production Gotchas & Anti-Patterns

1. **Panic Restarts**: Deleting pods indiscriminately before capturing diagnostic logs and event states.
2. **Untested Emergency Patches**: Applying unvalidated manifests during high-stress incidents.
3. **Ignoring Root Cause**: Resolving the immediate symptom without addressing the configuration defect.

## Security & Reliability Best Practices

- **Blameless Postmortems**: Document technical timeline, contributing factors, and preventive action items.
- **Automated Regression Testing**: Encode the failure scenario as an automated CI test.""",
            ["Executing destructive node deletions without capturing diagnostic logs first.",
             "Applying untested emergency patches that compound the cascade failure.",
             "Closing the incident before verifying long-term state stability."],
            "Follow structured incident response protocols: Assess Impact -> Triage -> Remediate -> Verify -> Blameless RCA.")
