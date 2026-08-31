# Declarative Lab Lifecycle & Orchestration State Machine

```text
 REQUEST ──► SCHEDULE ──► PROVISION ──► BOOTSTRAP ──► HEALTHCHECK ──► READY
                                                                         │
                                                                         ▼
 DESTROY ◄── CHECKPOINT ◄── SCORE ◄── VALIDATE ◄────────────────────── LEARN
```

## Lifecycle States

1. **`REQUEST`**: User initiates lab via API/Web UI. Authentication and rate-limits verified.
2. **`SCHEDULE`**: Lab Scheduler checks available capacity in local worker node or remote lab pool.
3. **`PROVISION`**: Ephemeral sandbox created (dedicated namespace, ResourceQuotas, LimitRanges, NetworkPolicies).
4. **`BOOTSTRAP`**: Initial state manifests applied (deployments, buggy services, misconfigurations).
5. **`HEALTHCHECK`**: Orchestrator probes sandbox until initial conditions are met.
6. **`READY`**: Terminal session initialized, WebSocket route opened, timer started.
7. **`LEARN`**: Learner interacts via real terminal, Monaco YAML editor, or resource visualizer.
8. **`VALIDATE`**: Learner submits task; validation engine evaluates live system state assertions.
9. **`SCORE`**: Points awarded based on objective completion, hint penalties, and time bonuses.
10. **`CHECKPOINT`**: Progress persisted to PostgreSQL and broadcast via NATS.
11. **`DESTROY`**: Sandbox namespace terminated, temporary resources purged, network rules cleaned.
