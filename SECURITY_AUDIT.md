# KubeLab — Comprehensive Security & Threat Model Audit

## 1. Zero-Trust Architecture Principles
1. **Never trust the client**: All evaluations, state retrievals, and manifest transformations happen strictly server-side.
2. **Never expose administrative kubeconfigs**: Learners interact via namespace-scoped server-side apply. No cluster-admin tokens or kubeconfig files are exposed to browsers or mobile clients.
3. **Strict Sandboxing**:
   - `runAsNonRoot: true`
   - `allowPrivilegeEscalation: false`
   - `capabilities: drop: ["ALL"]`
   - `seccompProfile: { type: "RuntimeDefault" }`
   - `ResourceQuota` on every sandbox namespace
   - Default-deny `NetworkPolicy` preventing cross-sandbox traversal.

---

## 2. Adversarial Penetration Test Results

| Attack Vector | Test File | Target Subsystem | Result | Defense Mechanism |
|---|---|---|---|---|
| **SQL Injection (SQLi)** | `security_adversarial_test.rs` | Auth & DB | **BLOCKED** | Parameterized queries with SQLx |
| **Cross-Site Scripting (XSS)** | `security_adversarial_test.rs` | API & Web | **BLOCKED** | Strict input sanitation & React escaping |
| **Path Traversal (`../..`)** | `security_adversarial_test.rs` | Lab Engine | **BLOCKED** | Canonical path validation & whitelist |
| **Rate Limit Flooding** | `rate_limit_test.rs` | Gateway | **BLOCKED** | Token bucket rate limiter (100 req/min) |
| **Forged / Tampered JWT** | `endpoint_authorization_matrix_test.rs` | Auth Gateway | **BLOCKED** | HMAC-SHA256 signature verification |
| **Cross-Tenant Hijack** | `cross_user_isolation_test.rs` | Sandboxes | **BLOCKED** | Session-bound namespace isolation |

---

## 3. Vulnerability Disclosure & Policy
Report vulnerabilities confidentially to `security@kubelab.io`. All critical security updates are patched with zero-downtime rolling deployments.
