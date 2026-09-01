#!/usr/bin/env python3
"""
Tracks 11 & 12: Troubleshooting (10 Labs) & SRE Performance (10 Labs)
"""

def register_trouble_and_sre(reg):
    # Track 11: Troubleshooting (10 labs)
    trouble_labs = [
        ("trouble-01-crashloopbackoff-diagnosis", "Diagnosing & Fixing CrashLoopBackOff Errors", "Extract exit codes (1, 137, 139), inspect previous container logs, and fix startup entrypoint failures."),
        ("trouble-02-imagepullbackoff-auth", "Resolving ImagePullBackOff & Registry Authentication", "Diagnose private registry 401 Unauthorized errors and attach imagePullSecrets."),
        ("trouble-03-pending-insufficient-cpu", "Debugging Unschedulable Pods (Insufficient CPU/Memory)", "Identify node resource exhaustion, taints/tolerations mismatches, and scale cluster capacity."),
        ("trouble-04-oomkilled-exit-137", "Root-Cause Analysis of OOMKilled (Exit 137) Outages", "Investigate kernel cgroup memory limits, heap dumps, and memory leak root causes."),
        ("trouble-05-missing-service-endpoints", "Fixing Missing Service Endpoints & Label Mismatches", "Repair broken traffic routing caused by selector/label mismatches between Services and Pods."),
        ("trouble-06-dns-nxdomain-resolution", "Troubleshooting CoreDNS Failures & ndots Expansion", "Diagnose CoreDNS latency, upstream timeout cascades, and ndots:5 query expansion."),
        ("trouble-07-node-notready-kubelet", "Recovering Node NotReady States & Kubelet Crashes", "Inspect node disk pressure taints, PIDs exhaustion, and restart crashed kubelet daemons."),
        ("trouble-08-pvc-mount-deadlock", "Resolving Multi-Attach Volume Deadlocks on Nodes", "Force detach stuck EBS/CSI volumes after node kernel panics to unblock rescheduling."),
        ("trouble-09-networkpolicy-traffic-blocked", "Diagnosing Blocked Traffic in Strict NetworkPolicies", "Use network policy diagnostics and flow logs to identify missing ingress/egress rules."),
        ("trouble-10-ingress-503-backend-dead", "Troubleshooting Ingress 503 Service Unavailable", "Debug Ingress controller routing timeouts and unready backend pod health checks.")
    ]

    for lab_id, title, summary in trouble_labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Alert["Alert / Failure Reported"] --> Triage["Diagnostic Inspection (kubectl describe / logs)"]
    Triage --> RootCause["Root Cause Identified ({lab_id})"]
    RootCause --> Fix["Remediation Manifest Applied"]
    Fix --> Healthy["Cluster State Converged (Healthy)"]""",
            f"""# {title}

Rapid root-cause analysis and deterministic remediation are essential skills for maintaining high availability in production Kubernetes clusters.

## Diagnostic Flowchart

```mermaid
graph TD
    Alert["Alert / Failure Reported"] --> Triage["Diagnostic Inspection (kubectl describe / logs)"]
    Triage --> RootCause["Root Cause Identified ({lab_id})"]
    RootCause --> Fix["Remediation Manifest Applied"]
    Fix --> Healthy["Cluster State Converged (Healthy)"]
```

## Key Operational Commands

```bash
# View logs from crashed container instance
kubectl logs pod/failing-pod --previous

# Check detailed exit code and termination reason
kubectl get pod failing-pod -o jsonpath='{{.status.containerStatuses[*].lastState.terminated}}'
```

## Common Production Gotchas & Anti-Patterns

1. **Treating Symptoms Instead of Root Causes**: Restarting pods repeatedly without fixing the underlying memory leak or database deadlock.
2. **Deleting PVCs During Mount Errors**: Deleting PersistentVolumeClaims instead of resolving the multi-attach volume lock, risking permanent data loss.
3. **Ignoring Previous Container Logs**: Forgetting to check `kubectl logs --previous` on restarted pods.

## Security & Reliability Best Practices

- **Automate Diagnostic Bundles**: Implement tools like `kubectl-debug` and automated node problem detectors.
- **Set Up Alerting on Crash Loops**: Fire PagerDuty alerts immediately when crash loops are detected.
```
""",
            ["Failing to inspect 'kubectl logs --previous' on pods that have restarted.",
             "Deleting PersistentVolumeClaims when encountering multi-attach errors, causing data loss.",
             "Ignoring container exit codes (137 = OOM, 139 = Segfault, 1 = Config/Runtime Error)."],
            "Always follow a structured diagnostic triage: Events -> Logs -> Previous Logs -> Describe Status -> Network Connectivity.")

    # Track 12: SRE Performance (10 labs)
    sre_labs = [
        ("sre-01-sli-slo-definition", "Defining Service Level Indicators (SLIs) & Objectives (SLOs)", "Formulate quantitative availability and latency SLIs using Prometheus PromQL expressions."),
        ("sre-02-multi-window-burn-rate-alert", "Multi-Window Multi-Burn-Rate Alerting Rules", "Implement Google SRE multi-window error budget burn rate alerting to eliminate noise and fatigue."),
        ("sre-03-vpa-resource-recommendations", "Vertical Pod Autoscaler (VPA) & Resource Right-Sizing", "Analyze live memory/CPU consumption and compute optimal resource requests using VPA."),
        ("sre-04-keda-event-driven-autoscaling", "Event-Driven Autoscaling with KEDA & Message Queues", "Scale Kubernetes workloads from 0 to N replicas based on Kafka lag and Redis queue depth."),
        ("sre-05-topology-spread-constraints", "Multi-Zone High Availability with TopologySpreadConstraints", "Distribute pod replicas evenly across availability zones using maxSkew and topologyKey."),
        ("sre-06-pod-priority-preemption", "Pod PriorityClasses & Preemption Architecture", "Protect critical workloads and allow graceful preemption of low-priority batch jobs."),
        ("sre-07-node-problem-detector", "Node Problem Detector & Automated Node Remediation", "Detect kernel deadlocks, corrupted filesystems, and automate cordoning before failures spread."),
        ("sre-08-chaos-mesh-pod-kill", "Chaos Engineering Experiments with Chaos Mesh", "Inject pod kills, network delays, and I/O faults to prove application self-healing resilience."),
        ("sre-09-concurrency-limits-overload", "Adaptive Concurrency Limiting & Load Shedding", "Implement token-bucket rate limiting to shed excess traffic gracefully with HTTP 429."),
        ("sre-10-kubelet-eviction-thresholds", "Kubelet Eviction Thresholds & Node OOM Management", "Configure hard and soft eviction thresholds to prevent catastrophic node kernel panics.")
    ]

    for lab_id, title, summary in sre_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    Metrics["Prometheus SLI Stream"] --> Engine["SLO Evaluation Engine ({lab_id})"]
    Engine --> Budget["Error Budget Balance"]
    Budget -->|Burn Rate > 14.4x| PagerAlert["Page On-Call SRE (1h Window)"]
    Budget -->|Burn Rate < 1x| Normal["Normal Operation (Ship Features)"]""",
            f"""# {title}

Site Reliability Engineering applies software engineering principles to infrastructure operations, balancing velocity with error budgets.

## Architectural Reliability Loop

```mermaid
graph LR
    Metrics["Prometheus SLI Stream"] --> Engine["SLO Evaluation Engine ({lab_id})"]
    Engine --> Budget["Error Budget Balance"]
    Budget -->|Burn Rate > 14.4x| PagerAlert["Page On-Call SRE (1h Window)"]
    Budget -->|Burn Rate < 1x| Normal["Normal Operation (Ship Features)"]
```

## Key Operational Commands

```bash
# Calculate 30-day availability SLO in PromQL
# sum(rate(http_requests_total[30d]))

# Inspect VPA resource recommendations
kubectl get vpa my-app-vpa -o yaml
```

## Common Production Gotchas & Anti-Patterns

1. **Alerting on Single-Minute Spikes**: Creating alerts on 1m spikes instead of multi-window burn rates creates severe on-call fatigue.
2. **Setting VPA UpdateMode to Auto in Production**: Allowing VPA to restart live pods during peak traffic without coordination.
3. **Omitting TopologySpreadConstraints**: Concentrating all replicas on a single node or zone, risking total outage during cloud zone failure.

## Security & Reliability Best Practices

- **Enforce maxSkew: 1**: Distribute replicas evenly across all failure domains.
- **Implement Rate Limiting**: Reject surplus traffic with HTTP 429 before backends collapse.""",
            ["Alerting on instantaneous error spikes rather than multi-window error budget burn rates.",
             "Concentrating all application replicas in a single availability zone.",
             "Enabling VPA updateMode: Auto on workloads without PodDisruptionBudgets."],
            "Implement multi-window multi-burn-rate alerts (1h 14.4x and 6h 6x) to protect error budgets without false alarms.")
