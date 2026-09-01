# KubeLab Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly via email to **security@kubelab.io**. Do NOT open a public GitHub issue.

We will acknowledge receipt within 48 hours and provide a detailed response within 5 business days.

## Security Architecture Overview

KubeLab implements defense-in-depth across every tier:

### Authentication & Authorization
- **JWT tokens** signed with HS256 (configurable secret, min 32 chars enforced)
- **Argon2id** password hashing (memory-hard, GPU/ASIC resistant)
- **RBAC**: Learner, Instructor, Admin roles with endpoint-level authorization matrix
- **Session management**: Redis-backed with explicit revocation support
- **Token validation** on every WebSocket upgrade and API request

### API Perimeter
- **CORS**: Restrictive origin allowlist (`CorsLayer` via `tower-http`); dev defaults to localhost-only
- **Rate limiting**: IP-based burst protection on auth and sensitive endpoints
- **Input validation**: `validator` crate with derive macros on all request payloads
- **CSRF protection**: SameSite cookie attributes on session tokens

### Lab Sandbox Isolation
- **Namespace isolation**: Each learner receives a dedicated Kubernetes namespace
- **Pod Security Standards (PSS)**: `restricted` profile enforced at namespace level
- **LimitRange**: CPU (500m) / memory (512Mi) defaults and maximums per container
- **ResourceQuota**: Hard limits on total namespace resource consumption
- **NetworkPolicy**: IMDS metadata endpoint (169.254.169.254) blocked; cross-namespace traffic denied by default
- **Server-side admission control**: Regex-based YAML admission rejects:
  - `privileged: true` containers
  - `hostNetwork/hostPID/hostIPC: true`
  - `hostPath` volume mounts
  - Container runtime socket mounts (`/var/run/docker.sock`, containerd, cri-o, podman)
  - Elevated Linux capabilities (`SYS_ADMIN`, `NET_ADMIN`, `SYS_PTRACE`)
  - System namespace targeting (`kube-system`, `default`, `kube-public`)
  - `cluster-admin` ClusterRoleBinding escalation

### Terminal Security
- JWT validation required before WebSocket upgrade
- Session ownership check (`claims.sub == session.user_id || role == admin`)
- Environment variable sanitization (`env_clear()` + explicit safe-list)
- 30-minute idle timeout with automatic session termination
- Process-level sandbox isolation

### Container Security
- All containers run as non-root users with dropped capabilities
- Minimal base images: `debian:bookworm-slim` (API), `node:22-alpine` (web)
- No runtime sockets mounted in any container
- All compose services include health checks

### Supply Chain
- `cargo audit` for Rust crate vulnerability scanning (CI enforced)
- `pnpm audit` for NPM dependency scanning (CI enforced)
- GitHub Actions pinned to SHA digests
- SBOM generation for release artifacts
- Container image scanning via Trivy

## Supported Versions

| Version | Supported |
|---|---|
| 1.0.x | ✅ Active |
| < 1.0 | ❌ Not supported |

## Security Testing

Security is validated through multiple automated test suites:

```bash
# Adversarial attack tests
cargo test -p kubelab-api --test security_adversarial_test

# Manifest admission policy tests
cargo test -p kubelab-api --test manifest_admission_test

# Terminal sandbox isolation tests
cargo test -p kubelab-api --test terminal_isolation_test

# CORS/CSRF validation tests
cargo test -p kubelab-api --test cors_csrf_test

# Rate limiting and auth tests
cargo test -p kubelab-api --test rate_limit_auth_test

# Cross-user isolation tests
cargo test -p kubelab-api --test cross_user_isolation_test

# Endpoint authorization matrix tests
cargo test -p kubelab-api --test endpoint_authorization_matrix_test
```

See [docs/SECURITY.md](docs/SECURITY.md) and [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) for complete details.
