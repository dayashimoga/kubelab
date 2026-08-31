# KubeLab Gap Analysis

## Known Gaps and Remediation Status

| Area | Gap | Severity | Remediation | Status |
|---|---|---|---|---|
| **Mobile** | iOS signed IPA distribution | Medium | Requires Apple Developer credentials | 🔲 Planned |
| **Mobile** | Lab terminal on mobile | Low | "Continue on Desktop" handoff implemented | ✅ Mitigated |
| **CI** | End-to-end Playwright tests | Medium | Specs exist but backend must be running | 🔲 Needs test fixtures |
| **Observability** | Production Grafana dashboards | Medium | Datasources configured, dashboards planned | 🔲 Planned |
| **Helm** | Helm chart packaging | Low | Currently raw manifests + ArgoCD | 🔲 Planned |
| **AI Tutor** | External LLM API integration | Medium | Service framework built, needs API key config | 🔲 Planned |
| **Scaling** | Load testing results | Low | Benchmarks run but not published | 🔲 Planned |
| **Auth** | OAuth/OIDC social login | Low | JWT-based auth works, social login planned | 🔲 Planned |
| **WCAG** | Full accessibility audit | Medium | Semantic HTML + ARIA basics in place | 🔲 Needs audit |
