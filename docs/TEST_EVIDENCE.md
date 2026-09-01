# KubeLab Test Evidence

## CI Pipeline Test Execution

### Quality Gate
- `cargo fmt --all -- --check` → PASS
- `cargo clippy --workspace --all-targets -- -D warnings` → PASS

### Unit Tests
- `cargo test --workspace --all-targets` → PASS
- Tests cover: API contracts, auth flows, terminal isolation, CORS/CSRF, rate limiting, admission control, grading engine

### Integration Tests
- `cargo test --workspace -- --ignored` → PASS (with PostgreSQL 16, Redis 7, NATS 2.10)
- Verifies database migrations, session management, event publishing

### Validation Engine
- `cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path labs/` → PASS
- Validates all 154 declarative lab schema definitions

### Web Build
- `pnpm --filter @kubelab/web build` → PASS
- TypeScript strict mode compilation
- Next.js production build with optimized bundles

### Mobile
- `flutter analyze --fatal-infos --fatal-warnings` → PASS
- `flutter test --coverage` → PASS
- `flutter build apk --release` → APK produced (non-zero verified)
- `flutter build appbundle --release` → AAB produced (non-zero verified)

### Container Builds
- `docker buildx build --file Containerfile.api` → PASS
- `docker buildx build --file Containerfile.web` → PASS

### Security
- `cargo audit` → PASS (known advisories acknowledged)
- `pnpm audit --prod` → PASS

### Production Certification
- 13-gate `validate-production.sh` → PASS
- Gates: formatting, linting, unit tests, lab validation, documentation structure, container definitions, security checks
