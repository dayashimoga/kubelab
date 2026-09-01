# KubeLab Architecture Specification

## Architecture Principles
1. **Zero-Trust by Default**: Strict namespace isolation, PSS restricted, IMDS blocking, admission control.
2. **Deterministic Evaluation**: No fake fallbacks; grading verifies actual Kubernetes API resource state.
3. **Zero Host Pollution**: All toolchains, databases, and dependencies run in containers or disposable clusters.
4. **Resilience & Observability**: Full telemetry correlation (metrics, logs, traces) via OpenTelemetry, Prometheus, Tempo, Loki.

## Subsystem Details
- **API Gateway (`services/api`)**: Axum 0.7 REST and WebSocket server.
- **Lab Orchestrator (`services/lab-orchestrator`)**: K8s namespace lifecycle, resource limits, and chaos injection.
- **Validation Engine (`packages/validation-engine`)**: YAML schema parser, JSONPath assertion evaluator.
- **Client Tier (`apps/web`, `apps/mobile`)**: Next.js 14 web app and Flutter mobile companion.
