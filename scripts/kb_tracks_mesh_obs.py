#!/usr/bin/env python3
"""
Tracks 9 & 10: Service Mesh with Istio (11 Labs) & Observability (10 Labs)
"""

def register_mesh_and_obs(reg):
    # Track 9: Service Mesh (11 labs)
    mesh_labs = [
        ("mesh-01-istio-control-plane", "Deploying Istio Service Mesh Operator", "Install istiod control plane and configure global service mesh profiles."),
        ("mesh-02-sidecar-injection-envoy", "Envoy Sidecar Proxy Automatic Injection", "Enable namespace sidecar injection, inspect iptables packet interception, and configure Envoy sidecars."),
        ("mesh-03-virtualservice-canary-routing", "VirtualService Dynamic Traffic Shifting & Canary", "Route traffic dynamically between v1 and v2 service subsets based on percentages and HTTP headers."),
        ("mesh-04-destinationrule-traffic-policy", "DestinationRule Load Balancing & Connection Pools", "Configure connection pool limits, HTTP keep-alive timeouts, and load balancing algorithms."),
        ("mesh-05-strict-mtls-peerauthentication", "Enforcing STRICT Mutual TLS Across Service Mesh", "Enforce zero-trust encrypted mTLS and SPIFFE identity validation using PeerAuthentication."),
        ("mesh-06-authorizationpolicy-rbac", "Fine-Grained Service Mesh Authorization Policies", "Enforce Layer 7 RBAC rules allowing specific JWT principals, HTTP methods, and paths."),
        ("mesh-07-fault-injection-latency", "Chaos Engineering & Fault Injection in Istio", "Inject artificial latency delays and HTTP 500 aborts into VirtualService routes to test resilience."),
        ("mesh-08-circuit-breaking-outlier-detection", "Circuit Breaking & Outlier Detection Ejection", "Automatically eject unhealthy upstream host instances when consecutive 5xx errors occur."),
        ("mesh-09-ingress-gateway-tls-termination", "Istio Ingress Gateway & TLS Termination", "Expose mesh services to external clients through Istio Gateway with SNI and TLS certificates."),
        ("mesh-10-egress-gateway-external-access", "Egress Gateway & Controlled External API Access", "Route outbound traffic to external SaaS APIs through dedicated, monitored Egress Gateways."),
        ("mesh-11-envoyfilter-lua-scripting", "EnvoyFilter Custom Lua Scripting & Wasm Plugins", "Inject custom Lua scripts and WebAssembly filters into Envoy proxy request pipelines.")
    ]

    for lab_id, title, summary in mesh_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    ClientPod["Client Pod (Envoy: {lab_id})"] -->|mTLS| ServerPod["Server Pod ({title})"]
    istiod["istiod Control Plane"] -->|xDS Push| ClientPod
    istiod -->|xDS Push| ServerPod""",
            f"""# {title}

Service mesh architecture provides transparent security (mTLS), advanced traffic steering, fault resilience, and deep Layer 7 observability.

## Architectural Data Plane

```mermaid
graph LR
    ClientPod["Client Pod (Envoy: {lab_id})"] -->|mTLS| ServerPod["Server Pod ({title})"]
    istiod["istiod Control Plane"] -->|xDS Push| ClientPod
    istiod -->|xDS Push| ServerPod
```

## Key Operational Commands

```bash
# Check proxy synchronization status
istioctl proxy-status

# Inspect active cluster and route configurations in Envoy
istioctl proxy-config routes pod/my-pod
```

## Common Production Gotchas & Anti-Patterns

1. **Enabling STRICT mTLS Without DestinationRule**: Setting `PeerAuthentication` to `STRICT` without updating `DestinationRule` trafficPolicy to `ISTIO_MUTUAL` breaks client connections.
2. **Missing Ingress Gateway Secrets**: Deploying a Gateway without creating the required TLS secret in the `istio-system` namespace.
3. **Unbounded Envoy Memory**: Failing to configure connection limits on high-concurrency microservices.

## Security & Reliability Best Practices

- **Enforce STRICT mTLS Mesh-Wide**: Ensure all inter-service communications are encrypted and authenticated via SPIFFE identities.
- **Configure Outlier Detection**: Automatically eject degraded instances before user-facing error cascades occur.""",
            ["Configuring PeerAuthentication STRICT without matching DestinationRule ISTIO_MUTUAL.",
             "Missing istio-injection=enabled label on application namespaces.",
             "Applying VirtualServices without declaring corresponding DestinationRule subsets."],
            "Always verify proxy synchronization with `istioctl proxy-status` before executing canary shifts.")

    # Track 10: Observability (10 labs)
    obs_labs = [
        ("obs-01-prometheus-operator-crds", "Deploying Prometheus Operator & Core CRDs", "Deploy Prometheus, Alertmanager, and ServiceMonitor custom resource definitions."),
        ("obs-02-servicemonitor-metric-scrape", "Configuring Declarative ServiceMonitor Metrics", "Scrape application /metrics endpoints dynamically using Kubernetes ServiceMonitors."),
        ("obs-03-promql-error-rate-alerts", "Writing PromQL Alerts for Latency & Error Rates", "Author PrometheusRule alert expressions measuring HTTP 5xx rates and p99 latency percentiles."),
        ("obs-04-alertmanager-routing-receivers", "Alertmanager Notification Routing & Inhabitation", "Route alerts to Slack, PagerDuty, and email with grouping and silence inhibitions."),
        ("obs-05-grafana-dashboards-as-code", "Provisioning Grafana Dashboards as Code", "Import and manage Grafana dashboard JSON models via Kubernetes ConfigMaps and sidecars."),
        ("obs-06-opentelemetry-collector-pipeline", "Building OpenTelemetry Collector Pipelines", "Configure OTel receivers, batch processors, and OTLP exporters for unified telemetry."),
        ("obs-07-distributed-tracing-tempo", "Distributed Context Tracing with Grafana Tempo", "Trace distributed transactions across microservices with W3C traceparent context propagation."),
        ("obs-08-loki-promtail-log-pipeline", "Log Aggregation with Grafana Loki & Promtail", "Ship, index, and query structured container logs using Promtail and LogQL."),
        ("obs-09-kube-state-metrics-deployment", "Cluster Health Metrics with kube-state-metrics", "Expose Kubernetes object state metrics (pod restarts, deployment availability, quota usage)."),
        ("obs-10-blackbox-exporter-probes", "Synthetic Endpoint Monitoring with Blackbox Exporter", "Probe external endpoints over HTTP, TCP, and ICMP to monitor uptime and SSL certificate validity.")
    ]

    for lab_id, title, summary in obs_labs:
        reg(lab_id, title, summary,
            f"""graph LR
    App["Workload: {title}"] -->|Telemetry: {lab_id}| Collector["OTel Collector / Prometheus"]
    Collector --> Storage["Prometheus / Tempo / Loki ({lab_id})"]
    Storage --> Dashboards["Grafana Dashboards & Alertmanager"]""",
            f"""# {title}

Observability encompasses metrics, distributed traces, and structured logs to deliver full-stack visibility and rapid incident triage.

## Architectural Telemetry Pipeline

```mermaid
graph LR
    App["Application Workload"] -->|OTLP / Metrics| Collector["OTel Collector / Prometheus"]
    Collector --> Storage["Prometheus / Tempo / Loki"]
    Storage --> Dashboards["Grafana Dashboards & Alertmanager"]
```

## Key Operational Commands

```bash
# Query Prometheus directly via CLI
curl -s http://prometheus:9090/api/v1/query?query=up | jq .

# Verify OTel Collector pipeline health
kubectl logs -l app.kubernetes.io/name=opentelemetry-collector
```

## Common Production Gotchas & Anti-Patterns

1. **High Cardinality Metrics**: Putting unbounded user IDs or IP addresses into Prometheus metric labels, causing memory explosion.
2. **Missing Relabeling Configurations**: Failing to drop unused metrics before storage ingestion.
3. **Broken Trace Header Propagation**: Forgetting to forward the `traceparent` HTTP header in downstream RPC client calls.

## Security & Reliability Best Practices

- **Enforce Metric Drop Rules**: Drop high-cardinality ephemeral labels at scrape time.
- **Configure Multi-Window Alerts**: Implement multi-window burn rate alerts to eliminate false-positive alert fatigue.""",
            ["Injecting high-cardinality labels (UUIDs, IP addresses) into Prometheus metrics, causing OOM.",
             "Failing to forward W3C traceparent headers, breaking distributed trace continuity.",
             "Missing matching labels on ServiceMonitors preventing Prometheus from finding scrape targets."],
            "Follow the four Golden Signals (Latency, Traffic, Errors, Saturation) in all dashboard and alert definitions.")
