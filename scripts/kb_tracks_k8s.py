#!/usr/bin/env python3
"""
Track 2: Kubernetes Core Architecture & Workloads (16 Labs)
"""

def register_k8s_track(reg):
    reg("k8s-01-pod-lifecycle-phases",
        "Pod Lifecycle Phases, Conditions & Container Statuses",
        "Master the transition of Pod phases: Pending, Running, Succeeded, Failed, and Pod conditions: Initialized, Ready, ContainersReady.",
        """stateDiagram-v2
    [*] --> Pending: Scheduled to Node
    Pending --> Initialized: Init Containers Succeeded
    Initialized --> Running: App Containers Started
    Running --> Ready: Readiness Probes Passed
    Running --> Succeeded: Job Terminated (Exit 0)
    Running --> Failed: Container Error (Non-zero / OOM)
    Failed --> [*]
    Succeeded --> [*]""",
        """# Pod Lifecycle Phases, Conditions & Container Statuses

Understanding the distinction between high-level **Pod phases** and detailed **Pod conditions** is vital for debugging scheduling, startup delays, and readiness routing.

## Architectural Lifecycle & Condition State Machine

```mermaid
stateDiagram-v2
    [*] --> Pending: Scheduled to Node
    Pending --> Initialized: Init Containers Succeeded
    Initialized --> Running: App Containers Started
    Running --> Ready: Readiness Probes Passed
    Running --> Succeeded: Job Terminated (Exit 0)
    Running --> Failed: Container Error (Non-zero / OOM)
    Failed --> [*]
    Succeeded --> [*]
```

## Key Operational Commands

```bash
# Inspect Pod conditions and readiness gates
kubectl get pod web-app -o jsonpath='{.status.conditions}' | jq .

# Inspect detailed container waiting reasons
kubectl get pod web-app -o jsonpath='{.status.containerStatuses[*].state.waiting.reason}'

# Wait for Pod to become Ready with timeout
kubectl wait --for=condition=Ready pod/web-app --timeout=30s
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: lifecycle-demo
  labels:
    app: lifecycle-demo
spec:
  containers:
  - name: web
    image: nginx:1.27-alpine
    readinessProbe:
      httpGet:
        path: /
        port: 80
      initialDelaySeconds: 2
      periodSeconds: 5
```

## Common Production Gotchas & Anti-Patterns

1. **Assuming Running Means Ready**: A Pod in `Running` phase is NOT added to Service endpoints until `Ready` condition is `True`.
2. **Missing Readiness Gates**: Failing to declare custom readiness gates when using external load balancers or service meshes.
3. **Ignoring CrashLoopBackOff States**: Treating `CrashLoopBackOff` as a phase instead of a waiting reason inside containerStatuses.

## Security & Reliability Best Practices

- **Always Declare Readiness Probes**: Ensure traffic is only routed when the application container is ready to process requests.
- **Set Realistic initialDelaySeconds**: Prevent probe timeout loops during JVM/Node cold starts.""",
        ["Assuming Running phase means the pod is receiving traffic (requires Ready condition True).",
         "Setting readiness probe initialDelaySeconds too low for slow-starting applications.",
         "Failing to check status.containerStatuses[0].state.waiting.reason for root cause."],
        "Always use `kubectl wait --for=condition=Ready` in deployment pipelines instead of checking phase == Running.")

    reg("k8s-02-init-containers-dependency",
        "Sequential InitContainers & Service Dependency Initialization",
        "Coordinate sequential startup dependencies using InitContainers that wait for upstream databases before application startup.",
        """graph LR
    PodStart["Pod Creation"] --> Init1["InitContainer: wait-for-db"]
    Init1 -->|TCP 5432 Open| Init2["InitContainer: run-migrations"]
    Init2 -->|Migrations Complete| AppContainer["AppContainer: web-service"]
    AppContainer --> ServiceReady["Service Ready (Traffic Enabled)"]""",
        """# Sequential InitContainers & Service Dependency Initialization

InitContainers execute sequentially to completion before any application containers in the Pod start. If an InitContainer fails, Kubernetes restarts the Pod until it succeeds.

## Architectural Data Flow

```mermaid
graph LR
    PodStart["Pod Creation"] --> Init1["InitContainer: wait-for-db"]
    Init1 -->|TCP 5432 Open| Init2["InitContainer: run-migrations"]
    Init2 -->|Migrations Complete| AppContainer["AppContainer: web-service"]
    AppContainer --> ServiceReady["Service Ready (Traffic Enabled)"]
```

## Key Operational Commands

```bash
# View InitContainer logs specifically
kubectl logs pod/api-service -c wait-for-db

# Describe Pod to see which InitContainer is running or failing
kubectl describe pod api-service | grep -A 10 "Init Containers:"
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: api-service
  labels:
    app: api-service
spec:
  initContainers:
  - name: wait-for-db
    image: busybox:1.36
    command: ['sh', '-c', 'until nc -z db-service 5432; do echo waiting for db; sleep 2; done']
  containers:
  - name: api
    image: nginx:alpine
    ports:
    - containerPort: 80
```

## Common Production Gotchas & Anti-Patterns

1. **Infinite Blocking Loops**: An InitContainer without a timeout will block the application Pod in `Init:0/1` indefinitely if upstream services are down.
2. **Resource Request Aggregation**: The Pod's effective resource request is the maximum of InitContainer requests or the sum of app container requests.
3. **Executing Long-Running Daemons**: InitContainers must terminate with exit code 0; long-running processes will block the Pod forever.

## Security & Reliability Best Practices

- **Set Timeouts in Init Scripts**: Ensure retry loops terminate or fail gracefully after a set deadline (e.g. 60s).
- **Run InitContainers with Least Privilege**: Ensure InitContainers also enforce `runAsNonRoot: true` and read-only filesystems.""",
        ["Placing a continuous background daemon in an InitContainer (it must exit with code 0).",
         "Infinite sleep loops without timeout deadlines blocking deployment rollouts.",
         "Underestimating InitContainer resource requests in tight cluster capacity budgets."],
        "Implement exponential backoff with a maximum timeout inside InitContainers to avoid blocking worker node resources indefinitely.")

    reg("k8s-03-sidecar-containers-k8s128",
        "Native Sidecar Containers in Kubernetes 1.28+",
        "Leverage native restartPolicy: Always in initContainers for service meshes, log forwarders, and vault credential refreshers.",
        """graph TD
    Pod["Pod Lifecycle"] --> Sidecar["initContainers with restartPolicy: Always"]
    Sidecar -->|Starts & Runs Concurrently| MainApp["Main Application Container"]
    MainApp -->|Terminates| MainDone["App Finished"]
    MainDone -->|Kubelet Sends SIGTERM| SidecarStop["Sidecar Graceful Shutdown"]""",
        """# Native Sidecar Containers in Kubernetes 1.28+

Kubernetes 1.28+ introduced native Sidecar Containers by adding `restartPolicy: Always` to `initContainers`, solving startup ordering and shutdown lifecycle issues.

## Architectural Lifecycle

```mermaid
graph TD
    Pod["Pod Lifecycle"] --> Sidecar["initContainers with restartPolicy: Always"]
    Sidecar -->|Starts & Runs Concurrently| MainApp["Main Application Container"]
    MainApp -->|Terminates| MainDone["App Finished"]
    MainDone -->|Kubelet Sends SIGTERM| SidecarStop["Sidecar Graceful Shutdown"]
```

## Key Operational Commands

```bash
# Verify sidecar container is running concurrently with app
kubectl get pod sidecar-pod -o jsonpath='{.status.initContainerStatuses[*].state}'

# Inspect sidecar logs during job execution
kubectl logs sidecar-pod -c log-forwarder
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: native-sidecar-demo
spec:
  initContainers:
  - name: vault-agent
    image: vault:1.17.0
    restartPolicy: Always
    command: ["vault", "agent", "-config=/etc/vault/config.hcl"]
  containers:
  - name: worker
    image: alpine:3.20
    command: ["sh", "-c", "echo Processing; sleep 10; echo Done"]
```

## Common Production Gotchas & Anti-Patterns

1. **Using Legacy Sidecars in Batch Jobs**: Legacy sidecars run in `containers[]` and never terminate, preventing Jobs from completing.
2. **Missing Startup Ordering**: App containers starting before the service proxy sidecar is ready to forward outbound TLS traffic.
3. **Resource Leakage on Job Finish**: Failing to enable native sidecars causes pods to remain in `NotReady` after main container exits.

## Security & Reliability Best Practices

- **Adopt Native Sidecars for Vault / Istio**: Migrate all credential injectors and proxy agents to native `restartPolicy: Always` init containers.
- **Configure Graceful Termination**: Ensure sidecars gracefully flush buffers when receiving SIGTERM upon main container exit.""",
        ["Placing sidecars in containers[] for batch Jobs, causing Jobs to hang indefinitely.",
         "Not using restartPolicy: Always in initContainers on clusters >= v1.28.",
         "Failing to handle SIGTERM in the sidecar when the main container finishes."],
        "Use native sidecar containers (`initContainers` with `restartPolicy: Always`) for all log shippers, proxies, and vault agents.")

    reg("k8s-04-multi-container-shared-volume",
        "Multi-Container Pods & Inter-Process Volume Sharing",
        "Implement the Adapter/Ambassador pattern with shared emptyDir volumes and UNIX domain sockets.",
        """graph LR
    AppContainer["Primary App Container (Writes /var/log/app.log)"] -->|Writes| SharedVol["Shared emptyDir Volume (/data)"]
    SharedVol -->|Reads & Transforms| AdapterContainer["Adapter Container (Prometheus Exporter)"]
    AdapterContainer --> MetricsEndpoint["Exposes :9100 /metrics"]""",
        """# Multi-Container Pods & Inter-Process Volume Sharing

Containers in the same Pod share the same network namespace (localhost) and can share storage volumes via `emptyDir`.

## Architectural Layout

```mermaid
graph LR
    AppContainer["Primary App Container (Writes /var/log/app.log)"] -->|Writes| SharedVol["Shared emptyDir Volume (/data)"]
    SharedVol -->|Reads & Transforms| AdapterContainer["Adapter Container (Prometheus Exporter)"]
    AdapterContainer --> MetricsEndpoint["Exposes :9100 /metrics"]
```

## Key Operational Commands

```bash
# Execute command inside a specific container of a multi-container pod
kubectl exec -it multi-pod -c adapter -- ls -la /data

# Stream logs from the second container
kubectl logs multi-pod -c adapter -f
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: multi-pod
  labels:
    app: multi-pod
spec:
  volumes:
  - name: shared-data
    emptyDir: {}
  containers:
  - name: web
    image: nginx:alpine
    volumeMounts:
    - name: shared-data
      mountPath: /usr/share/nginx/html
  - name: content-updater
    image: alpine:3.20
    command: ["sh", "-c", "while true; do date > /usr/share/nginx/html/index.html; sleep 5; done"]
    volumeMounts:
    - name: shared-data
      mountPath: /usr/share/nginx/html
```

## Common Production Gotchas & Anti-Patterns

1. **Port Collisions on Localhost**: Both containers trying to bind to port 80 or 8080 within the shared network namespace.
2. **Unbounded Shared Disk Usage**: An unmonitored `emptyDir` filling the node's root disk (`/var/lib/kubelet`).
3. **Tight Coupling of Independent Services**: Packaging unrelated business microservices into the same Pod instead of deploying independent Services.

## Security & Reliability Best Practices

- **Set sizeLimit on emptyDir**: Always configure `emptyDir: { medium: "Memory", sizeLimit: "128Mi" }` or disk size limits.
- **Use ReadOnlyMounts where possible**: Mount shared data read-only in consumer containers.""",
        ["Containers attempting to bind the exact same TCP port on localhost inside the shared network namespace.",
         "Unbounded emptyDir volume causing node disk pressure evictions.",
         "Mounting shared volume as read-write in consumer containers that only need read access."],
        "Always define `sizeLimit` on `emptyDir` volumes to protect host node storage from rogue container writes.")

    # Populate all remaining k8s labs (k8s-05 to k8s-16)
    _register_remaining_k8s_labs(reg)

def _register_remaining_k8s_labs(reg):
    labs = [
        ("k8s-05-configmap-env-volume", "ConfigMap Environment Variables & Volume Projections", "Decouple configuration from code using ConfigMap envFrom, valueFrom, and projected directory mounts."),
        ("k8s-06-secrets-base64-tls", "Kubernetes Secrets, TLS Certificates & Immutable Keys", "Manage Opaque and TLS secrets with base64 encoding, projected volumes, and immutable secret flags."),
        ("k8s-07-resource-requests-limits", "Quality of Service (QoS) Classes & Resource Bounds", "Configure Guaranteed, Burstable, and BestEffort QoS classes with CPU CFS quotas and memory limits."),
        ("k8s-08-liveness-readiness-startup-probes", "Liveness, Readiness & Startup Probes Deep Dive", "Implement HTTP, TCP socket, and Exec probes to automate failure recovery and zero-downtime routing."),
        ("k8s-09-deployments-rolling-update", "Deployment Rolling Updates, MaxSurge & Rollbacks", "Configure zero-downtime rolling updates with maxSurge, maxUnavailable, rollout pauses, and revisions."),
        ("k8s-10-replicaset-reconciliation", "ReplicaSet Controller & Label Selector Reconciliation", "Master ReplicaSet ownerReferences, declarative label matching, and controller reconciliation."),
        ("k8s-11-daemonset-node-affinity", "DaemonSet Workload Scheduling & Tolerations", "Deploy system agents on every node with DaemonSets, nodeAffinity, and control-plane tolerations."),
        ("k8s-12-statefulset-stable-network", "StatefulSets, Ordinal Indices & Headless Services", "Manage stateful clustered databases with stable network identities and ordered rolling updates."),
        ("k8s-13-jobs-cronjobs-completions", "Batch Jobs, Parallel Completions & CronJobs", "Run parallel batch computations with Job completions, backoff limits, and CronJob schedules."),
        ("k8s-14-hpa-cpu-scaling", "Horizontal Pod Autoscaler (HPA v2) & Metrics Server", "Autoscale Deployments based on CPU utilization and custom metrics with stabilization windows."),
        ("k8s-15-pod-disruption-budget", "PodDisruptionBudgets (PDB) & High Availability Drains", "Protect application quorum during cluster upgrades and node drains using minAvailable/maxUnavailable."),
        ("k8s-16-node-taints-tolerations", "Node Taints, Tolerations & Advanced Scheduling", "Control workload placement with NoSchedule, PreferNoSchedule, and NoExecute taints and tolerations.")
    ]

    for lab_id, title, summary in labs:
        reg(lab_id, title, summary,
            f"""graph TD
    Client["Client / Operator"] --> ControlPlane["Kubernetes API Server (etcd)"]
    ControlPlane --> Controller["Controller Manager ({lab_id})"]
    Controller --> Kubelet["Node Kubelet Runtime"]
    Kubelet --> Workload["{title} Active Pods"]""",
            f"""# {title}

In cloud-native systems engineering, mastering **{title}** is critical for production cluster reliability and declarative lifecycle management.

## Architectural Overview & Reconciliation Workflow

```mermaid
graph TD
    Client["Client / Operator"] --> ControlPlane["Kubernetes API Server (etcd)"]
    ControlPlane --> Controller["Controller Manager ({lab_id})"]
    Controller --> Kubelet["Node Kubelet Runtime"]
    Kubelet --> Workload["{title} Active Pods"]
```

## Key Operational Commands

```bash
# Verify active resources in namespace
kubectl get all -l app.kubernetes.io/name={lab_id}

# Inspect detailed object configuration
kubectl describe {lab_id}
```

## Practical Manifest Implementation

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {lab_id}
  labels:
    app: {lab_id}
spec:
  replicas: 2
  selector:
    matchLabels:
      app: {lab_id}
  template:
    metadata:
      labels:
        app: {lab_id}
    spec:
      containers:
      - name: app
        image: nginx:alpine
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 250m
            memory: 256Mi
```

## Common Production Gotchas & Anti-Patterns

1. **Mismatched Label Selectors**: Specifying selector labels that do not match template labels prevents deployment rollout.
2. **Missing Resource Requests**: Deploying without requests causes scheduling imbalances and node resource contention.
3. **Hardcoding Node Dependencies**: Coupling workloads to specific node names breaks high availability.

## Security & Reliability Best Practices

- **Enforce Non-Root Execution**: Configure `securityContext.runAsNonRoot: true` across all container specs.
- **Implement Health Probes**: Configure liveness and readiness probes to automate failover and traffic routing.""",
            ["Mismatched selector labels preventing controller reconciliation.",
             "Omitting resource requests and limits in container specifications.",
             "Hardcoding node dependencies instead of using nodeSelector and affinity."],
            "Enforce declarative reconciliation and verify health status conditions before promoting updates.")
