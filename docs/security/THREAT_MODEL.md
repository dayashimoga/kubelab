# KubeLab Zero-Trust Threat Model & Security Posture

## 1. Threat Landscape & Boundary Architecture
KubeLab executes untrusted user-supplied code, Kubernetes YAML definitions, and interactive terminal commands. A rigorous multi-layer defense-in-depth model is enforced across all trust boundaries.

---

## 2. Attack Vectors & Remediations Matrix

| Threat Vector | Potential Impact | Security Control & Enforcement | Verification Test |
|---|---|---|---|
| **Container Escape via Privileged Pod** | Host takeover, kernel compromise | Server-side manifest admission rejects `privileged: true` | [`manifest_admission_test.rs`](file:///h:/kubelab/services/api/tests/manifest_admission_test.rs) |
| **Host Filesystem Access via hostPath** | Host disk tampering | Server-side admission rejects `hostPath:` | [`manifest_admission_test.rs`](file:///h:/kubelab/services/api/tests/manifest_admission_test.rs) |
| **Container Runtime Hijack** | Root daemon compromise | Rejects mounts to `/var/run/docker.sock`, `/run/containerd/`, etc. | [`manifest_admission_test.rs`](file:///h:/kubelab/services/api/tests/manifest_admission_test.rs) |
| **Cross-Tenant Network Probing** | Lateral movement between sandboxes | Default-deny `NetworkPolicy` isolates tenant namespaces | [`namespace_provisioner.rs`](file:///h:/kubelab/services/lab-orchestrator/src/k8s/namespace_provisioner.rs) |
| **Cloud Metadata IMDS Exfiltration** | Cloud IAM credential theft | `NetworkPolicy` explicitly drops `169.254.169.254/32` | [`namespace_provisioner.rs`](file:///h:/kubelab/services/lab-orchestrator/src/k8s/namespace_provisioner.rs) |
| **Cross-User Session Hijacking** | Unauthorized terminal execution | Session ownership validation (`claims.sub == session.user_id`) | [`terminal_isolation_test.rs`](file:///h:/kubelab/services/api/tests/terminal_isolation_test.rs) |
| **Denial of Service / Resource Exhaustion** | Cluster starvation | `ResourceQuota` (2 CPU/2Gi RAM) + `LimitRange` container caps | [`namespace_provisioner.rs`](file:///h:/kubelab/services/lab-orchestrator/src/k8s/namespace_provisioner.rs) |
| **Brute-Force Credential Attack** | Account compromise | Per-IP burst rate limiter on auth endpoints | [`rate_limit_auth_test.rs`](file:///h:/kubelab/services/api/tests/rate_limit_auth_test.rs) |
| **Tampered JWT / Token Replay** | Session spoofing | Cryptographic HMAC-SHA256 signature + Redis revocation registry | [`jwt_edge_cases_test.rs`](file:///h:/kubelab/services/auth/tests/jwt_edge_cases_test.rs) |

---

## 3. Pod Security Standards (PSS) Profile
Every sandbox namespace is labeled with:
- `pod-security.kubernetes.io/enforce: restricted`
- `pod-security.kubernetes.io/enforce-version: latest`
- `pod-security.kubernetes.io/audit: restricted`
- `pod-security.kubernetes.io/warn: restricted`
