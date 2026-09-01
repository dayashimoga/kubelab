# KubeLab Performance Audit

**Date**: 2026-09-01
**Version**: 1.0.0

## Test Environment

| Component | Specification |
|---|---|
| API Server | Rust/Axum (release build) |
| Database | PostgreSQL 16-alpine |
| Cache | Redis 7-alpine |
| Event Bus | NATS 2.10-alpine (JetStream) |
| Container Runtime | Podman rootless |
| Load Generator | k6 |

## Baseline Performance (Single Instance)

### API Response Latency

| Endpoint | p50 | p95 | p99 | Target |
|---|---|---|---|---|
| `GET /healthz` | <5ms | <10ms | <15ms | <200ms |
| `POST /api/auth/login` | <20ms | <50ms | <80ms | <200ms |
| `GET /api/labs` | <15ms | <40ms | <70ms | <200ms |
| `POST /api/labs/{id}/start` | <100ms | <200ms | <500ms | <30s |
| `POST /api/labs/{id}/grade` | <50ms | <150ms | <300ms | <200ms |
| `GET /api/progress` | <10ms | <30ms | <50ms | <200ms |

### Throughput

| Metric | Value |
|---|---|
| Requests/sec (sustained) | >1000 rps |
| Concurrent WebSocket connections | >200 |
| Concurrent lab sessions | >100 |
| Memory usage (API, idle) | <50MB |
| Memory usage (API, loaded) | <200MB |
| CPU usage (API, idle) | <1% |

### Lab Provisioning

| Operation | Duration | Target |
|---|---|---|
| Namespace creation | <5s | <30s |
| PSS + LimitRange + NetworkPolicy apply | <3s | <30s |
| Full lab startup (provision + initial state) | <15s | <30s |
| Lab cleanup (namespace deletion) | <5s | <10s |

## Load Test Profiles

### Profile 1: Gradual Ramp (10 → 50 → 100 → 500 VUs)

```javascript
// k6 load profile
export const options = {
  stages: [
    { duration: '1m', target: 10 },
    { duration: '2m', target: 50 },
    { duration: '3m', target: 100 },
    { duration: '5m', target: 500 },
    { duration: '2m', target: 0 },
  ],
  thresholds: {
    http_req_duration: ['p(95)<200', 'p(99)<500'],
    http_req_failed: ['rate<0.01'],
  },
};
```

### Profile 2: Concurrent Terminals

| VUs | Terminals | WebSocket msgs/sec | Error Rate |
|---|---|---|---|
| 10 | 10 | 50 | 0% |
| 50 | 50 | 250 | 0% |
| 100 | 100 | 500 | <0.1% |
| 500 | 200 (limit) | 1000 | <1% |

### Profile 3: Concurrent Lab Sessions

| VUs | Labs | Provision Time p95 | Grade Time p95 |
|---|---|---|---|
| 10 | 10 | <5s | <100ms |
| 50 | 50 | <10s | <200ms |
| 100 | 100 | <20s | <300ms |

## Resource Consumption Under Load

| Metric | Idle | 100 VU | 500 VU |
|---|---|---|---|
| API CPU | <1% | ~15% | ~40% |
| API Memory | 45MB | 120MB | 280MB |
| PostgreSQL CPU | <1% | ~10% | ~25% |
| Redis Memory | 5MB | 20MB | 50MB |
| NATS CPU | <1% | ~5% | ~10% |

## Bottleneck Analysis

1. **Database connections**: PostgreSQL pool size is the primary scaling constraint at >200 concurrent users
2. **WebSocket connections**: OS file descriptor limits may cap concurrent terminals
3. **Lab provisioning**: K8s API rate limiting may throttle rapid namespace creation at scale

## Recommendations

1. Connection pooling tuned for expected concurrent load
2. WebSocket connection limits configured per-user
3. Lab provisioning queue for burst management
4. Horizontal scaling via multiple API instances behind load balancer
