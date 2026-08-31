use crate::models::{InterviewQuestion, LessonDetail, TrackSummary};

pub fn get_default_tracks() -> Vec<TrackSummary> {
    vec![
        TrackSummary {
            id: "track-foundations".to_string(),
            slug: "foundations".to_string(),
            title: "Cloud-Native & Linux Foundations".to_string(),
            description: "Linux systems engineering, shell scripting, namespaces, cgroups, and OCI container fundamentals with Docker and Podman.".to_string(),
            icon: "Terminal".to_string(),
            difficulty: "beginner".to_string(),
            order: 1,
            total_lessons: 8,
            total_xp: 1500,
        },
        TrackSummary {
            id: "track-kubernetes".to_string(),
            slug: "kubernetes".to_string(),
            title: "Kubernetes Core Architecture & Workloads".to_string(),
            description: "Master Pods, Deployments, Services, ConfigMaps, Secrets, Storage, and Declarative manifests in real Kubernetes clusters.".to_string(),
            icon: "Boxes".to_string(),
            difficulty: "beginner".to_string(),
            order: 2,
            total_lessons: 15,
            total_xp: 2500,
        },
        TrackSummary {
            id: "track-k8s-admin".to_string(),
            slug: "k8s-admin".to_string(),
            title: "Cluster Administration & etcd Operations".to_string(),
            description: "Bootstrap clusters with kubeadm, handle control-plane high availability, etcd snapshots and disaster recovery, and node maintenance.".to_string(),
            icon: "Server".to_string(),
            difficulty: "intermediate".to_string(),
            order: 3,
            total_lessons: 12,
            total_xp: 1800,
        },
        TrackSummary {
            id: "track-networking".to_string(),
            slug: "networking".to_string(),
            title: "Cloud-Native Networking, CNI & Gateway API".to_string(),
            description: "Deep dive into CNI plugins (Calico/Cilium), CoreDNS resolution, Ingress Controllers, eBPF data planes, and Kubernetes Gateway API.".to_string(),
            icon: "Network".to_string(),
            difficulty: "intermediate".to_string(),
            order: 4,
            total_lessons: 12,
            total_xp: 2100,
        },
        TrackSummary {
            id: "track-security".to_string(),
            slug: "security".to_string(),
            title: "Kubernetes Security, RBAC & Policy Hardening".to_string(),
            description: "Implement RBAC, Pod Security Standards, NetworkPolicies, Seccomp profiles, image vulnerability scanning, and CIS benchmarks.".to_string(),
            icon: "ShieldCheck".to_string(),
            difficulty: "intermediate".to_string(),
            order: 5,
            total_lessons: 12,
            total_xp: 2250,
        },
        TrackSummary {
            id: "track-helm".to_string(),
            slug: "helm".to_string(),
            title: "Packaging with Helm & Kustomize".to_string(),
            description: "Author production Helm charts, manage chart repositories, leverage Go templates, and apply dry Kustomize overlays.".to_string(),
            icon: "Package".to_string(),
            difficulty: "intermediate".to_string(),
            order: 6,
            total_lessons: 8,
            total_xp: 1500,
        },
        TrackSummary {
            id: "track-gitops".to_string(),
            slug: "gitops".to_string(),
            title: "GitOps & Continuous Delivery with Argo CD".to_string(),
            description: "Implement declarative GitOps workflows, App-of-Apps pattern, automated sync policies, drift detection, and automated rollbacks.".to_string(),
            icon: "GitBranch".to_string(),
            difficulty: "intermediate".to_string(),
            order: 7,
            total_lessons: 10,
            total_xp: 2000,
        },
        TrackSummary {
            id: "track-observability".to_string(),
            slug: "observability".to_string(),
            title: "OpenTelemetry, Prometheus & Grafana".to_string(),
            description: "End-to-end distributed tracing with OpenTelemetry, metric collection with Prometheus, PromQL alerting, and Grafana dashboarding.".to_string(),
            icon: "Activity".to_string(),
            difficulty: "advanced".to_string(),
            order: 8,
            total_lessons: 10,
            total_xp: 2400,
        },
        TrackSummary {
            id: "track-service-mesh".to_string(),
            slug: "service-mesh".to_string(),
            title: "Service Mesh with Istio & Envoy Proxy".to_string(),
            description: "Traffic shifting, canary releases, mutual TLS (mTLS), fault injection, rate limiting, and Envoy sidecar telemetry.".to_string(),
            icon: "Layers".to_string(),
            difficulty: "advanced".to_string(),
            order: 9,
            total_lessons: 10,
            total_xp: 2300,
        },
        TrackSummary {
            id: "track-sre".to_string(),
            slug: "sre".to_string(),
            title: "Site Reliability Engineering & SLOs".to_string(),
            description: "Define meaningful SLIs/SLOs, error budget burn rates, alerting thresholds, HPA/VPA autoscaling, and capacity planning.".to_string(),
            icon: "Gauge".to_string(),
            difficulty: "advanced".to_string(),
            order: 10,
            total_lessons: 8,
            total_xp: 2000,
        },
        TrackSummary {
            id: "track-platform-eng".to_string(),
            slug: "platform-eng".to_string(),
            title: "Platform Engineering & Multi-Cluster".to_string(),
            description: "Build Internal Developer Platforms (IDPs), write Kubernetes Operators and CRDs in Go/Rust, and manage multi-cluster fleets.".to_string(),
            icon: "Cpu".to_string(),
            difficulty: "expert".to_string(),
            order: 11,
            total_lessons: 7,
            total_xp: 2500,
        },
        TrackSummary {
            id: "track-incidents".to_string(),
            slug: "incidents".to_string(),
            title: "Production Incident Response & Chaos".to_string(),
            description: "Real-world break-fix simulations: debug crashloops, network partitions, DNS degradation, expired certs, and GitOps sync deadlocks.".to_string(),
            icon: "AlertTriangle".to_string(),
            difficulty: "expert".to_string(),
            order: 12,
            total_lessons: 10,
            total_xp: 3000,
        },
    ]
}

pub fn get_default_lessons() -> Vec<LessonDetail> {
    vec![
        LessonDetail {
            id: "k8s-pod-architecture".to_string(),
            track_slug: "kubernetes".to_string(),
            title: "Understanding the Pod: The Atomic Unit of Kubernetes".to_string(),
            slug: "understanding-pods".to_string(),
            order: 1,
            duration_minutes: 15,
            xp: 150,
            summary: "Learn why Kubernetes uses Pods instead of bare containers, how shared network namespaces and volumes work, and how the pause container coordinates lifecycle.".to_string(),
            content_markdown: r#"# Understanding Pods: The Fundamental Primitive

In Kubernetes, you never deploy individual containers directly. Instead, you deploy **Pods**. A Pod is the smallest deployable compute unit that you can create and manage in Kubernetes.

## The Role of the Pause Container
Every Pod includes an infrastructure container called `pause` that holds the Linux network namespace (`NET`) and IPC namespace open. When user containers within the pod start or restart, they attach to the network namespace owned by the `pause` container.

### Why Pods Matter:
1. **Shared Network**: All containers in a Pod share `localhost` and the Pod IP address.
2. **Shared Storage**: Containers in a Pod can mount shared `emptyDir` or persistent volumes.
3. **Co-scheduling**: Kubernetes guarantees that all containers in a Pod are placed on the exact same physical node.
"#.to_string(),
            concepts: vec!["pod".to_string(), "pause-container".to_string(), "network-namespace".to_string()],
            prerequisites: vec![],
            associated_lab_id: Some("k8s-pod-basics".to_string()),
            associated_quiz_id: Some("quiz-k8s-pods".to_string()),
            trivia: vec![
                "The pause container uses almost 0 CPU and 0 MB RAM, serving only to keep the PID and NET namespaces alive.".to_string(),
                "Pods were inspired by Borg allocations at Google.".to_string(),
            ],
            interview_questions: vec![
                InterviewQuestion {
                    question: "Why does Kubernetes use Pods instead of running raw Docker containers?".to_string(),
                    answer: "Pods allow multiple co-located containers (such as logging sidecars, proxy proxies) to share the same network namespace (localhost), storage volumes, and lifecycle constraints on a single node.".to_string(),
                    key_points: vec!["Shared IPC and Network namespace".to_string(), "Atomic scheduling on the same node".to_string()],
                }
            ],
            mistakes_to_avoid: vec![
                "Don't put independent, loosely coupled services in the same Pod. Use separate Pods and Services instead.".to_string(),
            ],
            production_tips: vec![
                "Always set container resource requests and limits to enable predictable scheduling and QoS classes.".to_string(),
            ],
        },
        LessonDetail {
            id: "k8s-deployments-rollouts".to_string(),
            track_slug: "kubernetes".to_string(),
            title: "Deployments, ReplicaSets & Zero-Downtime Rollouts".to_string(),
            slug: "deployments-and-rollouts".to_string(),
            order: 2,
            duration_minutes: 20,
            xp: 200,
            summary: "Master declarative workload updates with Deployments, RollingUpdate strategies, revision history, and instant rollbacks.".to_string(),
            content_markdown: r#"# Deployments & Rolling Updates

A **Deployment** provides declarative updates for Pods and ReplicaSets. You describe a desired state in a Deployment, and the Deployment Controller changes the actual state to the desired state at a controlled rate.

## RollingUpdate Mechanics
By default, Deployments use the `RollingUpdate` strategy with:
- `maxSurge`: 25% (maximum extra pods allowed during update)
- `maxUnavailable`: 25% (maximum unavailable pods allowed during update)

```yaml
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
```
"#.to_string(),
            concepts: vec!["deployment".to_string(), "replicaset".to_string(), "rolling-update".to_string()],
            prerequisites: vec!["k8s-pod-architecture".to_string()],
            associated_lab_id: Some("k8s-deployments-scaling".to_string()),
            associated_quiz_id: Some("quiz-k8s-deployments".to_string()),
            trivia: vec![
                "Deployments do not manage Pods directly; they manage ReplicaSets, which in turn create Pods.".to_string(),
            ],
            interview_questions: vec![
                InterviewQuestion {
                    question: "How do you perform zero-downtime rollouts in Kubernetes?".to_string(),
                    answer: "Configure strategy type RollingUpdate with maxUnavailable: 0 and maxSurge: 1, combined with reliable Readiness Probes and terminationGracePeriodSeconds.".to_string(),
                    key_points: vec!["maxUnavailable: 0".to_string(), "Readiness probe checks".to_string()],
                }
            ],
            mistakes_to_avoid: vec![
                "Never update pod template without defining readiness probes; traffic might route to unready containers.".to_string(),
            ],
            production_tips: vec![
                "Keep revisionHistoryLimit set to a reasonable number (e.g. 5-10) to avoid bloated etcd storage.".to_string(),
            ],
        },
        LessonDetail {
            id: "linux-namespaces-cgroups".to_string(),
            track_slug: "foundations".to_string(),
            title: "Linux Kernel Primitives: Namespaces & cgroups v2".to_string(),
            slug: "namespaces-and-cgroups".to_string(),
            order: 1,
            duration_minutes: 15,
            xp: 150,
            summary: "Deep dive into kernel isolation primitives: PID, Mount, Network, IPC, UTS, and User namespaces along with cgroups v2 resource quotas.".to_string(),
            content_markdown: r#"# Linux Namespaces & cgroups

Containers are not virtual machines; they are standard Linux processes running with restricted views of the system kernel.

## 1. Namespaces (What a process can see)
- **PID**: Process ID isolation
- **NET**: Network devices, ports, routes
- **MNT**: Filesystem mount points
- **UTS**: Hostname and domain name
- **IPC**: Inter-process communication / shared memory
- **USER**: UID/GID mapping for rootless containers

## 2. cgroups (What a process can use)
Control Groups limit and monitor CPU, memory, IO, and pids.
"#.to_string(),
            concepts: vec!["namespaces".to_string(), "cgroups-v2".to_string(), "kernel-isolation".to_string()],
            prerequisites: vec![],
            associated_lab_id: Some("linux-03-namespaces-isolation".to_string()),
            associated_quiz_id: Some("quiz-linux-namespaces".to_string()),
            trivia: vec![
                "cgroups was originally created by Google engineers in 2006 under the name 'Process Containers'.".to_string(),
            ],
            interview_questions: vec![
                InterviewQuestion {
                    question: "What is the primary difference between Linux namespaces and cgroups?".to_string(),
                    answer: "Namespaces isolate WHAT a process can see (network, PIDs, mounts), while cgroups control HOW MUCH resources a process can consume (CPU, RAM, IO).".to_string(),
                    key_points: vec!["Visibility vs Resource limit".to_string(), "cgroups v2 unified hierarchy".to_string()],
                }
            ],
            mistakes_to_avoid: vec![
                "Do not run containers as root (UID 0) inside the container without User Namespaces enabled.".to_string(),
            ],
            production_tips: vec![
                "Always verify cgroups v2 is active on container host nodes for accurate memory pressure stall metrics.".to_string(),
            ],
        },
        LessonDetail {
            id: "gitops-argocd-workflow".to_string(),
            track_slug: "gitops".to_string(),
            title: "GitOps Core Architecture with Argo CD".to_string(),
            slug: "argocd-gitops-architecture".to_string(),
            order: 1,
            duration_minutes: 20,
            xp: 200,
            summary: "Implement declarative GitOps continuous delivery, automated reconciliation loops, self-healing, and App-of-Apps patterns.".to_string(),
            content_markdown: r#"# GitOps & Argo CD

GitOps is an operational framework where the desired state of a Kubernetes cluster is stored in Git version control as the single source of truth.

## Argo CD Architecture
- **API Server**: Exposes Web UI & gRPC API.
- **Repository Server**: Clones git repositories and renders manifests (Helm/Kustomize).
- **Application Controller**: Continuous reconciliation loop comparing Git vs Live Cluster state.
"#.to_string(),
            concepts: vec!["gitops".to_string(), "argocd".to_string(), "reconciliation-loop".to_string(), "drift-detection".to_string()],
            prerequisites: vec!["k8s-deployments-rollouts".to_string()],
            associated_lab_id: Some("gitops-01-argocd-setup".to_string()),
            associated_quiz_id: Some("quiz-gitops-argocd".to_string()),
            trivia: vec![
                "The term GitOps was originally coined by Alexis Richardson, CEO of Weaveworks in 2017.".to_string(),
            ],
            interview_questions: vec![
                InterviewQuestion {
                    question: "How does Argo CD detect and handle configuration drift?".to_string(),
                    answer: "Argo CD watches the Kubernetes API and compares live cluster resources with target Git manifests. If drift occurs, status turns OutOfSync, and if selfHeal is true, it automatically overwrites live state to match Git.".to_string(),
                    key_points: vec!["Continuous reconciliation".to_string(), "selfHeal and prune flags".to_string()],
                }
            ],
            mistakes_to_avoid: vec![
                "Avoid applying manual `kubectl edit` in GitOps-managed clusters because Argo CD will immediately overwrite changes.".to_string(),
            ],
            production_tips: vec![
                "Use AppProjects to restrict destination namespaces and cluster credentials for multi-tenant teams.".to_string(),
            ],
        },
        LessonDetail {
            id: "mesh-istio-mtls-canary".to_string(),
            track_slug: "service-mesh".to_string(),
            title: "Service Mesh: Envoy Sidecars, mTLS & Canary Releases".to_string(),
            slug: "istio-mtls-and-canary".to_string(),
            order: 1,
            duration_minutes: 20,
            xp: 200,
            summary: "Understand Envoy proxy sidecar interception, zero-trust mutual TLS, VirtualServices for traffic shifting, and DestinationRules.".to_string(),
            content_markdown: r#"# Istio Service Mesh & Envoy

A Service Mesh provides a dedicated infrastructure layer for managing service-to-service communication with security, observability, and traffic control.

## Envoy Sidecar Interception
`iptables` redirects incoming and outgoing container TCP traffic into the Envoy sidecar proxy (port 15001/15006), enabling transparent mTLS and metrics capture.

## Traffic Shifting (Canary)
```yaml
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: frontend-vs
spec:
  hosts:
  - frontend
  http:
  - route:
    - destination:
        host: frontend
        subset: v1
      weight: 90
    - destination:
        host: frontend
        subset: v2
      weight: 10
```
"#.to_string(),
            concepts: vec!["service-mesh".to_string(), "istio".to_string(), "mtls".to_string(), "envoy".to_string(), "canary".to_string()],
            prerequisites: vec!["k8s-pod-architecture".to_string()],
            associated_lab_id: Some("mesh-03-traffic-shifting-canary".to_string()),
            associated_quiz_id: Some("quiz-istio-mesh".to_string()),
            trivia: vec![
                "Envoy was created by Matt Klein at Lyft to solve the monolithic architecture migration to microservices.".to_string(),
            ],
            interview_questions: vec![
                InterviewQuestion {
                    question: "What is the difference between an Istio VirtualService and a DestinationRule?".to_string(),
                    answer: "A VirtualService defines routing rules (traffic shifting, path rewrites, headers), while a DestinationRule defines policies that apply AFTER routing has occurred (mTLS settings, subsets, circuit breaking, load balancer algorithms).".to_string(),
                    key_points: vec!["VirtualService: Routing".to_string(), "DestinationRule: Post-routing policies".to_string()],
                }
            ],
            mistakes_to_avoid: vec![
                "Don't enforce STRICT mTLS across a cluster before verifying that all clients have sidecars injected, or un-injected clients will be rejected.".to_string(),
            ],
            production_tips: vec![
                "Configure outlier detection (circuit breaking) on DestinationRules to automatically eject unhealthy upstream instances.".to_string(),
            ],
        },
    ]
}
