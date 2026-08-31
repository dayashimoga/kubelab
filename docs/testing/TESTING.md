# Testing Strategy & Quality Assurance Gates

## Testing Layers

```text
▲ E2E / Production Certification Gate (scripts/validate-production.sh)
│
├── Chaos & Resilience Tests (inject failure, verify recovery)
├── Security & Sandboxing Tests (container breakouts, SSRF, non-root)
├── Deterministic Lab Tests (provision, learner action, state assert, destroy)
├── Mobile Integration & Widget Tests (Flutter driver)
├── Web UI & Accessibility (WCAG 2.2 AA, axe-core)
├── API & Integration Tests (Axum + PostgreSQL + Redis + NATS)
└── Unit Tests (Rust & TypeScript, >= 95% target coverage)
```

## Quality Thresholds

- **Mandatory Pass Rate**: 100% (Zero skipped or ignored tests)
- **Minimum Test Coverage**: > 90% (Hard floor), >= 95% for core validation engine and security
- **Accessibility Standard**: WCAG 2.2 Level AA compliance across 320px to 2560px+ breakpoints
- **Terminal Latency Benchmark**: < 100ms round-trip WebSocket echo
