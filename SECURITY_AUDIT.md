# KubeLab — Comprehensive Security Audit & Threat Modeling

## 1. Threat Modeling & Mitigation Summary

| Threat Vector | Potential Impact | Implemented Mitigation | Verification Test |
|---|---|---|---|
| **SQL Injection (SQLi)** | Data exfiltration, credential theft | Parameterized SQL via `sqlx` prepared queries. Zero raw string interpolation. | `tests/security_adversarial_test.rs` |
| **Sandbox Escape / Privilege Escalation** | Host takeover from learner pod | Pod Security Standard Restricted profile, `allowPrivilegeEscalation: false`, `runAsNonRoot: true`, `seccompProfile: RuntimeDefault`. | `tests/kube_client_test.rs`, `security_adversarial_test.rs` |
| **Cross-Tenant Network Probing** | Lateral movement between lab sandboxes | Default-deny Ingress NetworkPolicy automatically created for every sandbox namespace (`lab-isolate`). | `scripts/lab-up.ps1`, `tests/kube_client_test.rs` |
| **Brute Force & DoS Attacks** | API exhaustion, server crash | Token Bucket rate limiter enforcing 100 requests per minute with burst rejection. | `tests/rate_limit_test.rs` |
| **Credential Hijacking & Replay** | Session impersonation | Argon2id password hashing with random salt; short-lived JWT access tokens + Redis blacklist revocation on logout. | `tests/auth_flow_test.rs`, `tests/redis_session_test.rs` |
| **Unauthenticated WebSocket Execution** | Unauthorized shell access | JWT token verification guard on `/ws/terminal/:session_id` WebSocket handshake. | `services/api/src/routes/terminal_ws.rs` |

---

## 2. Cryptographic Standards
- **Password Hashing**: Argon2id with memory cost 19,456 KiB, iterations 2, parallelism 1.
- **Tokens**: HMAC-SHA256 JWT with expiration and audience claims.
- **Container Isolation**: Linux namespaces, cgroups v2, and seccomp syscall filtering.
