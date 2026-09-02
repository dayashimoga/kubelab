#!/usr/bin/env python3
"""
Track 15: Exam & Certification Drills (4 Labs)
"""

def register_cert_track(reg):
    cert_labs = [
        ("cert-01-cka-speed-drill", "CKA Comprehensive Administration Speed Drill", "Complete multi-objective tasks covering RBAC, NetworkPolicy, etcd backup, Ingress routing, and node maintenance under strict time constraints."),
        ("cert-02-ckad-speed-drill", "CKAD Application Developer Speed Drill", "Build multi-container pods, configure Canary deployments, author CronJobs, and manage ConfigMaps under time pressure."),
        ("cert-03-cks-speed-drill", "CKS Security Specialist Speed Drill", "Enforce Pod Security Standards, write Falco security rules, configure seccomp profiles, and audit cluster benchmarks."),
        ("cert-04-kcna-comprehensive-drill", "KCNA Cloud Native Associate Speed Drill", "Test holistic understanding of CNCF architecture, Kubernetes core primitives, GitOps, and observability.")
    ]

    for lab_id, title, summary in cert_labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Exam["Certification Candidate: {title}"] --> Task1["Objective 1: Core Tasks ({lab_id})"]
    Exam --> Task2["Objective 2: Verification ({lab_id})"]
    Exam --> Task3["Objective 3: Reliability & Security ({lab_id})"]
    Task1 --> Evaluator["Deterministic Evaluator ({lab_id})"]
    Task2 --> Evaluator
    Task3 --> Evaluator
    Evaluator --> Result["Certification Score (80% Threshold)"]""",
            f"""# {title}

**Official Certification Speed Drill**: This hands-on scenario evaluates practical exam competency under timed, multi-objective conditions.

## Exam Architecture & Evaluation

```mermaid
graph TD
    Exam["Certification Candidate (Timed Drill)"] --> Task1["Objective 1: RBAC & Security"]
    Exam --> Task2["Objective 2: Core Workloads & Networking"]
    Exam --> Task3["Objective 3: Troubleshooting & Cluster Ops"]
    Task1 --> Evaluator["Deterministic Automated Exam Evaluator"]
    Task2 --> Evaluator
    Task3 --> Evaluator
    Evaluator --> Result["Certification Score (Passing Threshold: 80%)"]
```

## Timed Exam Strategy & Tips

1. **Use Imperative Commands to Generate YAML**: `kubectl run ... --dry-run=client -o yaml > pod.yaml`
2. **Set Kubectl Aliases**: `alias k=kubectl`, `export do="--dry-run=client -o yaml"`
3. **Verify Every Task**: Run `kubectl get` and test connectivity before moving to the next objective.

## Key Operational Commands

```bash
# Fast YAML generation for Deployments
kubectl create deployment web --image=nginx:alpine $do > deploy.yaml

# Fast Service creation
kubectl expose deployment web --port=80 --target-port=8080 $do > svc.yaml
```

## Common Production Gotchas & Anti-Patterns

1. **Typing Out YAML from Memory**: Writing manifests from scratch instead of using `--dry-run=client -o yaml` consumes excessive time.
2. **Context and Namespace Confusion**: Forgetting to specify the required `-n <namespace>` parameter during resource creation.
3. **Leaving Resources Unverified**: Moving on without verifying that created pods have reached the `Running` and `Ready` state.

## Security & Reliability Best Practices

- **Time Management**: Allocate maximum 5 minutes per objective during certification exams.
- **Double-Check Namespaces**: Always confirm the active namespace with `kubectl config set-context --current --namespace=...`.""",
            ["Forgetting to specify the required namespace in kubectl commands.",
             "Writing YAML manifests manually from scratch instead of generating skeletons with --dry-run=client -o yaml.",
             "Failing to verify that pods reach Ready condition before finishing the scenario."],
            "Master imperative kubectl generation commands and alias shortcuts for maximum efficiency.")
