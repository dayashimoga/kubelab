# KubeLab Security & Multi-Tenancy Audit

## 1. Threat Model & Isolation Architecture

KubeLab implements multi-layered security controls to protect the host, container runtime, and other learners from malicious activity or unintended container escapes.

```text
[ Learner Browser ]
       │ HTTPS / WSS
       ▼
[ Axum API Gateway ] ──── AuthClaims Middleware (JWT Verification & Role Check)
       │
       ▼
[ Ephemeral Sandbox ] (Non-Root UID 10001, CAP_DROP ALL, Read-Only Rootfs)
       │
       ▼
[ Kubernetes Namespace ] ── NetworkPolicy (Default Deny Ingress/Egress except DNS/API)
```

---

## 2. Security Controls Matrix

| Control Category | Mechanism | Verification Method | Status |
|---|---|---|---|
| **Authentication** | Argon2id password hashing (19MB, 2 iter, 1 lane) | Unit test in `services/auth` | **PASS** |
| **Token Security** | HS256 JWT with expiration and subject claims | Automated tamper test in `security_adversarial_test.rs` | **PASS** |
| **Endpoint Auth** | `AuthClaims` extractor rejecting missing/invalid tokens | Integration test in `security_adversarial_test.rs` | **PASS** |
| **Injection Resilience** | Strict JSON schema deserialization & parameter binding | Adversarial SQLi / Command injection tests | **PASS** |
| **Path Traversal** | URL-encoded traversal sanitized by Axum router | Path traversal test (`/v1/labs/..%2F..%2Fetc%2Fpasswd`) | **PASS** |
| **Container Isolation** | Non-root UID 10001, dropped capabilities in Containerfile | Verified in `Containerfile.api` and `Containerfile.web` | **PASS** |
| **Sandbox Network** | Default Deny NetworkPolicy in provisioned namespaces | Verified in `lab-orchestrator` | **PASS** |

---

## 3. Vulnerability Disclosure & Policy
Refer to [SECURITY.md](file:///H:/kubelab/SECURITY.md) for security bug bounty and vulnerability reporting procedures.
