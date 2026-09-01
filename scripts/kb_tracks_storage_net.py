#!/usr/bin/env python3
"""
Tracks 3 & 4: Storage (10 Labs) & Cloud-Native Networking (12 Labs)
"""

def register_storage_and_networking(reg):
    # Track 3: Storage (10 labs)
    storage_labs = [
        ("storage-01-emptydir-scratch", "emptyDir Scratch Volumes & In-Memory Mounts", "Use emptyDir with medium: Memory for ephemeral high-performance scratch space."),
        ("storage-02-hostpath-local", "hostPath Volumes & Node-Locked Workloads", "Understand hostPath directoryOrCreate mechanics and security implications on single-node setups."),
        ("storage-03-nfs-static-pv-pvc", "Static PersistentVolume & PersistentVolumeClaim Binding", "Bind static PVs and PVCs with accessModes ReadWriteMany and ReadWriteOnce."),
        ("storage-04-storageclass-dynamic-csi", "Dynamic CSI StorageClasses & WaitForFirstConsumer", "Configure dynamic volume provisioning with volumeBindingMode: WaitForFirstConsumer and reclaimPolicy: Retain."),
        ("storage-05-volume-resizing-online", "Online Volume Expansion & Filesystem Resizing", "Expand live PersistentVolumeClaims without downtime using allowVolumeExpansion: true."),
        ("storage-06-volume-snapshots-restore", "CSI VolumeSnapshots & Point-in-Time Restoration", "Take point-in-time CSI volume snapshots and restore them into new PersistentVolumeClaims."),
        ("storage-07-statefulset-pvctemplate", "StatefulSet volumeClaimTemplates & Storage Binding", "Dynamically allocate independent persistent volumes per StatefulSet replica."),
        ("storage-08-local-persistent-volume", "Local PersistentVolumes with NodeAffinity", "Pin high-performance local NVMe SSDs using local PVs and WaitForFirstConsumer."),
        ("storage-09-csi-driver-secrets-store", "Secrets Store CSI Driver & Vault Key Injection", "Mount cloud vault secrets directly as filesystem files without storing raw secrets in etcd."),
        ("storage-10-volume-mount-subpath", "volumeMounts subPath & Single-File Config Injection", "Inject specific configuration files into existing directories using subPath without masking directory contents.")
    ]

    for lab_id, title, summary in storage_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    PVC["PersistentVolumeClaim ({lab_id})"] -->|Binds| PV["PersistentVolume (CSI Storage)"]
    PV --> Driver["CSI Driver (NodeStage & NodePublish)"]
    Driver --> Pod["Application Pod VolumeMount"]""",
            f"""# {title}

Mastering Kubernetes storage primitives ensures stateful data persistence, high I/O throughput, and zero data loss during pod rescheduling.

## Architectural Data Flow

```mermaid
graph LR
    PVC["PersistentVolumeClaim ({lab_id})"] -->|Binds| PV["PersistentVolume (CSI Storage)"]
    PV --> Driver["CSI Driver (NodeStage & NodePublish)"]
    Driver --> Pod["Application Pod VolumeMount"]
```

## Key Operational Commands

```bash
# Verify PVC binding status
kubectl get pvc,pv -o wide

# Inspect StorageClass parameters
kubectl describe storageclass standard
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: {lab_id}-pvc
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

## Common Production Gotchas & Anti-Patterns

1. **Immediate Binding on Multi-Zone Clusters**: Using `volumeBindingMode: Immediate` causes volumes to be created in zones with no available compute nodes.
2. **AccessMode Mismatches**: Requesting `ReadWriteMany` on block storage providers that only support `ReadWriteOnce`.
3. **Overwriting Directories with Mounts**: Mounting a volume to `/etc` without `subPath`, masking all existing system files.

## Security & Reliability Best Practices

- **Set WaitForFirstConsumer**: Always configure `volumeBindingMode: WaitForFirstConsumer` on multi-zone clusters.
- **Enable allowVolumeExpansion**: Set `allowVolumeExpansion: true` in StorageClasses to permit online storage resizing.""",
            ["Using Immediate binding mode in multi-zone clusters leading to unschedulable pods.",
             "Requesting ReadWriteMany on cloud block storage that only supports ReadWriteOnce.",
             "Mounting whole volumes over non-empty directories without subPath."],
            "Always specify `volumeBindingMode: WaitForFirstConsumer` in production StorageClasses.")

    # Track 4: Networking (12 labs)
    net_labs = [
        ("net-00-clusterip-basics", "ClusterIP Service & kube-proxy Packet Routing", "Expose internal workloads using ClusterIP services, EndpointSlices, and iptables/IPVS proxying."),
        ("net-01-nodeport-external", "NodePort Services & externalTrafficPolicy Routing", "Route external traffic into cluster nodes using NodePort and compare Cluster vs Local traffic policies."),
        ("net-02-loadbalancer-cloud", "LoadBalancer Services & Cloud Provider Integration", "Provision cloud load balancers with health checks and external-ip allocations."),
        ("net-03-ingress-controller-routing", "Ingress Controllers & Path-Based HTTP Routing", "Route HTTP/HTTPS traffic with Ingress resources, TLS termination, and path prefix rules."),
        ("net-04-networkpolicy-default-deny", "Zero-Trust Default-Deny NetworkPolicies", "Enforce zero-trust network isolation with default-deny ingress and egress NetworkPolicies."),
        ("net-05-networkpolicy-ingress-egress", "Fine-Grained Ingress & Egress NetworkPolicies", "Allow specific ingress from frontend pods and egress to database CIDRs on dedicated ports."),
        ("net-06-coredns-custom-records", "CoreDNS Custom Records & Upstream Forwarding", "Customize cluster DNS with CoreDNS Corefile rewrite rules and upstream forwarders."),
        ("net-07-metallb-baremetal-bgp", "MetalLB LoadBalancer for Bare-Metal Clusters", "Deploy MetalLB with IPAddressPools and L2/BGP advertisements for on-prem LoadBalancers."),
        ("net-08-cilium-ebpf-dataplane", "Cilium eBPF Dataplane & Kernel Packet Filtering", "Accelerate network throughput and enforce Layer 7 policies using Cilium eBPF dataplane."),
        ("net-09-calico-bgp-peering", "Calico CNI IPPools & BGP Route Distribution", "Configure Calico IP pools, VXLAN encapsulation, and BGP peering with Top-of-Rack switches."),
        ("net-10-gateway-api-httproute", "Kubernetes Gateway API & HTTPRoute Splitting", "Adopt the next-generation Gateway API with GatewayClasses, Gateways, and HTTPRoutes."),
        ("net-11-dual-stack-ipv4-ipv6", "IPv4/IPv6 Dual-Stack Service & Pod Networking", "Configure dual-stack pod networking and multi-family ClusterIP services.")
    ]

    for lab_id, title, summary in net_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    Client["Client Traffic"] --> Service["Kubernetes Service / Ingress ({lab_id})"]
    Service --> Proxy["kube-proxy / CNI eBPF DataPlane"]
    Proxy --> Endpoints["Pod Endpoints (Backend Replicas)"]""",
            f"""# {title}

Kubernetes networking provides service discovery, load balancing, and network security policies across all containerized workloads.

## Architectural Traffic Routing

```mermaid
graph LR
    Client["Client Traffic"] --> Service["Kubernetes Service / Ingress ({lab_id})"]
    Service --> Proxy["kube-proxy / CNI eBPF DataPlane"]
    Proxy --> Endpoints["Pod Endpoints (Backend Replicas)"]
```

## Key Operational Commands

```bash
# Check service endpoints and IP allocations
kubectl get svc,endpoints,endpointslices -o wide

# Test internal DNS resolution
kubectl run dns-test --rm -it --image=busybox:1.36 -- nslookup kubernetes.default
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Service
metadata:
  name: {lab_id}
  labels:
    app: {lab_id}
spec:
  type: ClusterIP
  selector:
    app: {lab_id}
  ports:
  - port: 80
    targetPort: 8080
```

## Common Production Gotchas & Anti-Patterns

1. **Mismatched TargetPorts**: Setting `targetPort` to a different port than the container's listening port leads to connection refused.
2. **Missing Egress to CoreDNS**: Applying an egress NetworkPolicy without allowing UDP/TCP port 53 to kube-system breaks all DNS resolution.
3. **Empty Endpoints due to Selector Typos**: Mismatch between Service `spec.selector` and Pod `metadata.labels`.

## Security & Reliability Best Practices

- **Enforce Default-Deny Policies**: Apply default-deny NetworkPolicies per namespace and explicitly allow required traffic flows.
- **Adopt EndpointSlices**: Leverage EndpointSlices for scalable service routing in large clusters with >1000 pods.""",
            ["Applying strict egress NetworkPolicies without allowing port 53 to CoreDNS, breaking DNS.",
             "Mismatched selector labels leading to 0 endpoints on the Service.",
             "Confusing service 'port' (external service port) with 'targetPort' (container port)."],
            "Always test DNS resolution and verify endpoints using `kubectl get endpoints` when debugging network services.")
