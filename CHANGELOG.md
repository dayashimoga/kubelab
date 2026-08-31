# Changelog

All notable changes to the KubeLab platform are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-31 - True Production Certification & 145 Labs

### Added
- **145 Unique Declarative Labs**: Comprehensive YAML definitions across 14 tracks (Linux, Kubernetes, Admin, Networking, Security, Storage, Helm, GitOps, Observability, Istio, SRE, Incidents, Platform Engineering, CKA/CKAD/CKS).
- **Dynamic Lab Catalog Loader**: `services/labs/src/catalog.rs` recursively scans and loads all lab files from disk with schema validation.
- **Real Interactive Terminal**: WebSocket terminal connecting to real PTY / subprocess shell (`/bin/bash` or `powershell.exe`) with `KUBELAB_NAMESPACE` and JWT query authentication. Eradicated fake command fallback.
- **JWT Refresh Token Lifecycle & Redis Revocation**: Full access/refresh token rotation with instant blacklist revocation on logout in Redis.
- **Configurable AI Tutor**: Ollama container and OpenAI provider integration with 5 contextual pedagogical modes (Explain, Socratic, Hint, Diagnose, Review).
- **Rich Curriculum Data**: Lessons, trivia, interview questions, mistakes to avoid, and production tips across all 12 learning tracks in `services/learning/src/data.rs`.
- **Grafana Dashboard Auto-Provisioning**: Datasource and dashboard provisioning configs with `kubelab-overview.json` monitoring API throughput, p99 latency, and active sessions.
- **Kind Cluster Provisioning Config**: `infrastructure/kind/cluster-config.yaml` for automated disposable Kind cluster creation with Podman.
- **PWA Service Worker**: `apps/web/public/sw.js` with offline route precaching and navigation fallback.
- **Mobile Companion App**: Complete Flutter Material 3 Dark theme companion app in `apps/mobile/lib/`.
- **Production Audit & Traceability Docs**: `PRODUCTION_READINESS_AUDIT.md`, `REQUIREMENTS_TRACEABILITY.md`, `GAP_ANALYSIS.md`, `CURRICULUM_MATRIX.md`, `CONTENT_COVERAGE.md`, `TEST_EVIDENCE.md`, `SECURITY_AUDIT.md`, `PERFORMANCE_AUDIT.md`, `LAB_CERTIFICATION.md`.

### Changed
- Refactored `services/api/src/routes/labs.rs` to extract user ID from `AuthClaims`, record database sessions, and emit typed NATS domain events.
- Updated `validate-production.ps1` with strict validation gates for $\ge 120$ labs and full workspace execution.
