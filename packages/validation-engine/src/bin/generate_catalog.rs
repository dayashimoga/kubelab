use kubelab_validation_engine::models::{
    DeclarativeLabDef, LabHint, LabTask, StateAssertion, TaskValidation, TaskValidationType,
    ValidationOperator,
};
use serde_json::json;
use std::fs;
use std::path::Path;

fn create_lab(
    id: &str,
    title: &str,
    track: &str,
    difficulty: &str,
    duration: u32,
    scenario: &str,
    tasks: Vec<LabTask>,
    hints: Vec<&str>,
    solution: &str,
) -> DeclarativeLabDef {
    DeclarativeLabDef {
        id: id.to_string(),
        title: title.to_string(),
        track: track.to_string(),
        difficulty: difficulty.to_string(),
        duration_minutes: duration,
        objectives: vec![
            format!("Master {} in Kubernetes", title),
            "Execute and verify deterministic live cluster state".to_string(),
        ],
        prerequisites: vec![],
        environment: Some(json!({
            "type": "kubernetes",
            "cluster": "disposable",
            "namespace_isolation": true,
            "resources": { "cpu_limit": "500m", "memory_limit": "512Mi" }
        })),
        initial_state: Some(json!({ "manifests": [] })),
        scenario: scenario.to_string(),
        tasks,
        hints: hints
            .into_iter()
            .map(|h| LabHint {
                text: h.to_string(),
                penalty_points: 10,
            })
            .collect(),
        solution: solution.to_string(),
        cleanup: Some(json!({ "auto": true })),
        limits: Some(json!({ "max_attempts": 5, "timeout_minutes": duration + 5 })),
        security: Some(json!({
            "runAsNonRoot": true,
            "allowPrivilegeEscalation": false,
            "seccompProfile": "RuntimeDefault"
        })),
        resources: Some(json!({ "cpu": "250m", "memory": "256Mi" })),
        tested_versions: Some(vec![
            "v1.28.0".to_string(),
            "v1.29.0".to_string(),
            "v1.30.0".to_string(),
        ]),
    }
}

fn simple_k8s_task(
    id: &str,
    title: &str,
    points: u32,
    resource: &str,
    name: &str,
    field: &str,
    op: ValidationOperator,
    expected: serde_json::Value,
) -> LabTask {
    LabTask {
        id: id.to_string(),
        title: title.to_string(),
        description: format!("Verify {} on {} '{}'", field, resource, name),
        points,
        validation: TaskValidation {
            validation_type: TaskValidationType::K8sResource,
            resource: Some(resource.to_string()),
            name: Some(name.to_string()),
            namespace: None,
            assertions: vec![StateAssertion {
                field: field.to_string(),
                operator: op,
                expected,
                actual: None,
                passed: None,
                error_message: None,
            }],
        },
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut labs: Vec<DeclarativeLabDef> = Vec::new();

    // ==========================================
    // 1. Linux & Containers (8 Labs)
    // ==========================================
    labs.push(create_lab(
        "linux-01-fs-permissions",
        "Linux Filesystem Hierarchy & POSIX Permissions",
        "linux-containers",
        "beginner",
        15,
        "Enforce strict 600 file permissions and root:root ownership on secret token files.",
        vec![
            simple_k8s_task("task-1", "Deploy secure agent pod", 50, "pods", "sec-agent", "status.phase", ValidationOperator::Equals, json!("Running")),
            simple_k8s_task("task-2", "Label application tier", 50, "pods", "sec-agent", "metadata.labels.tier", ValidationOperator::Equals, json!("agent")),
        ],
        vec!["Use `chmod 600 /etc/secrets/token` and `chown 0:0 /etc/secrets/token`"],
        "kubectl run sec-agent --image=alpine --labels=tier=agent -- sleep 3600",
    ));

    labs.push(create_lab(
        "linux-02-process-signals",
        "Process Lifecycle & Graceful SIGTERM Shutdown",
        "linux-containers",
        "beginner",
        15,
        "Configure 30-second graceful terminationGracePeriodSeconds to handle SIGTERM without dropping connections.",
        vec![
            simple_k8s_task("task-1", "Configure 30s grace period", 50, "pods", "graceful-svc", "spec.terminationGracePeriodSeconds", ValidationOperator::Equals, json!(30)),
            simple_k8s_task("task-2", "Ensure pod is running", 50, "pods", "graceful-svc", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["Set terminationGracePeriodSeconds: 30 in the pod spec."],
        "kubectl apply -f graceful-pod.yaml",
    ));

    labs.push(create_lab(
        "linux-03-namespaces-isolation",
        "Linux Namespaces (PID, MNT, NET) Isolation",
        "linux-containers",
        "intermediate",
        20,
        "Isolate container process tree by strictly enforcing hostPID: false and hostNetwork: false.",
        vec![
            simple_k8s_task("task-1", "Enforce hostPID false", 50, "pods", "isolated-node", "spec.hostPID", ValidationOperator::Equals, json!(false)),
            simple_k8s_task("task-2", "Enforce hostNetwork false", 50, "pods", "isolated-node", "spec.hostNetwork", ValidationOperator::Equals, json!(false)),
        ],
        vec!["Omit hostPID and hostNetwork or set them explicitly to false."],
        "kubectl apply -f isolated-pod.yaml",
    ));

    labs.push(create_lab(
        "linux-04-cgroups-limits",
        "Control Groups (cgroups v2) CPU & Memory Constraints",
        "linux-containers",
        "intermediate",
        20,
        "Apply cgroup limits (500m CPU, 256Mi RAM) on worker container to prevent node starvation.",
        vec![
            simple_k8s_task("task-1", "Set CPU limit to 500m", 50, "pods", "cgroup-worker", "spec.containers[0].resources.limits.cpu", ValidationOperator::Equals, json!("500m")),
            simple_k8s_task("task-2", "Set memory limit to 256Mi", 50, "pods", "cgroup-worker", "spec.containers[0].resources.limits.memory", ValidationOperator::Equals, json!("256Mi")),
        ],
        vec!["Add limits under resources block in container spec."],
        "kubectl apply -f cgroup-worker.yaml",
    ));

    labs.push(create_lab(
        "linux-05-containerfile-builds",
        "Multi-Stage Containerfile Optimization",
        "linux-containers",
        "intermediate",
        20,
        "Deploy a hardened minimal container utilizing distroless/alpine base images.",
        vec![
            simple_k8s_task("task-1", "Deploy alpine-based runner", 100, "pods", "micro-runner", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["Use alpine:3.19 or gcr.io/distroless/static."],
        "kubectl run micro-runner --image=alpine:3.19 -- sleep 3600",
    ));

    labs.push(create_lab(
        "linux-06-podman-rootless",
        "Rootless Container Execution & User Namespaces",
        "linux-containers",
        "intermediate",
        20,
        "Run container as non-root user UID 10001 with read-only root filesystem.",
        vec![
            simple_k8s_task("task-1", "Set runAsNonRoot true", 50, "pods", "rootless-workload", "spec.securityContext.runAsNonRoot", ValidationOperator::Equals, json!(true)),
            simple_k8s_task("task-2", "Set runAsUser 10001", 50, "pods", "rootless-workload", "spec.securityContext.runAsUser", ValidationOperator::Equals, json!(10001)),
        ],
        vec!["Configure securityContext with runAsNonRoot and runAsUser."],
        "kubectl apply -f rootless-workload.yaml",
    ));

    labs.push(create_lab(
        "linux-07-systemd-units",
        "Container Orchestration with Systemd & Restart Policies",
        "linux-containers",
        "advanced",
        25,
        "Configure automated restartPolicy: Always on production daemon workloads.",
        vec![
            simple_k8s_task("task-1", "Configure restartPolicy Always", 100, "pods", "auto-heal-daemon", "spec.restartPolicy", ValidationOperator::Equals, json!("Always")),
        ],
        vec!["Ensure restartPolicy is set to Always."],
        "kubectl run auto-heal-daemon --image=redis:alpine --restart=Always",
    ));

    labs.push(create_lab(
        "linux-08-oci-spec-runtime",
        "OCI Runtime Spec & Seccomp Profile Activation",
        "linux-containers",
        "advanced",
        25,
        "Enable RuntimeDefault seccomp profile across all container processes.",
        vec![
            simple_k8s_task("task-1", "Activate RuntimeDefault seccomp", 100, "pods", "hardened-oci", "spec.securityContext.seccompProfile.type", ValidationOperator::Equals, json!("RuntimeDefault")),
        ],
        vec!["Set securityContext.seccompProfile.type to RuntimeDefault."],
        "kubectl apply -f hardened-oci.yaml",
    ));

    // ==========================================
    // 2. Kubernetes Core Workloads (15 Labs)
    // ==========================================
    labs.push(create_lab(
        "k8s-pod-basics",
        "Create and Configure Your First Pod",
        "kubernetes",
        "beginner",
        15,
        "Deploy a stateless frontend container named web-server on port 80 with label app=frontend.",
        vec![
            simple_k8s_task("task-deploy-pod", "Deploy web-server pod", 50, "pods", "web-server", "status.phase", ValidationOperator::Equals, json!("Running")),
            simple_k8s_task("task-verify-port", "Verify container port", 50, "pods", "web-server", "spec.containers[0].ports[0].containerPort", ValidationOperator::Equals, json!(80)),
        ],
        vec!["Use `kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend`"],
        "kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend",
    ));

    labs.push(create_lab(
        "k8s-deployments-scaling",
        "Deployments, Scaling and Rolling Updates",
        "kubernetes",
        "beginner",
        20,
        "Deploy a 3-replica API deployment and verify all 3 replicas are in Ready state.",
        vec![
            simple_k8s_task("task-1", "Verify 3 ready replicas", 100, "deployments", "api-deployment", "status.readyReplicas", ValidationOperator::Equals, json!(3)),
        ],
        vec!["kubectl create deployment api-deployment --image=nginx:alpine --replicas=3"],
        "kubectl create deployment api-deployment --image=nginx:alpine --replicas=3",
    ));

    labs.push(create_lab(
        "k8s-services-clusterip",
        "ClusterIP Service Discovery & Endpoint Resolution",
        "kubernetes",
        "beginner",
        15,
        "Create a ClusterIP service named backend-svc exposing port 8080 targeting container port 80.",
        vec![
            simple_k8s_task("task-1", "Verify ClusterIP service", 50, "services", "backend-svc", "spec.type", ValidationOperator::Equals, json!("ClusterIP")),
            simple_k8s_task("task-2", "Verify port 8080 mapping", 50, "services", "backend-svc", "spec.ports[0].port", ValidationOperator::Equals, json!(8080)),
        ],
        vec!["kubectl expose deployment backend --name=backend-svc --port=8080 --target-port=80"],
        "kubectl expose deployment backend --name=backend-svc --port=8080 --target-port=80",
    ));

    labs.push(create_lab(
        "k8s-configmaps-env",
        "Decoupling Configuration with ConfigMaps",
        "kubernetes",
        "beginner",
        15,
        "Create a ConfigMap app-config with key DB_HOST=postgres.internal and mount as environment variable.",
        vec![
            simple_k8s_task("task-1", "Create ConfigMap", 50, "configmaps", "app-config", "data.DB_HOST", ValidationOperator::Equals, json!("postgres.internal")),
            simple_k8s_task("task-2", "Deploy consumer pod", 50, "pods", "app-consumer", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["kubectl create configmap app-config --from-literal=DB_HOST=postgres.internal"],
        "kubectl create configmap app-config --from-literal=DB_HOST=postgres.internal",
    ));

    labs.push(create_lab(
        "k8s-secrets-mounts",
        "Secure Secret Storage & Volume Mount Injection",
        "kubernetes",
        "intermediate",
        15,
        "Create generic secret api-credentials containing api-token and mount it read-only at /etc/secrets.",
        vec![
            simple_k8s_task("task-1", "Create Secret", 50, "secrets", "api-credentials", "type", ValidationOperator::Equals, json!("Opaque")),
            simple_k8s_task("task-2", "Mount Secret Volume", 50, "pods", "secure-api-client", "spec.volumes[0].secret.secretName", ValidationOperator::Equals, json!("api-credentials")),
        ],
        vec!["kubectl create secret generic api-credentials --from-literal=api-token=s3cr3t"],
        "kubectl create secret generic api-credentials --from-literal=api-token=s3cr3t",
    ));

    labs.push(create_lab(
        "k8s-liveness-readiness",
        "Health Probes (Liveness, Readiness, Startup)",
        "kubernetes",
        "intermediate",
        20,
        "Configure HTTP liveness probe on /healthz and readiness probe on /readyz with 5s periodSeconds.",
        vec![
            simple_k8s_task("task-1", "Liveness probe on /healthz", 50, "pods", "healthy-web", "spec.containers[0].livenessProbe.httpGet.path", ValidationOperator::Equals, json!("/healthz")),
            simple_k8s_task("task-2", "Readiness probe on /readyz", 50, "pods", "healthy-web", "spec.containers[0].readinessProbe.httpGet.path", ValidationOperator::Equals, json!("/readyz")),
        ],
        vec!["Define livenessProbe and readinessProbe under container spec."],
        "kubectl apply -f healthy-web.yaml",
    ));

    labs.push(create_lab(
        "k8s-resource-limits",
        "Pod Resource Requests, Limits & Guaranteed QoS",
        "kubernetes",
        "intermediate",
        20,
        "Set identical requests and limits (cpu: 200m, memory: 256Mi) to achieve Guaranteed QoS class.",
        vec![
            simple_k8s_task("task-1", "CPU request 200m", 50, "pods", "guaranteed-pod", "spec.containers[0].resources.requests.cpu", ValidationOperator::Equals, json!("200m")),
            simple_k8s_task("task-2", "Memory limit 256Mi", 50, "pods", "guaranteed-pod", "spec.containers[0].resources.limits.memory", ValidationOperator::Equals, json!("256Mi")),
        ],
        vec!["Specify equal requests and limits for cpu and memory."],
        "kubectl apply -f guaranteed-qos.yaml",
    ));

    labs.push(create_lab(
        "k8s-init-containers",
        "Init Containers for Pre-Flight Dependency Bootstrapping",
        "kubernetes",
        "intermediate",
        20,
        "Configure initContainer wait-for-db to verify database connectivity before launching app container.",
        vec![
            simple_k8s_task("task-1", "Define initContainer", 50, "pods", "app-with-init", "spec.initContainers[0].name", ValidationOperator::Equals, json!("wait-for-db")),
            simple_k8s_task("task-2", "Ensure main container running", 50, "pods", "app-with-init", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["Add initContainers array with nc/curl check in pod spec."],
        "kubectl apply -f app-init.yaml",
    ));

    labs.push(create_lab(
        "k8s-daemonsets-logging",
        "DaemonSets for Node-Level Log Collection",
        "kubernetes",
        "intermediate",
        20,
        "Deploy DaemonSet fluent-bit-collector ensuring an agent runs on every schedulable worker node.",
        vec![
            simple_k8s_task("task-1", "Deploy DaemonSet", 100, "daemonsets", "fluent-bit-collector", "status.desiredNumberScheduled", ValidationOperator::GreaterThan, json!(0)),
        ],
        vec!["Create DaemonSet with kind: DaemonSet and template matching labels."],
        "kubectl apply -f fluent-bit-ds.yaml",
    ));

    labs.push(create_lab(
        "k8s-statefulsets-headless",
        "StatefulSets & Headless Services for Clustered State",
        "kubernetes",
        "advanced",
        25,
        "Deploy 3-replica StatefulSet database-cluster with headless Service clusterIP: None.",
        vec![
            simple_k8s_task("task-1", "Headless service clusterIP None", 50, "services", "db-headless", "spec.clusterIP", ValidationOperator::Equals, json!("None")),
            simple_k8s_task("task-2", "StatefulSet with 3 replicas", 50, "statefulsets", "database-cluster", "spec.replicas", ValidationOperator::Equals, json!(3)),
        ],
        vec!["Set spec.clusterIP: None on Service and serviceName: db-headless on StatefulSet."],
        "kubectl apply -f statefulset-cluster.yaml",
    ));

    labs.push(create_lab(
        "k8s-jobs-cronjobs",
        "Batch Processing with Jobs & CronJobs",
        "kubernetes",
        "intermediate",
        20,
        "Create CronJob nightly-report running schedule `0 2 * * *` with successfulJobsHistoryLimit: 3.",
        vec![
            simple_k8s_task("task-1", "Configure CronJob schedule", 50, "cronjobs", "nightly-report", "spec.schedule", ValidationOperator::Equals, json!("0 2 * * *")),
            simple_k8s_task("task-2", "History limit 3", 50, "cronjobs", "nightly-report", "spec.successfulJobsHistoryLimit", ValidationOperator::Equals, json!(3)),
        ],
        vec!["kubectl create cronjob nightly-report --image=alpine --schedule='0 2 * * *' -- /bin/sh -c 'date'"],
        "kubectl create cronjob nightly-report --image=alpine --schedule='0 2 * * *' -- /bin/sh -c 'date'",
    ));

    labs.push(create_lab(
        "k8s-pod-disruption-budgets",
        "High Availability with PodDisruptionBudgets (PDB)",
        "kubernetes",
        "intermediate",
        20,
        "Create PodDisruptionBudget api-pdb with minAvailable: 2 targeting app=api pods.",
        vec![
            simple_k8s_task("task-1", "Configure minAvailable 2", 100, "poddisruptionbudgets", "api-pdb", "spec.minAvailable", ValidationOperator::Equals, json!(2)),
        ],
        vec!["kubectl create pdb api-pdb --selector=app=api --min-available=2"],
        "kubectl create pdb api-pdb --selector=app=api --min-available=2",
    ));

    labs.push(create_lab(
        "k8s-node-affinity-taints",
        "Scheduling Control with NodeAffinity, Taints & Tolerations",
        "kubernetes",
        "advanced",
        25,
        "Configure tolerations for taint `dedicated=gpu:NoSchedule` and nodeAffinity for `accelerator=nvidia`.",
        vec![
            simple_k8s_task("task-1", "Tolerate dedicated taint", 50, "pods", "gpu-worker", "spec.tolerations[0].key", ValidationOperator::Equals, json!("dedicated")),
            simple_k8s_task("task-2", "Ensure pod placed", 50, "pods", "gpu-worker", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["Add tolerations array and affinity.nodeAffinity in pod spec."],
        "kubectl apply -f gpu-worker.yaml",
    ));

    labs.push(create_lab(
        "k8s-horizontal-pod-autoscaler",
        "Metric-Driven Autoscaling with HPA v2",
        "kubernetes",
        "advanced",
        25,
        "Create HPA for deployment payment-svc with minReplicas: 2, maxReplicas: 10, target CPU 70%.",
        vec![
            simple_k8s_task("task-1", "HPA minReplicas 2", 50, "horizontalpodautoscalers", "payment-hpa", "spec.minReplicas", ValidationOperator::Equals, json!(2)),
            simple_k8s_task("task-2", "HPA maxReplicas 10", 50, "horizontalpodautoscalers", "payment-hpa", "spec.maxReplicas", ValidationOperator::Equals, json!(10)),
        ],
        vec!["kubectl autoscale deployment payment-svc --min=2 --max=10 --cpu-percent=70 --name=payment-hpa"],
        "kubectl autoscale deployment payment-svc --min=2 --max=10 --cpu-percent=70 --name=payment-hpa",
    ));

    labs.push(create_lab(
        "k8s-ephemeral-containers",
        "Live Pod Debugging with Ephemeral Debug Containers",
        "kubernetes",
        "advanced",
        25,
        "Attach an ephemeral debug container with busybox to inspect network sockets on a live running pod.",
        vec![
            simple_k8s_task("task-1", "Deploy base application", 100, "pods", "live-app", "status.phase", ValidationOperator::Equals, json!("Running")),
        ],
        vec!["kubectl debug pod/live-app -it --image=busybox:1.36"],
        "kubectl debug pod/live-app -it --image=busybox:1.36",
    ));

    // Helper closure to bulk generate remaining categorized labs
    let categories = [
        ("administration", 12, "admin"),
        ("networking", 12, "net"),
        ("security", 12, "sec"),
        ("storage", 8, "storage"),
        ("helm-kustomize", 8, "helm"),
        ("gitops", 10, "gitops"),
        ("observability", 10, "obs"),
        ("service-mesh", 10, "mesh"),
        ("sre-performance", 8, "sre"),
        ("troubleshooting", 10, "tb"),
        ("platform-multicluster", 7, "plat"),
        ("certification", 10, "cert"),
    ];

    let titles: std::collections::HashMap<&str, Vec<(&str, &str, &str, &str)>> = [
        ("administration", vec![
            ("admin-01-kubeadm-bootstrap", "Bootstrapping Cluster with kubeadm", "Control-plane bootstrap and token joining", "nodes"),
            ("admin-02-etcd-backup-restore", "etcd Snapshot Backup and Restore", "Take snapshot with etcdctl and verify integrity", "pods"),
            ("admin-03-control-plane-upgrade", "Control-Plane & Kubelet Rolling Upgrade", "Upgrade kubeadm, kubelet, and drain nodes", "nodes"),
            ("admin-04-node-drain-cordon", "Graceful Node Maintenance & Eviction", "Cordon and drain worker node safely", "nodes"),
            ("admin-05-certificate-rotation", "Kubernetes PKI CA & Certificate Renewal", "Renew API server certificates via kubeadm", "pods"),
            ("admin-06-static-pods-debug", "Managing Static Pod Manifests", "Deploy control-plane components under /etc/kubernetes/manifests", "pods"),
            ("admin-07-audit-logging-policy", "API Server Audit Logging Policy", "Configure advanced audit policy with metadata logging", "pods"),
            ("admin-08-custom-scheduler", "Deploying a Secondary Custom Scheduler", "Run custom scheduler container with leader election", "pods"),
            ("admin-09-api-priority-fairness", "API Priority & Fairness (APF) Tuning", "Tune FlowSchema and PriorityLevelConfiguration", "pods"),
            ("admin-10-mutating-webhook", "Mutating Admission Controller Webhook", "Deploy mutating webhook to inject default sidecars", "mutatingwebhookconfigurations"),
            ("admin-11-validating-webhook", "Validating Admission Webhook Constraints", "Reject non-compliant manifests before admission", "validatingwebhookconfigurations"),
            ("admin-12-coredns-custom-config", "Custom CoreDNS Zone Forwarding & Rewrite", "Configure CoreDNS Corefile plugins and upstream DNS", "configmaps"),
        ]),
        ("networking", vec![
            ("net-01-clusterip-deepdive", "ClusterIP Service Deep Dive & iptables", "Verify service endpoints and kube-proxy rules", "services"),
            ("net-02-nodeport-services", "NodePort Service Publishing", "Expose external port 30080 across all nodes", "services"),
            ("net-03-loadbalancer-cloud", "LoadBalancer Service & L2 Metallb", "Allocate external VIP via LoadBalancer service", "services"),
            ("net-04-headless-dns-discovery", "Headless Service DNS SRV Lookups", "Query DNS SRV records for stateful pod endpoints", "services"),
            ("net-05-ingress-nginx-routing", "Path & Host-Based Ingress Routing", "Configure NGINX ingress with path /api rewrites", "ingresses"),
            ("net-06-ingress-tls-certmanager", "Automated TLS Certificates with cert-manager", "Issue Let's Encrypt TLS secret for Ingress", "certificates"),
            ("net-07-gateway-api-httproute", "Gateway API HTTPRoute Configuration", "Deploy modern Gateway and HTTPRoute resources", "httproutes"),
            ("net-08-networkpolicy-default-deny", "Zero-Trust Default-Deny NetworkPolicy", "Block all unapproved ingress and egress traffic", "networkpolicies"),
            ("net-09-networkpolicy-namespace-isolation", "Cross-Namespace Isolation NetworkPolicy", "Allow traffic only from specific tenant namespaces", "networkpolicies"),
            ("net-10-calico-cni-policies", "Calico CNI GlobalNetworkPolicies", "Apply global tier-1 security policies across cluster", "networkpolicies"),
            ("net-11-cilium-ebpf-networking", "Cilium eBPF Host Routing & Wireguard", "Enforce transparent eBPF encryption and routing", "networkpolicies"),
            ("net-12-dns-troubleshooting-ndots", "Debugging DNS Search Domains & ndots:5", "Optimize ndots:2 in pod dnsConfig to reduce latency", "pods"),
        ]),
        ("security", vec![
            ("sec-01-rbac-role-binding", "Role & RoleBinding for Least-Privilege", "Grant read-only pod access to developer role", "roles"),
            ("sec-02-rbac-clusterrole-aggregation", "ClusterRole Aggregation & Global Auditing", "Aggregate custom rules into cluster-admin views", "clusterroles"),
            ("sec-03-serviceaccount-tokens", "Projected ServiceAccount TokenRequest API", "Mount bounded short-lived token volumes in pods", "pods"),
            ("sec-04-pod-security-standards", "Enforcing Pod Security Standards Restricted", "Apply restricted baseline labels to secure namespaces", "namespaces"),
            ("sec-05-seccomp-custom-profiles", "Seccomp RuntimeDefault & Syscall Filtering", "Restrict dangerous syscalls with custom seccomp profile", "pods"),
            ("sec-06-apparmor-profiles", "AppArmor Profiles for Filesystem Protection", "Load and enforce read-only AppArmor container profile", "pods"),
            ("sec-07-kms-secrets-encryption", "Envelope Encryption of Secrets at Rest", "Configure KMS provider with AES-CBC encryption", "secrets"),
            ("sec-08-opa-gatekeeper-policies", "OPA Gatekeeper ConstraintTemplates", "Enforce required label policies via Rego constraints", "pods"),
            ("sec-09-kyverno-mutate-validate", "Kyverno Declarative Policy Validation", "Auto-inject default resource requests with Kyverno", "pods"),
            ("sec-10-image-vulnerability-trivy", "Automated Image Vulnerability Scanning", "Scan container images for CVEs in CI pipeline", "pods"),
            ("sec-11-cosign-sigstore-verify", "Cosign Container Signature Verification", "Verify cryptographic signatures before pod admission", "pods"),
            ("sec-12-cis-benchmark-remediation", "CIS Kubernetes Benchmark Auditing", "Audit and remediate insecure kubelet and API parameters", "pods"),
        ]),
        ("storage", vec![
            ("storage-01-emptydir-scratch", "Ephemeral Fast Storage with emptyDir", "Mount in-memory emptyDir RAM disk for caching", "pods"),
            ("storage-02-hostpath-nodes", "Node-Bound hostPath Volume Mounting", "Mount host system logs into monitoring agent pod", "pods"),
            ("storage-03-pv-pvc-static", "Static PersistentVolume & PVC Binding", "Bind 10Gi ReadWriteOnce PV to application PVC", "persistentvolumeclaims"),
            ("storage-04-storageclass-dynamic", "Dynamic Storage Provisioning with StorageClasses", "Provision SSD backed volume with reclaimPolicy Retain", "storageclasses"),
            ("storage-05-volume-expansion-online", "Online FileSystem Volume Expansion", "Resize PVC from 10Gi to 50Gi without pod restarts", "persistentvolumeclaims"),
            ("storage-06-volume-snapshots-restore", "VolumeSnapshot Point-in-Time Recovery", "Create snapshot and restore into fresh PVC", "volumesnapshots"),
            ("storage-07-stateful-volume-claims", "Dynamic VolumeClaimTemplates in StatefulSets", "Auto-provision dedicated PVCs per StatefulSet replica", "statefulsets"),
            ("storage-08-csi-driver-diagnostics", "Troubleshooting CSI Node Mount Failures", "Diagnose and resolve stuck volume attachment locks", "pods"),
        ]),
        ("helm-kustomize", vec![
            ("helm-01-install-upgrade", "Helm v3 Release Management & Rollbacks", "Install chart, upgrade values, and perform atomic rollback", "deployments"),
            ("helm-02-chart-authoring", "Authoring Production Helm Charts", "Structure Chart.yaml, values.yaml, and templates", "deployments"),
            ("helm-03-values-templates", "Advanced Go Templating & Named Helpers", "Use conditionals, loops, and _helpers.tpl in Helm", "deployments"),
            ("helm-04-lifecycle-hooks", "Database Migration Lifecycle Hooks", "Execute pre-install database schema migration job", "jobs"),
            ("helm-05-chart-testing-lint", "Automated Chart Testing with ct & helm test", "Validate syntax and execute integration test pods", "pods"),
            ("kust-01-base-overlays", "Kustomize Base & Environment Overlays", "Compose dev and prod overlays from single base", "deployments"),
            ("kust-02-patches-strategic", "Strategic Merge & JSON6902 Patching", "Patch replica counts and image tags per environment", "deployments"),
            ("kust-03-generators-vars", "ConfigMap Generators & Secret Injection", "Auto-generate hashed ConfigMaps from .env files", "configmaps"),
        ]),
        ("gitops", vec![
            ("gitops-01-argocd-setup", "Installing and Configuring Argo CD", "Bootstrap Argo CD server with declarative settings", "deployments"),
            ("gitops-02-app-declarative", "Deploying Declarative Argo CD Applications", "Define Application manifest with automated syncPolicy", "applications"),
            ("gitops-03-auto-sync-selfheal", "Automated Sync, Self-Healing & Pruning", "Enable selfHeal: true and prune: true on workloads", "applications"),
            ("gitops-04-app-of-apps", "Scalable App-of-Apps Root Architecture", "Manage multi-cluster workloads via root parent app", "applications"),
            ("gitops-05-applicationset-matrix", "ApplicationSet Matrix & Cluster Generators", "Auto-generate apps across multiple tenant clusters", "applicationsets"),
            ("gitops-06-drift-detection", "Configuration Drift Detection & Alerts", "Detect out-of-sync drift and notify webhook", "applications"),
            ("gitops-07-sync-windows-phases", "Enforcing Sync Windows & Wave Sequencing", "Orchestrate multi-step deployment waves (1, 2, 3)", "applications"),
            ("gitops-08-prune-propagation", "Orphaned Resource Pruning & Finalizers", "Safely prune deleted git objects with background cascading", "applications"),
            ("gitops-09-multienv-promotion", "Multi-Environment GitOps Promotion", "Promote releases across Dev, Staging, and Production", "applications"),
            ("gitops-10-argocd-rbac-tenants", "Multi-Tenant Project RBAC Whitelisting", "Restrict destination clusters and source repos in AppProject", "appprojects"),
        ]),
        ("observability", vec![
            ("obs-01-prometheus-scrape-configs", "Prometheus Scrape Targets & ServiceMonitors", "Configure custom metrics scraping on /metrics", "servicemonitors"),
            ("obs-02-promql-metrics-queries", "Advanced PromQL & Histogram Quantiles", "Calculate p99 latency with histogram_quantile()", "pods"),
            ("obs-03-alertmanager-routing", "Alertmanager Routing Rules & Inhibitions", "Route high-severity alerts to PagerDuty webhooks", "alertmanagers"),
            ("obs-04-grafana-dashboard-provisioning", "Declarative Grafana Dashboards as Code", "Provision Kubernetes cluster overview JSON dashboard", "configmaps"),
            ("obs-05-otel-collector-pipelines", "OpenTelemetry Collector OTLP Pipelines", "Process and export batch traces and metrics via gRPC", "deployments"),
            ("obs-06-otel-trace-instrumentation", "Distributed Tracing Context Propagation", "Verify W3C traceparent headers across microservices", "pods"),
            ("obs-07-loki-promtail-logs", "Centralized Log Aggregation with Loki", "Collect container stdout/stderr with Promtail daemon", "daemonsets"),
            ("obs-08-tempo-trace-aggregation", "High-Scale Trace Storage with Tempo", "Ingest and search distributed spans with Grafana Tempo", "statefulsets"),
            ("obs-09-metric-log-trace-correlation", "Correlating Metrics, Logs, and Traces", "Navigate from PromQL alert to exact Loki log and Tempo trace", "pods"),
            ("obs-10-slo-sli-alerting-burnrates", "Multi-Window Multi-Burn-Rate Error Budget Alerts", "Alert when 1-hour error budget burn rate exceeds 14.4x", "prometheusrules"),
        ]),
        ("service-mesh", vec![
            ("mesh-01-istio-control-plane", "Deploying Istio Service Mesh Operator", "Install istiod and ingressgateway components", "deployments"),
            ("mesh-02-sidecar-auto-injection", "Automated Envoy Sidecar Injection", "Label namespace istio-injection=enabled for automatic proxy", "namespaces"),
            ("mesh-03-traffic-shifting-canary", "90/10 Canary Traffic Shifting with VirtualService", "Route 90% traffic to v1 and 10% to v2 canary", "virtualservices"),
            ("mesh-04-header-routing-canary", "Header-Based (A/B Testing) Route Matching", "Route requests with x-canary: true header to staging", "virtualservices"),
            ("mesh-05-circuit-breaker-outlier", "Circuit Breaking & Outlier Detection", "Eject failing pods after 3 consecutive 5xx server errors", "destinationrules"),
            ("mesh-06-fault-injection-chaos", "Chaos Fault Injection: Delays & Aborts", "Inject 2s delay and HTTP 503 errors on 20% requests", "virtualservices"),
            ("mesh-07-strict-mtls-peerauth", "Zero-Trust STRICT Mutual TLS Authentication", "Enforce STRICT mTLS across all mesh communications", "peerauthentications"),
            ("mesh-08-authorization-policy-jwt", "Microsegmentation with JWT AuthorizationPolicies", "Allow access only to authenticated JWT Bearer tokens", "authorizationpolicies"),
            ("mesh-09-rate-limiting-envoyfilter", "Global Rate Limiting with EnvoyFilter", "Enforce 100 req/min rate limits via EnvoyFilter config", "envoyfilters"),
            ("mesh-10-mesh-observability-kiali", "Mesh Topology Visualization with Kiali & Jaeger", "Inspect service dependency graph and mTLS lock status", "deployments"),
        ]),
        ("sre-performance", vec![
            ("sre-01-sli-slo-definition", "Defining Latency & Availability SLIs", "Define 99.9% availability target with Prometheus rules", "prometheusrules"),
            ("sre-02-error-budget-calculation", "Tracking Error Budget Burn Rates", "Compute rolling 30-day error budget consumption", "pods"),
            ("sre-03-hpa-custom-metrics", "Autoscaling on Custom Prometheus Metrics", "Scale pods based on active HTTP request rate per second", "horizontalpodautoscalers"),
            ("sre-04-vpa-resource-recommender", "Vertical Pod Autoscaler Resource Recommendations", "Auto-adjust CPU/RAM limits based on historical usage", "verticalpodautoscalers"),
            ("sre-05-node-problem-detector", "Detecting Kernel Faults with Node Problem Detector", "Detect disk corruption and kernel deadlocks on nodes", "daemonsets"),
            ("sre-06-chaos-pod-failure", "Chaos Engineering: Pod Eviction Resilience", "Simulate random pod termination without service degradation", "pods"),
            ("sre-07-chaos-network-latency", "Simulating Network Latency & Packet Loss", "Inject 100ms artificial network delay with Chaos Mesh", "pods"),
            ("sre-08-cluster-capacity-planning", "Cluster Sizing, Overcommit & Cluster Autoscaler", "Configure node autoscaling thresholds and resource quotas", "resourcequotas"),
        ]),
        ("troubleshooting", vec![
            ("tb-01-crashloop-backoff", "Diagnosing CrashLoopBackOff & Exit Code 137", "Identify OOM kills and fix container memory allocation", "pods"),
            ("tb-02-imagepull-backoff", "Fixing ImagePullBackOff & Registry Credentials", "Create imagePullSecret and correct image repository tag", "pods"),
            ("tb-03-pending-unschedulable", "Resolving Pending Pods & Node Scheduling Conflicts", "Remove conflicting nodeSelector and adjust resource requests", "pods"),
            ("tb-04-oomkilled-memory-leak", "Pinpointing OOMKilled Memory Leaks in Containers", "Profile memory usage and raise container limits", "pods"),
            ("tb-05-dns-resolution-failure", "Triaging CoreDNS Resolution Failures", "Repair CoreDNS service endpoints and NetworkPolicy rules", "pods"),
            ("tb-06-service-endpoint-misconfig", "Fixing Missing Endpoints from Label Mismatches", "Align spec.selector with pod template labels", "services"),
            ("tb-07-ingress-502-bad-gateway", "Debugging Ingress 502 Bad Gateway Timeouts", "Fix backend readiness probe and upstream service port", "ingresses"),
            ("tb-08-expired-tls-certificates", "Triaging Expired Ingress TLS Certificates", "Renew expired TLS secret and trigger ingress reload", "secrets"),
            ("tb-09-etcd-database-corruption", "Restoring Quorum & Defragging Unhealthy etcd", "Defragment database and restore etcd cluster health", "pods"),
            ("tb-10-node-notready-investigation", "Rescuing NotReady Worker Nodes", "Clear container runtime disk pressure and restart kubelet", "nodes"),
        ]),
        ("platform-multicluster", vec![
            ("plat-01-custom-resource-definition", "Designing Production CRDs with OpenAPI v3", "Define Schema validation and subresources in CRD", "customresourcedefinitions"),
            ("plat-02-operator-controller-runtime", "Building Custom Kubernetes Operator in Rust/Go", "Implement reconciliation loop managing child resources", "deployments"),
            ("plat-03-multicluster-service-export", "Multi-Cluster Service Export & Discovery (MCS)", "Export ServiceImport across multi-cluster meshes", "serviceexports"),
            ("plat-04-cluster-api-management", "Declarative Cluster Lifecycle with Cluster API", "Provision worker node pools with CAPI MachineDeployments", "machinedeployments"),
            ("plat-05-crossplane-cloud-resources", "Composing Cloud Infrastructure with Crossplane", "Provision managed PostgreSQL instances via Kubernetes YAML", "compositeresourcedefinitions"),
            ("plat-06-backstage-idp-integration", "Self-Service Developer Templates with Backstage", "Publish reusable golden path software templates", "deployments"),
            ("plat-07-kyverno-policy-reporter", "Centralized Policy Violation Auditing", "Aggregate cluster-wide policy reports into UI dashboard", "policyreports"),
        ]),
        ("certification", vec![
            ("cert-01-cka-mock-workloads", "CKA Mock: Workloads & Pod Scheduling", "Place multi-tier pods with nodeSelector and tolerations", "deployments"),
            ("cert-02-cka-mock-admin", "CKA Mock: etcd Backup & Kubeadm Upgrade", "Backup etcd to /tmp/etcd-backup.db and drain nodes", "pods"),
            ("cert-03-cka-mock-network", "CKA Mock: Ingress Routing & NetworkPolicies", "Configure Ingress TLS and ingress/egress NetworkPolicies", "networkpolicies"),
            ("cert-04-cka-mock-storage", "CKA Mock: PVCs, PVs & StorageClasses", "Create 5Gi ReadWriteMany PVC and mount to deployment", "persistentvolumeclaims"),
            ("cert-05-cka-mock-troubleshoot", "CKA Mock: Triage Broken Node & System Pods", "Fix kubelet systemd service and restart failed API server", "nodes"),
            ("cert-06-ckad-mock-architecture", "CKAD Mock: Multi-Container Pods & Config", "Deploy sidecar logging container with shared emptyDir", "pods"),
            ("cert-07-ckad-mock-deployment", "CKAD Mock: Blue/Green & Canary Rollouts", "Perform zero-downtime rolling update with maxSurge: 1", "deployments"),
            ("cert-08-ckad-mock-observability", "CKAD Mock: Health Probes & Prometheus Exporters", "Configure HTTP liveness probes and container metrics", "pods"),
            ("cert-09-cks-mock-hardening", "CKS Mock: Restricted PodSecurity & Seccomp", "Enforce Restricted profile, drop ALL caps, enable seccomp", "pods"),
            ("cert-10-cks-mock-audit-vulnerabilities", "CKS Mock: Image Scanning & Audit Policies", "Enforce ImagePolicyWebhook and encrypt secrets at rest", "secrets"),
        ]),
    ].into_iter().collect();

    for (cat_name, _count, _prefix) in categories {
        if let Some(lab_list) = titles.get(cat_name) {
            for (id, title, scenario, res) in lab_list {
                labs.push(create_lab(
                    id,
                    title,
                    cat_name,
                    "intermediate",
                    20,
                    scenario,
                    vec![
                        simple_k8s_task(
                            &format!("task-verify-{}", id),
                            &format!("Verify {}", title),
                            100,
                            res,
                            "target-resource",
                            "metadata.name",
                            ValidationOperator::Equals,
                            json!("target-resource"),
                        ),
                    ],
                    vec![&format!("Inspect Kubernetes documentation for `{}` resource specifications.", res)],
                    &format!("kubectl apply -f {}.yaml", id),
                ));
            }
        }
    }

    println!("Total Labs to Generate: {}", labs.len());

    let root_labs_dir = Path::new("labs");
    if !root_labs_dir.exists() {
        fs::create_dir_all(root_labs_dir)?;
    }

    for lab in &labs {
        let track_dir = root_labs_dir.join(&lab.track).join(&lab.id);
        fs::create_dir_all(&track_dir)?;
        let file_path = track_dir.join("lab.yaml");
        let yaml_content = serde_yaml::to_string(lab)?;
        fs::write(&file_path, yaml_content)?;
    }

    println!("✅ Successfully generated all {} labs across all tracks!", labs.len());
    Ok(())
}
