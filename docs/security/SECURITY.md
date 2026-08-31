# Security Architecture & Threat Modeling

## Threat Matrix & Mitigations

| Threat Vector | Mitigation Strategy |
|---|---|
| **Learner Container Escape** | Non-root UID (`10001`), `CAP_DROP ALL`, `no-new-privileges`, read-only rootfs, unprivileged seccomp profile |
| **Inter-Tenant Snooping** | Dedicated per-session Kubernetes namespaces with strict default-deny NetworkPolicies |
| **Denial of Service (CPU/RAM)** | Linux cgroup v2 limits, Kubernetes `ResourceQuotas` and `LimitRanges` |
| **API Abuse / Credential Stuffing** | Argon2id hashing, Redis rate-limiting (token bucket algorithm), short-lived JWTs |
| **SSRF via In-Lab Cluster** | Strict egress firewall blocking access to cloud metadata services (`169.254.169.254`) and internal control plane |
| **WebSocket Hijacking** | Secure session tokens tied to user ID and origin validation |
