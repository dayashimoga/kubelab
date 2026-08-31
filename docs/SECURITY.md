# KubeLab Security

## Security Architecture

### Authentication & Authorization
- **JWT tokens**: Signed with HS256, configurable secret (min 32 chars)
- **Argon2 password hashing**: Memory-hard, resistant to GPU/ASIC attacks
- **Role-based access**: Learner, Instructor, Admin roles
- **Session management**: Redis-backed with revocation support

### API Security
- **CORS**: Restrictive origin policy via `tower-http` CorsLayer
- **Rate limiting**: IP-based burst protection on auth endpoints
- **Input validation**: `validator` crate for request payload validation
- **CSRF protection**: SameSite cookie attributes

### Lab Sandbox Isolation
- **Namespace isolation**: Each learner gets a dedicated K8s namespace
- **Pod Security Standards**: `restricted` profile enforced (no privileged, no hostNetwork, no hostPID)
- **LimitRange**: CPU/memory defaults and maximums enforced
- **NetworkPolicy**: IMDS metadata endpoint (169.254.169.254) blocked
- **Admission control**: Server-side rejection of dangerous manifests

### Container Security
- **Non-root execution**: All containers run as non-root users
- **Minimal base images**: `debian:bookworm-slim` for API, `node:22-alpine` for web
- **No runtime sockets**: Docker/containerd sockets never mounted
- **Health checks**: All containers have HEALTHCHECK directives

### Terminal Security
- **JWT validation on WebSocket upgrade**: Token verified before connection
- **Session ownership**: `claims.sub == session.user_id` enforced
- **Environment sanitization**: Only safe variables propagated
- **Idle timeout**: 30-minute inactivity disconnection
- **No host shell access**: Sandboxed process execution only

### Supply Chain
- **Cargo audit**: Automated vulnerability scanning in CI
- **pnpm audit**: NPM dependency vulnerability scanning
- **Pinned action SHAs**: GitHub Actions use versioned tags
- **SBOM generation**: SPDX JSON Software Bill of Materials for container images
