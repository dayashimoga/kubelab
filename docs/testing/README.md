# Testing & Verification Framework

## Test Pyramid
- **Unit & Contract**: `cargo test --workspace`, Vitest, Flutter test
- **Integration**: Backed by live PostgreSQL, Redis, NATS
- **E2E**: Playwright full journey, accessibility (axe-core, WCAG 2.2 AA)
- **Security & Adversarial**: Privilege escalation, tenant escape, socket access attacks
- **Resilience & Chaos**: Chaos recovery, disaster recovery (backup-restore)
