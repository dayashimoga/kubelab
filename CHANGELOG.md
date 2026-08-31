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
