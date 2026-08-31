# KubeLab System Architecture & Component Interactions

## Microservices Topology

1. **`services/api` (Axum/Rust)**: High-throughput API gateway routing REST, GraphQL, and WebSocket protocols.
2. **`services/auth` (Rust)**: JWT issuance, session storage in Redis, and RBAC enforcement.
3. **`services/learning` (Rust)**: Manages tracks, modules, lessons, concept taxonomy, and full-text search.
4. **`services/assessment` (Rust)**: Quizzes, question randomization, timed exams, and scoring algorithms.
5. **`services/labs` (Rust)**: Lab definitions catalog, session requests, and interactive shell multiplexing.
6. **`services/lab-orchestrator` (Rust/Tokio)**: Podman/kind orchestrator, namespace provisioning, and garbage collection.
7. **`services/progress` (Rust)**: XP accumulation, streak calculations, badge unlocking, and Skill Graph DAG calculations.
8. **`services/notification` (Rust)**: Async event consumer broadcasting in-app alerts and webhooks via NATS.
9. **`services/ai-tutor` (Rust)**: Socratic AI assistant providing contextual hints and troubleshooting guidance.
10. **`packages/validation-engine` (Rust)**: Deterministic evaluation engine querying Kubernetes API to grade exercises.

## Event Bus Architecture (NATS)

```text
[Lab Session Created] ──────► NATS ("labs.session.created") ─────► Lab Orchestrator (Provision Cluster/NS)
[Task Submitted]     ──────► NATS ("labs.task.validate")   ─────► Validation Engine (Evaluate State)
[Task Passed]        ──────► NATS ("learning.task.passed") ─────► Progress Service (Award XP/Badges)
                                                            ─────► Notification Service (Dispatch Alert)
```
