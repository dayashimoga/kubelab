# KubeLab Threat Model

## Attack Surface

| Vector | Threat | Mitigation | Status |
|---|---|---|---|
| **API endpoints** | Unauthenticated access | JWT middleware on all routes | ✅ Implemented |
| **Auth endpoints** | Brute force login | IP-based rate limiting | ✅ Implemented |
| **Password storage** | Credential theft | Argon2 hashing | ✅ Implemented |
| **WebSocket terminal** | Session hijacking | Token validation + ownership check | ✅ Implemented |
| **WebSocket terminal** | Host escape | Sandboxed process, env sanitization | ✅ Implemented |
| **Lab manifests** | Privileged pod creation | Server-side admission controller | ✅ Implemented |
| **Lab manifests** | Host filesystem access | hostPath rejection in admission | ✅ Implemented |
| **Lab manifests** | Container runtime escape | Socket mount rejection | ✅ Implemented |
| **Lab manifests** | Cluster admin escalation | cluster-admin binding rejection | ✅ Implemented |
| **Lab namespace** | Cross-tenant access | Dedicated namespace + RBAC | ✅ Implemented |
| **Lab namespace** | Resource exhaustion | LimitRange + ResourceQuota | ✅ Implemented |
| **Lab namespace** | Cloud metadata access | IMDS NetworkPolicy blocking | ✅ Implemented |
| **Container images** | Known CVEs | cargo audit + pnpm audit in CI | ✅ Implemented |
| **CORS** | Cross-origin attacks | Restrictive CORS policy | ✅ Implemented |
| **Dependencies** | Supply chain attacks | Lockfiles committed, audited | ✅ Implemented |

## Admission Controller Rules

The server-side admission controller (`services/api/src/admission.rs`) rejects:

1. `privileged: true` in security contexts
2. `hostNetwork: true`, `hostPID: true`, `hostIPC: true`
3. `hostPath` volume mounts
4. Runtime socket mounts (`docker.sock`, `containerd.sock`, `cri-dockerd.sock`)
5. `ClusterRoleBinding` with `cluster-admin` role
6. Containers running as root (UID 0)
7. Containers without resource limits

## Security Testing

```bash
# Run all security tests
cargo test -p kubelab-api --test security_adversarial_test
cargo test -p kubelab-api --test manifest_admission_test
cargo test -p kubelab-api --test terminal_isolation_test
cargo test -p kubelab-api --test cors_csrf_test
cargo test -p kubelab-api --test rate_limit_auth_test
```
