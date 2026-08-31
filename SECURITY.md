# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of KubeLab seriously. If you discover a security vulnerability, please follow responsible disclosure:

1. **Do not disclose publicly.** Please do not open public GitHub issues or discussions regarding potential vulnerabilities.
2. **Email the security team.** Send details to `security@kubelab.io` with:
   - Type of vulnerability (e.g. container breakout, privilege escalation, SSRF, auth bypass)
   - Step-by-step reproduction instructions or Proof of Concept (PoC)
   - Potential impact
3. **Response timeline:**
   - Initial acknowledgment: Within 24 hours
   - Status update & validation: Within 72 hours
   - Patch release: Within 7 days for critical severity

## Security Architecture Highlights

- **Lab Sandboxing:** Learner terminal shells execute as non-root users (`uid 10001`) with dropped Linux capabilities (`CAP_DROP ALL`).
- **Resource Isolation:** Kubernetes NetworkPolicies deny inter-namespace traffic between tenant labs.
- **Seccomp & Storage:** Read-only root filesystems on learner containers with limited tmpfs mounts.
- **API Guarding:** Strict JWT cryptographic signature verification, rate-limiting per IP/user, and parameter validation.
