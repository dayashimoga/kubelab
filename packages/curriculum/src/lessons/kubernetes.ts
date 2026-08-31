import { Lesson } from '@kubelab/shared-types';

export const KUBERNETES_LESSONS: Lesson[] = [
  {
    id: 'k8s-pod-architecture',
    moduleId: 'mod-k8s-core',
    trackSlug: 'kubernetes',
    title: 'Understanding the Pod: The Atomic Unit of Kubernetes',
    slug: 'understanding-pods',
    order: 1,
    durationMinutes: 15,
    xp: 150,
    summary:
      'Learn why Kubernetes uses Pods instead of bare containers, how shared network namespaces and volumes work, and how the pause container coordinates lifecycle.',
    contentMarkdown: `
# Understanding Pods: The Fundamental Primitive

In Kubernetes, you never deploy individual containers directly. Instead, you deploy **Pods**. A Pod is the smallest deployable compute unit that you can create and manage in Kubernetes.

## Why Pods Instead of Individual Containers?

A Pod represents a single instance of a running process in your cluster. It encapsulates:
1. **One or more tightly coupled application containers**
2. **Shared storage volumes**
3. **A unique network IP address**
4. **Shared Linux namespaces** (IPC, Network, and UTS)

\`\`\`mermaid
graph TD
    subgraph Pod ["Pod: web-pod (IP: 10.244.1.45)"]
        Pause["Pause Container (Holds Network NS)"]
        Main["Main Container (Nginx :80)"]
        Sidecar["Sidecar Container (Log Shipper)"]
        Vol[("Shared Volume (/var/log)")]
    end
    Main -->|Writes logs| Vol
    Sidecar -->|Reads logs| Vol
\`\`\`

## The Role of the "Pause" Container

Every Pod in Kubernetes includes a hidden infrastructure container called **pause** (or \`pause-amd64\`). 
The pause container starts first, creates the network namespace, and binds the Pod's IP address. All other containers in the Pod join this shared network namespace using \`--net=container:pause\`.

This design means:
- Containers in the same Pod can communicate with each other over \`localhost\`.
- Containers in the same Pod share the same port space (they cannot bind to the same port).
- If an application container crashes and restarts, the Pod's IP address remains unchanged.

## Declarative Pod Specification

\`\`\`yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx-web
  labels:
    app: web
    tier: frontend
spec:
  containers:
  - name: nginx
    image: nginx:1.25-alpine
    ports:
    - containerPort: 80
    resources:
      requests:
        cpu: "100m"
        memory: "128Mi"
      limits:
        cpu: "250m"
        memory: "256Mi"
\`\`\`

## Common Production Gotchas

1. **Avoid Multi-Container Pods for Independent Scaling**: Only co-locate containers in the same Pod when they must share files or network interfaces (e.g. Envoy proxies, Fluentbit agents). If two containers can scale independently, put them in separate Deployments.
2. **Always Define Resource Requests & Limits**: Without resource requests, the Kubernetes scheduler cannot place pods reliably, leading to noisy neighbor problems and OOMKilled evictions.
`,
    diagramConfig: {
      type: 'pod_namespace_diagram',
      layers: ['cgroups', 'net_ns', 'mount_ns', 'user_ns'],
    },
    concepts: ['pod', 'pause-container', 'linux-namespaces', 'cgroups', 'sidecar-pattern'],
    prerequisites: ['linux-namespaces-cgroups'],
    associatedLabId: 'k8s-pod-basics',
    associatedQuizId: 'quiz-k8s-pods',
  },
  {
    id: 'k8s-deployments-rollouts',
    moduleId: 'mod-k8s-core',
    trackSlug: 'kubernetes',
    title: 'Deployments, ReplicaSets & Zero-Downtime Rollouts',
    slug: 'deployments-and-rollouts',
    order: 2,
    durationMinutes: 20,
    xp: 200,
    summary:
      'Master declarative workload updates with Deployments, RollingUpdate strategies, revision history, and instant rollbacks.',
    contentMarkdown: `
# Deployments & Rolling Updates

A **Deployment** provides declarative updates for Pods and ReplicaSets. You describe a *desired state* in a Deployment, and the Deployment Controller changes the actual state to the desired state at a controlled rate.

\`\`\`mermaid
graph LR
    Dep[Deployment: web-app v2] -->|Manages| RS2[ReplicaSet: web-app-v2 (3/3)]
    Dep -.->|Old RS (0/3)| RS1[ReplicaSet: web-app-v1]
    RS2 --> P1[Pod 1]
    RS2 --> P2[Pod 2]
    RS2 --> P3[Pod 3]
\`\`\`

## RollingUpdate Strategy Mechanics

When you update the \`spec.template\` of a Deployment, a new ReplicaSet is created. Kubernetes carefully coordinates pod creation and termination using two key parameters:

- **\`maxSurge\`**: The maximum number of Pods that can be created over the desired number of Pods.
- **\`maxUnavailable\`**: The maximum number of Pods that can be unavailable during the update process.

\`\`\`yaml
spec:
  replicas: 5
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1        # Max 6 pods running at any moment
      maxUnavailable: 0  # Always maintain at least 5 healthy pods
\`\`\`

## Rollback Command Workflow

\`\`\`bash
# Inspect rollout history
kubectl rollout history deployment/web-app

# Undo a broken rollout to previous revision
kubectl rollout undo deployment/web-app

# Undo to specific revision
kubectl rollout undo deployment/web-app --to-revision=2
\`\`\`
`,
    concepts: ['deployment', 'replicaset', 'rolling-update', 'maxsurge', 'maxunavailable', 'rollback'],
    prerequisites: ['k8s-pod-architecture'],
    associatedLabId: 'k8s-deployments-scaling',
    associatedQuizId: 'quiz-k8s-deployments',
  },
  {
    id: 'k8s-services-networking',
    moduleId: 'mod-k8s-networking',
    trackSlug: 'kubernetes',
    title: 'Services, Endpoints & kube-proxy Packet Routing',
    slug: 'services-and-networking',
    order: 3,
    durationMinutes: 20,
    xp: 200,
    summary:
      'Demystify ClusterIP, NodePort, LoadBalancer, Endpoints/EndpointSlices, and iptables/IPVS packet routing inside the Linux kernel.',
    contentMarkdown: `
# Kubernetes Services: Stable Virtual IPs

Pods in Kubernetes are ephemeral — they get assigned dynamically changing IP addresses whenever they are rescheduled. A **Service** provides a single stable IP address (ClusterIP) and DNS name that routes traffic across a dynamic pool of healthy Pods.

\`\`\`mermaid
graph TD
    Client[Client Pod] -->|Requests http://web-service:80| VIP[ClusterIP Virtual IP: 10.96.0.120]
    VIP -->|kube-proxy iptables / IPVS rule| EP1[Pod IP 10.244.1.22:8080]
    VIP -->|kube-proxy iptables / IPVS rule| EP2[Pod IP 10.244.2.14:8080]
    VIP -->|kube-proxy iptables / IPVS rule| EP3[Pod IP 10.244.2.19:8080]
\`\`\`

## Service Types Compared

| Service Type | Scope | Use Case |
|---|---|---|
| **ClusterIP** (Default) | Cluster Internal | Microservice-to-microservice communication within the cluster. |
| **NodePort** | External via Node IP:Port | Direct external access via static high port (\`30000-32767\`). |
| **LoadBalancer** | External via Cloud Provider | Provisions an AWS NLB, GCP Cloud LB, or Azure LB automatically. |
| **ExternalName** | Cluster Internal -> External CNAME | Maps a Kubernetes DNS name to an external domain (e.g. \`db.external.com\`). |
`,
    concepts: ['service', 'clusterip', 'nodeport', 'loadbalancer', 'kube-proxy', 'endpointslices'],
    prerequisites: ['k8s-deployments-rollouts'],
    associatedLabId: 'k8s-services-clusterip',
    associatedQuizId: 'quiz-k8s-services',
  },
];
