# KubeLab Performance

## Benchmarks
Run via: `cargo bench --workspace`

## Targets

| Metric | Target | Measurement |
|---|---|---|
| API response latency (p95) | < 200ms | Prometheus histogram |
| Lab provisioning | < 30s | Trace span duration |
| 154-lab schema validation | < 5s | CI job duration |
| Web build time | < 60s | CI job duration |
| Container image size (API) | < 100MB | `podman images` |
| Container image size (Web) | < 200MB | `podman images` |

## Scaling Guidelines
- **API**: Stateless — horizontal scale behind load balancer
- **PostgreSQL**: Vertical scale + read replicas
- **Redis**: Single instance sufficient for < 10K concurrent sessions
- **NATS**: Clustered mode for HA
- **Lab clusters**: Multiple Kind/k3d clusters or remote cluster pool

## Optimization
- Rust release builds with LTO for minimal binary size
- Alpine-based container images for minimal footprint
- Cargo workspace for shared compilation
- pnpm for efficient Node.js dependency management
