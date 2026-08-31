# KubeLab Developer Guide

## Local Development

### Prerequisites for Native Development
If you prefer native toolchains over containerized development:

| Tool | Version | Install |
|---|---|---|
| Rust | stable (latest) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 22+ | [nodejs.org](https://nodejs.org/) |
| pnpm | 9.12+ | `npm install -g pnpm@9.12.0` |
| Flutter | stable (latest) | [flutter.dev](https://flutter.dev/docs/get-started/install) |

### Build Commands

```bash
# Backend
cargo build --workspace              # Debug build
cargo build --release -p kubelab-api # Release API binary
cargo test --workspace               # All unit tests
cargo fmt --all                      # Format code
cargo clippy --workspace -- -D warnings  # Lint

# Web
pnpm install --frozen-lockfile=false
pnpm --filter @kubelab/web build
pnpm --filter @kubelab/web dev       # Dev server with hot reload

# Mobile
cd apps/mobile
flutter pub get
flutter analyze --fatal-infos
flutter test
flutter build apk --release
flutter build appbundle --release
```

## Architecture Conventions

### Rust Services
- Each service is a Cargo workspace member under `services/`
- Services expose a library crate consumed by the API gateway
- The API gateway (`services/api`) is the only binary with network listeners
- Use `thiserror` for error types, `anyhow` for application errors
- Use `tracing` for structured logging, not `println!`

### Web (Next.js)
- App Router with TypeScript strict mode
- Components in `src/components/`, pages in `src/app/`
- Shared types from `packages/shared-types/`
- UI components from `packages/ui/`

### Mobile (Flutter)
- Material Design 3 with dark theme
- Screens in `lib/screens/`
- No deprecated APIs (`withOpacity` → `withAlpha`, `background` → `surface`)
- Widget tests in `test/`

### Testing
- Rust unit tests: `#[test]` and `#[tokio::test]` in source files or `tests/` directories
- Integration tests: `#[ignore]` attribute, require backing services
- Web E2E: Playwright specs in `apps/web/e2e/`
- Mobile: Flutter widget tests in `apps/mobile/test/`

## Contribution Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make changes following the coding conventions above
4. Run locally: `cargo fmt && cargo clippy -- -D warnings && cargo test`
5. Push and open a PR against `main`
6. CI will run automatically — all checks must pass
7. Get review and merge

## Code Quality

- `cargo fmt --all -- --check` enforced in CI
- `cargo clippy --workspace --all-targets -- -D warnings` enforced in CI
- `flutter analyze --fatal-infos --fatal-warnings` enforced in CI
- Playwright E2E tests run on every web change
