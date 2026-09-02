#!/usr/bin/env python3
"""
Authoritative Curriculum Knowledge Base Definition for KubeLab
Contains structured data, architecture diagrams, scenarios, tasks, validation rules,
common mistakes, production guidance, and question banks for all 154 labs across 15 tracks.
"""

TRACK_META = [
    {
        "id": "track-linux-containers",
        "slug": "linux-containers",
        "title": "Linux & Container Fundamentals",
        "description": "Linux systems engineering, kernel namespaces, cgroups v2, chroot, rootless OCI runtimes, multi-stage Containerfiles, and systemd integration.",
        "icon": "Terminal",
        "color": "#06B6D4",
        "difficulty": "beginner",
        "order": 1,
        "modules": [
            {
                "id": "mod-linux-containers-core",
                "slug": "linux-containers-architecture-workloads",
                "title": "Linux Kernel Primitives & Container Runtimes",
                "description": "Explore POSIX permissions, process signals, namespaces, and cgroups v2 resource control.",
                "lessons": [
                    {"lab_id": "linux-01-fs-permissions"},
                    {"lab_id": "linux-02-process-signals"},
                    {"lab_id": "linux-03-namespaces-isolation"},
                    {"lab_id": "linux-04-cgroupsv2-limits"},
                ]
            },
            {
                "id": "mod-linux-containers-advanced",
                "slug": "linux-containers-advanced",
                "title": "Rootless Containers, Containerfiles & OCI Runtime",
                "description": "Master rootless isolation, multi-stage builds, systemd quadlets, and runc OCI specifications.",
                "lessons": [
                    {"lab_id": "linux-05-rootless-containers"},
                    {"lab_id": "linux-06-multistage-containerfile"},
                    {"lab_id": "linux-07-systemd-service-quadlet"},
                    {"lab_id": "linux-08-oci-spec-runc"},
                ]
            }
        ]
    },
    {
        "id": "track-kubernetes",
        "slug": "kubernetes",
        "title": "Kubernetes Core Architecture & Workloads",
        "description": "Master Pods, Deployments, Services, ConfigMaps, Secrets, Storage, Probes, and Declarative manifests in real Kubernetes clusters.",
        "icon": "Boxes",
        "color": "#6366F1",
        "difficulty": "beginner",
        "order": 2,
        "modules": [
            {
                "id": "mod-kubernetes-primitives",
                "slug": "kubernetes-primitives",
                "title": "Pods, InitContainers, Sidecars & Config Projection",
                "description": "Core atomic workload primitives, multi-container communication, and decoupled configuration.",
                "lessons": [
                    {"lab_id": "k8s-01-pod-lifecycle-phases"},
                    {"lab_id": "k8s-02-init-containers-dependency"},
                    {"lab_id": "k8s-03-sidecar-containers-k8s128"},
                    {"lab_id": "k8s-04-multi-container-shared-volume"},
                    {"lab_id": "k8s-05-configmap-env-volume"},
                    {"lab_id": "k8s-06-secrets-base64-tls"},
                    {"lab_id": "k8s-07-resource-requests-limits"},
                    {"lab_id": "k8s-08-liveness-readiness-startup-probes"},
                ]
            },
            {
                "id": "mod-kubernetes-controllers",
                "slug": "kubernetes-controllers",
                "title": "Controllers, Scaling, Batch Jobs & Scheduling",
                "description": "Declarative controller reconciliation, rolling deployments, DaemonSets, StatefulSets, HPA, and node affinity.",
                "lessons": [
                    {"lab_id": "k8s-09-deployments-rolling-update"},
                    {"lab_id": "k8s-10-replicaset-reconciliation"},
                    {"lab_id": "k8s-11-daemonset-node-affinity"},
                    {"lab_id": "k8s-12-statefulset-stable-network"},
                    {"lab_id": "k8s-13-jobs-cronjobs-completions"},
                    {"lab_id": "k8s-14-hpa-cpu-scaling"},
                    {"lab_id": "k8s-15-pod-disruption-budget"},
                    {"lab_id": "k8s-16-node-taints-tolerations"},
                ]
            }
        ]
    },
    {
        "id": "track-storage",
        "slug": "storage",
        "title": "Storage & Persistent Volumes",
        "description": "StorageClasses, PersistentVolumeClaims, dynamic CSI volume provisioning, online expansion, volume snapshots, and stateful workloads.",
        "icon": "Database",
        "color": "#3B82F6",
        "difficulty": "intermediate",
        "order": 3,
        "modules": [
            {
                "id": "mod-storage-volumes",
                "slug": "storage-volumes",
                "title": "Volumes, Static PVs & Dynamic CSI Provisioning",
                "description": "Ephemeral and persistent storage options, CSI driver architecture, and access modes.",
                "lessons": [
                    {"lab_id": "storage-01-emptydir-scratch"},
                    {"lab_id": "storage-02-hostpath-local"},
                    {"lab_id": "storage-03-nfs-static-pv-pvc"},
                    {"lab_id": "storage-04-storageclass-dynamic-csi"},
                    {"lab_id": "storage-05-volume-resizing-online"},
                ]
            },
            {
                "id": "mod-storage-advanced",
                "slug": "storage-advanced",
                "title": "Volume Snapshots, Stateful Templates & Subpaths",
                "description": "Volume snapshot lifecycle, stateful claim templates, local disk pinning, and secret store CSI driver.",
                "lessons": [
                    {"lab_id": "storage-06-volume-snapshots-restore"},
                    {"lab_id": "storage-07-statefulset-pvctemplate"},
                    {"lab_id": "storage-08-local-persistent-volume"},
                    {"lab_id": "storage-09-csi-driver-secrets-store"},
                    {"lab_id": "storage-10-volume-mount-subpath"},
                ]
            }
        ]
    },
    {
        "id": "track-networking",
        "slug": "networking",
        "title": "Cloud-Native Networking, CNI & Gateway API",
        "description": "CNI plugins (Calico/Cilium), CoreDNS resolution, Ingress Controllers, eBPF data planes, NetworkPolicies, and Kubernetes Gateway API.",
        "icon": "Network",
        "color": "#10B981",
        "difficulty": "intermediate",
        "order": 4,
        "modules": [
            {
                "id": "mod-networking-services",
                "slug": "networking-services",
                "title": "Service Types, Ingress & Zero-Trust NetworkPolicies",
                "description": "ClusterIP, NodePort, LoadBalancer routing, Ingress controllers, and default-deny firewalling.",
                "lessons": [
                    {"lab_id": "net-00-clusterip-basics"},
                    {"lab_id": "net-01-nodeport-external"},
                    {"lab_id": "net-02-loadbalancer-cloud"},
                    {"lab_id": "net-03-ingress-controller-routing"},
                    {"lab_id": "net-04-networkpolicy-default-deny"},
                    {"lab_id": "net-05-networkpolicy-ingress-egress"},
                ]
            },
            {
                "id": "mod-networking-advanced",
                "slug": "networking-advanced",
                "title": "CoreDNS, Bare-Metal LoadBalancing, eBPF & Gateway API",
                "description": "CoreDNS customization, MetalLB BGP, Cilium eBPF dataplane, and modern Gateway API HTTPRoutes.",
                "lessons": [
                    {"lab_id": "net-06-coredns-custom-records"},
                    {"lab_id": "net-07-metallb-baremetal-bgp"},
                    {"lab_id": "net-08-cilium-ebpf-dataplane"},
                    {"lab_id": "net-09-calico-bgp-peering"},
                    {"lab_id": "net-10-gateway-api-httproute"},
                    {"lab_id": "net-11-dual-stack-ipv4-ipv6"},
                ]
            }
        ]
    },
    {
        "id": "track-helm-kustomize",
        "slug": "helm-kustomize",
        "title": "Packaging with Helm & Kustomize",
        "description": "Author production Helm charts, manage chart dependencies, leverage Go templates, and apply dry declarative Kustomize overlays.",
        "icon": "Package",
        "color": "#8B5CF6",
        "difficulty": "intermediate",
        "order": 5,
        "modules": [
            {
                "id": "mod-helm-templating",
                "slug": "helm-templating",
                "title": "Helm Architecture, Go Templates, Helpers & Hooks",
                "description": "Scaffold production charts, override values, write reusable named helpers, and execute lifecycle hooks.",
                "lessons": [
                    {"lab_id": "helm-01-chart-structure-scaffold"},
                    {"lab_id": "helm-02-values-override-templating"},
                    {"lab_id": "helm-03-named-templates-helpers"},
                    {"lab_id": "helm-04-chart-dependencies-subcharts"},
                    {"lab_id": "helm-05-hooks-lifecycle-post-install"},
                ]
            },
            {
                "id": "mod-kustomize-overlays",
                "slug": "kustomize-overlays",
                "title": "Kustomize Bases, Overlays, Generators & Patches",
                "description": "Dry declarative overlays, configMapGenerators, strategic merge patches, and post-renderer pipelines.",
                "lessons": [
                    {"lab_id": "helm-06-kustomize-base-overlays"},
                    {"lab_id": "helm-07-kustomize-configmap-secret-generator"},
                    {"lab_id": "helm-08-kustomize-json6902-patches"},
                    {"lab_id": "helm-09-kustomize-components-composition"},
                    {"lab_id": "helm-10-helm-kustomize-post-renderer"},
                ]
            }
        ]
    },
    {
        "id": "track-administration",
        "slug": "administration",
        "title": "Cluster Operations & Administration",
        "description": "Bootstrap clusters with kubeadm, handle control-plane high availability, etcd snapshots and disaster recovery, node drain, and PKI rotation.",
        "icon": "Server",
        "color": "#EC4899",
        "difficulty": "advanced",
        "order": 6,
        "modules": [
            {
                "id": "mod-admin-bootstrap",
                "slug": "admin-bootstrap",
                "title": "Kubeadm Bootstrap, Etcd Backup, Upgrades & Drain",
                "description": "Cluster lifecycle operations, etcd snapshot restoration, zero-downtime upgrades, and node cordon/drain.",
                "lessons": [
                    {"lab_id": "admin-01-kubeadm-bootstrap"},
                    {"lab_id": "admin-02-etcd-backup-restore"},
                    {"lab_id": "admin-03-control-plane-upgrade"},
                    {"lab_id": "admin-04-node-drain-cordon"},
                    {"lab_id": "admin-05-certificate-rotation"},
                ]
            },
            {
                "id": "mod-admin-internals",
                "slug": "admin-internals",
                "title": "Static Pods, Audit Logging, Schedulers & Webhooks",
                "description": "Debug static control plane pods, configure audit policy logging, write custom schedulers, and admission webhooks.",
                "lessons": [
                    {"lab_id": "admin-06-static-pods-debug"},
                    {"lab_id": "admin-07-audit-logging-policy"},
                    {"lab_id": "admin-08-custom-scheduler"},
                    {"lab_id": "admin-09-api-priority-fairness"},
                    {"lab_id": "admin-10-mutating-webhook"},
                ]
            }
        ]
    },
    {
        "id": "track-security",
        "slug": "security",
        "title": "Zero-Trust Kubernetes Security & RBAC",
        "description": "Implement RBAC, Pod Security Standards (Restricted), NetworkPolicies, Seccomp profiles, image vulnerability scanning, and CIS benchmarks.",
        "icon": "ShieldCheck",
        "color": "#F43F5E",
        "difficulty": "intermediate",
        "order": 7,
        "modules": [
            {
                "id": "mod-security-rbac-pss",
                "slug": "security-rbac-pss",
                "title": "RBAC Roles, Tokens, Pod Security Standards & Hardening",
                "description": "Granular authorization rules, TokenRequest projection, Pod Security Standards, and non-root securityContexts.",
                "lessons": [
                    {"lab_id": "sec-01-rbac-role-binding"},
                    {"lab_id": "sec-02-clusterrole-clusterbinding"},
                    {"lab_id": "sec-03-serviceaccount-token-projected"},
                    {"lab_id": "sec-04-pod-security-admission"},
                    {"lab_id": "sec-05-security-context-non-root"},
                    {"lab_id": "sec-06-seccomp-apparmor-profiles"},
                ]
            },
            {
                "id": "mod-security-policy-runtime",
                "slug": "security-policy-runtime",
                "title": "OPA Gatekeeper, Kyverno, Falco, Cosign & Sealed Secrets",
                "description": "Policy-as-code admission controllers, eBPF runtime threat detection, image cryptographic signing, and GitOps secret encryption.",
                "lessons": [
                    {"lab_id": "sec-07-image-vulnerability-trivy"},
                    {"lab_id": "sec-08-opa-gatekeeper-constraint"},
                    {"lab_id": "sec-09-kyverno-policy-validation"},
                    {"lab_id": "sec-10-falco-runtime-security"},
                    {"lab_id": "sec-11-kube-bench-cis-hardening"},
                    {"lab_id": "sec-12-cosign-image-signing"},
                    {"lab_id": "sec-13-sealed-secrets-gitops"},
                ]
            }
        ]
    },
    {
        "id": "track-gitops",
        "slug": "gitops",
        "title": "GitOps & Continuous Delivery with Argo CD",
        "description": "Implement declarative GitOps workflows, App-of-Apps pattern, automated sync policies, drift detection, and automated rollbacks.",
        "icon": "GitBranch",
        "color": "#A855F7",
        "difficulty": "intermediate",
        "order": 8,
        "modules": [
            {
                "id": "mod-gitops-argocd",
                "slug": "gitops-argocd",
                "title": "Argo CD Bootstrap, App-of-Apps, Sync Policies & Waves",
                "description": "Install Argo CD, structure enterprise multi-app hierarchies, configure automated self-healing, and wave-ordered migrations.",
                "lessons": [
                    {"lab_id": "gitops-01-argocd-bootstrap"},
                    {"lab_id": "gitops-02-app-of-apps-pattern"},
                    {"lab_id": "gitops-03-automated-sync-prune"},
                    {"lab_id": "gitops-04-sync-waves-hooks"},
                    {"lab_id": "gitops-05-drift-detection-remediation"},
                ]
            },
            {
                "id": "mod-gitops-progressive",
                "slug": "gitops-progressive",
                "title": "Argo Rollouts, Canary Analysis, Blue-Green & Flux CD",
                "description": "Progressive delivery with Argo Rollouts, Prometheus metric analysis steps, Blue-Green routing, and Flux CD controllers.",
                "lessons": [
                    {"lab_id": "gitops-06-argo-rollouts-canary"},
                    {"lab_id": "gitops-07-analysis-template-prometheus"},
                    {"lab_id": "gitops-08-blue-green-active-preview"},
                    {"lab_id": "gitops-09-flux-kustomization-controller"},
                    {"lab_id": "gitops-10-multi-cluster-argocd"},
                ]
            }
        ]
    },
    {
        "id": "track-service-mesh",
        "slug": "service-mesh",
        "title": "Service Mesh with Istio & Envoy Proxy",
        "description": "Traffic shifting, canary releases, mutual TLS (mTLS), fault injection, circuit breaking, rate limiting, and Envoy sidecar telemetry.",
        "icon": "Layers",
        "color": "#0EA5E9",
        "difficulty": "advanced",
        "order": 9,
        "modules": [
            {
                "id": "mod-mesh-istio-core",
                "slug": "mesh-istio-core",
                "title": "Istio Operator, Sidecar Injection, VirtualService & mTLS",
                "description": "Deploy istiod, automatic Envoy sidecar injection, VirtualService weight routing, and STRICT mutual TLS.",
                "lessons": [
                    {"lab_id": "mesh-01-istio-control-plane"},
                    {"lab_id": "mesh-02-sidecar-injection-envoy"},
                    {"lab_id": "mesh-03-virtualservice-canary-routing"},
                    {"lab_id": "mesh-04-destinationrule-traffic-policy"},
                    {"lab_id": "mesh-05-strict-mtls-peerauthentication"},
                    {"lab_id": "mesh-06-authorizationpolicy-rbac"},
                ]
            },
            {
                "id": "mod-mesh-traffic-resilience",
                "slug": "mesh-traffic-resilience",
                "title": "Fault Injection, Circuit Breaking, Gateways & Wasm/Lua",
                "description": "Inject chaos latency/aborts, configure outlier detection, manage ingress/egress gateways, and write custom EnvoyFilters.",
                "lessons": [
                    {"lab_id": "mesh-07-fault-injection-latency"},
                    {"lab_id": "mesh-08-circuit-breaking-outlier-detection"},
                    {"lab_id": "mesh-09-ingress-gateway-tls-termination"},
                    {"lab_id": "mesh-10-egress-gateway-external-access"},
                    {"lab_id": "mesh-11-envoyfilter-lua-scripting"},
                ]
            }
        ]
    },
    {
        "id": "track-observability",
        "slug": "observability",
        "title": "OpenTelemetry, Prometheus & Grafana",
        "description": "End-to-end distributed tracing with OpenTelemetry, metric collection with Prometheus, PromQL alerting, Loki log analysis, and Grafana.",
        "icon": "Activity",
        "color": "#14B8A6",
        "difficulty": "advanced",
        "order": 10,
        "modules": [
            {
                "id": "mod-obs-prometheus",
                "slug": "obs-prometheus",
                "title": "Prometheus Operator, ServiceMonitors, PromQL & Alerts",
                "description": "Deploy Prometheus CRDs, target scrape configurations, write rate/quantile PromQL queries, and route Alertmanager notifications.",
                "lessons": [
                    {"lab_id": "obs-01-prometheus-operator-crds"},
                    {"lab_id": "obs-02-servicemonitor-metric-scrape"},
                    {"lab_id": "obs-03-promql-error-rate-alerts"},
                    {"lab_id": "obs-04-alertmanager-routing-receivers"},
                    {"lab_id": "obs-05-grafana-dashboards-as-code"},
                ]
            },
            {
                "id": "mod-obs-tracing-logs",
                "slug": "obs-tracing-logs",
                "title": "OpenTelemetry Collector, Tempo Tracing, Loki Logs & Probes",
                "description": "Build OTel data pipelines, trace propagation across microservices with Tempo, Loki log streams, and Blackbox probes.",
                "lessons": [
                    {"lab_id": "obs-06-opentelemetry-collector-pipeline"},
                    {"lab_id": "obs-07-distributed-tracing-tempo"},
                    {"lab_id": "obs-08-loki-promtail-log-pipeline"},
                    {"lab_id": "obs-09-kube-state-metrics-deployment"},
                    {"lab_id": "obs-10-blackbox-exporter-probes"},
                ]
            }
        ]
    },
    {
        "id": "track-troubleshooting",
        "slug": "troubleshooting",
        "title": "Production Troubleshooting & Break-Fix",
        "description": "Diagnose CrashLoopBackOff, ImagePullBackOff, Pending unschedulable pods, OOMKills, DNS outages, missing endpoints, and node failures.",
        "icon": "Wrench",
        "color": "#F59E0B",
        "difficulty": "advanced",
        "order": 11,
        "modules": [
            {
                "id": "mod-trouble-pod-failures",
                "slug": "trouble-pod-failures",
                "title": "CrashLoopBackOff, ImagePullBackOff, CPU Starvation & OOM",
                "description": "Live root-cause diagnosis of failed container runtimes, registry authentication rejections, and memory leaks.",
                "lessons": [
                    {"lab_id": "trouble-01-crashloopbackoff-diagnosis"},
                    {"lab_id": "trouble-02-imagepullbackoff-auth"},
                    {"lab_id": "trouble-03-pending-insufficient-cpu"},
                    {"lab_id": "trouble-04-oomkilled-exit-137"},
                    {"lab_id": "trouble-05-missing-service-endpoints"},
                ]
            },
            {
                "id": "mod-trouble-cluster-outages",
                "slug": "trouble-cluster-outages",
                "title": "CoreDNS Outages, Dead Nodes, Storage Deadlocks & 503s",
                "description": "Troubleshoot cluster DNS resolution failures, dead kubelet daemons, multi-attach PVC locks, and Ingress 503 cascades.",
                "lessons": [
                    {"lab_id": "trouble-06-dns-nxdomain-resolution"},
                    {"lab_id": "trouble-07-node-notready-kubelet"},
                    {"lab_id": "trouble-08-pvc-mount-deadlock"},
                    {"lab_id": "trouble-09-networkpolicy-traffic-blocked"},
                    {"lab_id": "trouble-10-ingress-503-backend-dead"},
                ]
            }
        ]
    },
    {
        "id": "track-sre-performance",
        "slug": "sre-performance",
        "title": "Site Reliability Engineering & SLOs",
        "description": "Define meaningful SLIs/SLOs, calculate error budget burn rates, alerting thresholds, HPA/VPA autoscaling, and capacity planning.",
        "icon": "Gauge",
        "color": "#E11D48",
        "difficulty": "advanced",
        "order": 12,
        "modules": [
            {
                "id": "mod-sre-slos-scaling",
                "slug": "sre-slos-scaling",
                "title": "SLI/SLO Math, Multi-Window Burn Rates, VPA & KEDA",
                "description": "Calculate error budget exhaustion rates, configure multi-burn rate alerts, and drive event-driven autoscaling with KEDA.",
                "lessons": [
                    {"lab_id": "sre-01-sli-slo-definition"},
                    {"lab_id": "sre-02-multi-window-burn-rate-alert"},
                    {"lab_id": "sre-03-vpa-resource-recommendations"},
                    {"lab_id": "sre-04-keda-event-driven-autoscaling"},
                    {"lab_id": "sre-05-topology-spread-constraints"},
                ]
            },
            {
                "id": "mod-sre-resilience-chaos",
                "slug": "sre-resilience-chaos",
                "title": "Pod Priority, Problem Detectors, Chaos Mesh & Evictions",
                "description": "PriorityClass preemption, automated node problem detection, Chaos Mesh failure experiments, and kubelet eviction limits.",
                "lessons": [
                    {"lab_id": "sre-06-pod-priority-preemption"},
                    {"lab_id": "sre-07-node-problem-detector"},
                    {"lab_id": "sre-08-chaos-mesh-pod-kill"},
                    {"lab_id": "sre-09-concurrency-limits-overload"},
                    {"lab_id": "sre-10-kubelet-eviction-thresholds"},
                ]
            }
        ]
    },
    {
        "id": "track-platform-eng",
        "slug": "platform-eng",
        "title": "Platform Engineering & Multi-Cluster",
        "description": "Build Internal Developer Platforms (IDPs), write Kubernetes Operators and CRDs in Go/Rust, and manage multi-cluster fleets with Cluster API.",
        "icon": "Cpu",
        "color": "#6366F1",
        "difficulty": "expert",
        "order": 13,
        "modules": [
            {
                "id": "mod-platform-crds-crossplane",
                "slug": "platform-crds-crossplane",
                "title": "Custom Resources, Crossplane XRDs, Cluster API & vCluster",
                "description": "Author OpenAPI v3 CRDs, compose cloud resources with Crossplane, provision clusters with CAPI, and spin up vClusters.",
                "lessons": [
                    {"lab_id": "platform-01-custom-resource-definition"},
                    {"lab_id": "platform-02-crossplane-xrd-composition"},
                    {"lab_id": "platform-03-cluster-api-aws-provision"},
                    {"lab_id": "platform-04-vcluster-virtual-tenancy"},
                    {"lab_id": "platform-05-backstage-software-template"},
                ]
            },
            {
                "id": "mod-platform-promotion-karpenter",
                "slug": "platform-promotion-karpenter",
                "title": "Kargo GitOps Promotion, External Secrets & Karpenter",
                "description": "Automate multi-stage artifact promotion, synchronize external cloud secret vaults, and bin-pack nodes with Karpenter.",
                "lessons": [
                    {"lab_id": "platform-06-kargo-stage-promotion"},
                    {"lab_id": "platform-07-external-secrets-operator"},
                    {"lab_id": "platform-08-karpenter-node-autoscaling"},
                    {"lab_id": "platform-09-telepresence-local-intercept"},
                    {"lab_id": "platform-10-devspace-rapid-inner-loop"},
                ]
            }
        ]
    },
    {
        "id": "track-incidents",
        "slug": "incidents",
        "title": "Production Incident Response & Chaos",
        "description": "Real-world SEV-1 break-fix simulations: live CoreDNS failures, network partitions, PVC deadlocks, expired TLS certs, and GitOps sync jams.",
        "icon": "AlertTriangle",
        "color": "#EF4444",
        "difficulty": "expert",
        "order": 14,
        "modules": [
            {
                "id": "mod-incidents-network-controlplane",
                "slug": "incidents-network-controlplane",
                "title": "SEV-1 CoreDNS Crashes, HPA Thrashing, 503 Outages & Storage Locks",
                "description": "Live production incident response drills with real Kubernetes failure modes and deterministic health validation.",
                "lessons": [
                    {"lab_id": "incident-coredns-failure"},
                    {"lab_id": "incident-hpa-thrashing"},
                    {"lab_id": "incident-ingress-503-outage"},
                    {"lab_id": "incident-pvc-deadlock-multiattach"},
                    {"lab_id": "incident-etcd-quorum-loss"},
                ]
            },
            {
                "id": "mod-incidents-security-gitops",
                "slug": "incidents-security-gitops",
                "title": "SEV-1 Certificate Expirations, OOM Cascades, GitOps Jams & IPAM Exhaustion",
                "description": "Live incident recovery from expired control plane PKI, node memory exhaustion, deadlocked GitOps syncs, and CNI exhaustion.",
                "lessons": [
                    {"lab_id": "incident-cert-expiration-outage"},
                    {"lab_id": "incident-oom-cascade-starvation"},
                    {"lab_id": "incident-gitops-sync-jam"},
                    {"lab_id": "incident-istio-mtls-breakage"},
                    {"lab_id": "incident-cni-ip-exhaustion"},
                ]
            }
        ]
    },
    {
        "id": "track-certification",
        "slug": "certification",
        "title": "Real-World Exam & Certification Drills",
        "description": "Timed multi-objective scenario drills covering CKA, CKAD, and CKS curriculum competencies under strict deterministic state evaluation.",
        "icon": "Award",
        "color": "#FBBF24",
        "difficulty": "expert",
        "order": 15,
        "modules": [
            {
                "id": "mod-cert-cka-ckad",
                "slug": "cert-cka-ckad",
                "title": "CKA Administration & CKAD Workload Engineering Speed Drills",
                "description": "High-intensity timed drills for Certified Kubernetes Administrator and Application Developer exams.",
                "lessons": [
                    {"lab_id": "cert-01-cka-speed-drill"},
                    {"lab_id": "cert-02-ckad-speed-drill"},
                ]
            },
            {
                "id": "mod-cert-cks-kcna",
                "slug": "cert-cks-kcna",
                "title": "CKS Security Specialist & KCNA Comprehensive Speed Drills",
                "description": "High-intensity timed drills for Certified Kubernetes Security Specialist and Cloud Native Associate exams.",
                "lessons": [
                    {"lab_id": "cert-03-cks-speed-drill"},
                    {"lab_id": "cert-04-kcna-comprehensive-drill"},
                ]
            }
        ]
    }
]

from kb_data_builder import build_comprehensive_kb, generate_dynamic_questions, generate_dynamic_lab

KNOWLEDGE_BASE = build_comprehensive_kb()

def generate_question_bank(lab_id, kb, track_slug):
    return generate_dynamic_questions(lab_id, kb, track_slug)

def generate_lab_definition(lab_id, kb, track_slug):
    return generate_dynamic_lab(lab_id, kb, track_slug)
