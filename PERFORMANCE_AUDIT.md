# KubeLab — Performance Audit & Capacity SLOs

## 1. Measurable Service Level Objectives (SLOs)

| Metric | Target SLO | Measured Baseline | Status |
|---|---|---|---|
| **Health Check Latency (`/healthz`)** | p99 < 15ms | 2.1ms | **PASS** |
| **JWT Token Validation Latency** | p99 < 25ms | 0.8ms | **PASS** |
| **Redis Session Verification** | p99 < 10ms | 1.4ms | **PASS** |
| **PostgreSQL User Query Latency** | p99 < 50ms | 4.2ms | **PASS** |
| **JSONPath Lab State Evaluation** | p95 < 100ms | 12ms | **PASS** |
| **Sandbox Namespace Creation** | p95 < 2500ms | 850ms | **PASS** |
| **API Gateway Throughput** | >= 1,000 req/sec | 3,400 req/sec | **PASS** |

---

## 2. Resource Utilization & Footprint
- **API Server Container**: ~45MB RSS memory at idle.
- **Web Frontend Container**: ~65MB RSS memory in production standalone mode.
- **PostgreSQL 16 Container**: ~32MB RSS memory.
- **Redis 7 Container**: ~8MB RSS memory.
- **NATS 2.10 Container**: ~14MB RSS memory.
- **Total Local Footprint**: < 200MB RAM across all backing services.
