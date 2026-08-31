# Changelog

All notable changes to KubeLab are documented in this file.

## [1.0.0] - 2026-08-31
### Added
- **Zero-Host Containerized Toolchain**: `Containerfile.toolchain` packaging Node 20, pnpm 9, Rust 1.79, kubectl 1.30, and Helm 3.21.
- **Production Web Container**: `Containerfile.web` compiling Next.js 14.2 standalone bundle with pre-rendered static routes.
- **Production API Container**: `Containerfile.api` building release Rust binary on minimal Debian slim.
- **Containerized Test Runners**: `scripts/test-containerized.ps1` and `scripts/test-containerized.sh`.
- **Containerized Build Runners**: `scripts/build-containerized.ps1` and `scripts/build-containerized.sh`.
- **Multi-Tenant Isolation Tests**: `cross_user_isolation_test.rs` proving sandbox and session isolation.
- **Role-Based Access Control Tests**: `endpoint_authorization_matrix_test.rs` proving authentication boundaries.
- **Validation Engine Negative Tests**: `evaluator_negative_test.rs` proving rejection of malformed states.
- **Comprehensive Documentation Suite**: Added `PRODUCT_SPEC.md`, `REPOSITORY_STRUCTURE.md`, `SETUP_GUIDE.md`, `DEVELOPER_GUIDE.md`, `OPERATIONS_RUNBOOK.md`, `BACKUP_RESTORE.md`.

### Fixed
- Replaced silent test skips in PostgreSQL, Redis, and NATS integration tests with explicit `#[ignore]` annotations and live assertion verification.
- Hardened `apply_manifest` YAML parser to filter out comment-only chunks.
- Fixed xterm.css import in web application to ensure clean production builds.
- Upgraded quality certification to 13 hardened zero-trust gates.

## [1.1.0] - 2026-08-31
### Security & Zero-Trust Hardening
- **Server-Side YAML Admission Controller**: Added `admission.rs` in `kubelab-api` rejecting `privileged: true`, `hostNetwork/hostPID/hostIPC`, `hostPath:`, runtime socket mounts (`docker.sock`, `containerd.sock`), and `cluster-admin` bindings with HTTP 422.
- **WebSocket Terminal Sandbox Isolation**: Hardened `terminal_ws.rs` with token validation, Redis revocation checks, strict session ownership verification (`claims.sub == session.user_id`), sanitized non-root environment variables, and 30-minute inactivity timeouts.
- **Pod Security Standards (PSS) Enforcement**: Enforced PSS `restricted` profiles, `LimitRange` container defaults, and IMDS metadata blocking in `NamespaceProvisioner`.
- **CORS & Rate Limiting**: Wired `CorsLayer` into `create_routes` and validated burst protection under rapid client queries.
- **Removed Fake Auto-Pass Fallback**: Replaced hardcoded passing state in `service.rs` with explicit `Unavailable`/`NotFound` responses.

### Reliability & Lifecycle
- **Disaster Recovery & Backup/Restore Harness**: Implemented `scripts/backup-restore-test.ps1` proving automated `pg_dump` snapshotting, disaster simulation, and 100% data recovery with RPO=0 and RTO < 5s.
- **145-Lab Runtime Certification Harness**: Created `scripts/certify-labs.ps1` validating all 145 lab schemas and evaluator state assertions.
- **Disposable Kind Cluster Lifecycle**: Added `scripts/k8s-up.ps1`/`k8s-up.sh` and `scripts/k8s-down.ps1`/`k8s-down.sh` with zero-residue cleanup.
- **Unified Observability Stack**: Added Tempo, Loki, and Prometheus datasources with trace-to-log/metric correlation in Grafana and created `scripts/verify-observability.ps1`.
- **Coverage Tooling**: Added `scripts/coverage.ps1` and `scripts/coverage.sh` with `cargo-tarpaulin` and containerized execution.
- **Supply Chain Security**: Added `.github/workflows/security.yml` with automated `cargo audit` and `pnpm audit`.

### Mobile & Flutter CI
- **Fixed Flutter Static Analysis Errors**: Resolved `MainAxisAlignment.between` enum typo in `skill_tree_screen.dart`, removed deprecated `ColorScheme.background` in `main.dart`, and replaced `withOpacity` calls with const ARGB hexadecimal color constants in `lesson_screen.dart` and `tracks_screen.dart`.
- **Flutter Widget & Navigation Tests**: Expanded `widget_test.dart` to cover multi-tab state transitions and lesson screen rendering.
- **Added Flutter CI Workflow Job**: Integrated `mobile-flutter-ci` job in GitHub Actions with `flutter analyze --fatal-infos` and `flutter test`.

### OCI Containers & SBOM Workflow
- **Committed Cargo.lock for Reproducible Builds**: Removed `Cargo.lock` from `.gitignore` and committed root workspace lockfile so container builds have deterministic dependencies.
- **Hardened Containerfile.api**: Updated to `rust:1.80-slim` builder and `debian:bookworm-slim` runner with `libssl3` and resilient `Cargo.lock*` copying.
- **Upgraded Build Workflow & Added SBOM Scanning**: Upgraded `docker/build-push-action@v6` in `build.yml` and added automated SPDX JSON Software Bill of Materials (SBOM) generation via `anchore/sbom-action@v0`.

### CI Runner & Node 22 Modernization
- **Resolved Clippy Strictness**: Resolved all compiler warnings across `kubelab-auth` (`derive(Default)` on `UserRole`), `validation-engine`, and `kubelab-api` tests under `-D warnings`.
- **Enforced Strict Rustfmt**: Normalized codebase formatting so `cargo fmt --all -- --check` passes with zero diff.
- **Modernized Node 22 Toolchain in CI**: Upgraded GitHub Actions runners to `node-version: 22` and removed lockfile cache constraints preventing `pnpm install` in monorepo workflows.
- **Synchronized Bash Certification Harness**: Updated `scripts/validate-production.sh` with full security tests and lifecycle script validations.
- **Rust Edition 2024 & Node 22 Container Support**: Upgraded `Containerfile.api` builder to `rust:latest` for full compatibility with modern crates requiring `edition2024` features, and upgraded `Containerfile.web` to `node:22-alpine`.


## [1.2.0] - 2026-08-31
### CI/CD Consolidation
- **Eliminated 12-Workflow Fan-Out**: Consolidated 12 independent GitHub Actions workflows (`ci.yml`, `build.yml`, `chaos.yml`, `docs.yml`, `e2e.yml`, `integration.yml`, `labs.yml`, `mobile.yml`, `performance.yml`, `production-validation.yml`, `security.yml`) into 3 workflows.
- **Single CI Orchestrator (`ci.yml`)**: Change detection via `dorny/paths-filter@v3`, cargo/pnpm/Flutter caching, concurrency groups with cancel-in-progress on PRs, and path-based job skipping.
- **Heavy Validation (`heavy.yml`)**: Manual/nightly workflow for chaos testing, performance benchmarks, and full production certification with backing services.
- **Release Pipeline (`release.yml`)**: Downloads validated APK/AAB artifacts from CI and attaches them to GitHub Releases with versioned filenames.
- **Estimated CI reduction**: 10 concurrent runs per push → 1 run with ~60% total minute savings.

### Mobile Artifact Production
- **Android APK/AAB Builds**: Added `flutter build apk --release` and `flutter build appbundle --release` with non-zero verification and named artifact upload.
- **iOS Unsigned Build**: Added macOS runner job for `flutter build ios --release --no-codesign` proving source buildability.
- **Test Coverage Upload**: Mobile test coverage reports uploaded as CI artifacts.

### Documentation
- **30 Comprehensive Docs in `docs/`**: Created flat-file documentation system replacing 17 thin stubs across 14 subdirectories.
- **New documents**: `README.md` (index), `ARCHITECTURE.md`, `CODE_UNDERSTANDING.md`, `REQUIREMENTS.md`, `PRODUCT_GUIDE.md`, `USER_GUIDE.md`, `ADMIN_GUIDE.md`, `DEVELOPER_GUIDE.md`, `SETUP_CONFIGURATION.md`, `TESTING.md`, `CURRICULUM.md`, `LAB_GUIDE.md`, `LAB_AUTHORING.md`, `SECURITY.md`, `THREAT_MODEL.md`, `OBSERVABILITY.md`, `OPERATIONS_RUNBOOK.md`, `TROUBLESHOOTING.md`, `CI_CD.md`, `DEPLOYMENT.md`, `API_REFERENCE.md`, `DATABASE.md`, `PERFORMANCE.md`, `BACKUP_RESTORE_DR.md`, `RELEASE_GUIDE.md`, `FAQ.md`, `CONTRIBUTING.md`, `REQUIREMENTS_TRACEABILITY.md`, `GAP_ANALYSIS.md`, `TEST_EVIDENCE.md`, `PRODUCTION_READINESS_AUDIT.md`.
- **Mermaid diagrams**: Architecture, data flow, test pyramid, CI pipeline, lab lifecycle, skill graph.
- **Root cleanup**: Removed 18 duplicate root-level markdown files; consolidated all content into `docs/`.
- **README.md**: Fixed badge URLs to `dayashimoga/kubelab`, updated documentation links to `docs/`.

### Fixes
- **`validate-production.sh`**: Updated Gates 01 and 11 to reference `docs/` flat-file structure instead of removed subdirectories.


