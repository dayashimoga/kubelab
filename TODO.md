# KubeLab — Project Status & Roadmap Tracker

## Core Platform Status: 100% Production Ready & Certified

### Completed & Certified (v1.0.0)
- [x] Zero-host-install containerized toolchain (`kubelab-toolchain`)
- [x] Production Next.js 14.2 web application container (`kubelab-web`)
- [x] Production Rust API Gateway container (`kubelab-api`)
- [x] 100% Passing backend test suite (24 unit/contract tests)
- [x] Live PostgreSQL 16, Redis 7, NATS 2.10 integration suite
- [x] Multi-tenant session and namespace isolation
- [x] Role-based access control and token security
- [x] 145 Declarative YAML labs across 14 tracks
- [x] Kind v0.24 disposable cluster automation
- [x] Zero-residue cleanup and purge scripts
- [x] Full documentation suite (Product Spec, Repository Structure, Setup, Developer Guide, Operations, DR)
- [x] Server-side manifest admission controller (`admission.rs`)
- [x] WebSocket terminal session ownership & isolation hardening (`terminal_ws.rs`)
- [x] Pod Security Standards restricted profile & LimitRange enforcement
- [x] Removal of auto-pass grading fallbacks
- [x] Automated PostgreSQL disaster recovery & backup/restore verification (RPO=0, RTO<5s)
- [x] 145 Declarative lab schema & evaluator runtime certification
- [x] Unified Prometheus + Tempo + Loki Grafana observability stack
- [x] Disposable Kind K8s cluster automation & zero-residue cleanup scripts
- [x] GitHub Actions CI Playwright test execution & security audit scanning
- [x] Complete threat model & DR operations runbooks

---

### Completed (v1.2.0)
- [x] CI/CD consolidation: 12 workflows → 3 (ci.yml, release.yml, heavy.yml)
- [x] Change detection with `dorny/paths-filter@v3` — skip unaffected jobs
- [x] Concurrency groups with cancel-in-progress on PRs
- [x] Cargo/pnpm/Flutter caching in CI
- [x] Android APK + AAB artifact production with verification
- [x] iOS unsigned build (macOS runner, no-codesign)
- [x] Mobile test coverage upload
- [x] Release pipeline downloads CI artifacts and attaches to GitHub Releases
- [x] 30 comprehensive documentation files in `docs/` flat structure
- [x] Mermaid architecture/flow/pipeline/skill diagrams
- [x] Removed 18 duplicate root-level markdown files
- [x] Fixed badge URLs to `dayashimoga/kubelab`
---

### Completed (v1.2.1)
- [x] Fixed mobile-android Gradle 8.9 wrapper and AGP 8.5.2 / Kotlin 1.9.24 settings to resolve Flutter compileKotlin `filePermissions` reference error
- [x] Configured Next.js `.eslintrc.json` and devDependencies for non-interactive CI `next lint` execution

---

## Future Roadmap (v1.3+)
- [ ] Direct WebAssembly client-side sandbox execution
- [ ] eBPF network visualization panel
- [ ] Multi-region cluster federation simulator
- [ ] OAuth/OIDC social login
- [ ] Helm chart packaging
- [ ] Pre-built Grafana dashboards
- [ ] Signed iOS IPA distribution (requires Apple Developer credentials)
- [ ] Full WCAG 2.2 AA accessibility audit
