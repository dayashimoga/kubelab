# Security Architecture & Threat Mitigation

## Layered Defense
- **Authentication**: JWT HS256 + Argon2id
- **Authorization**: RBAC (Learner, Instructor, Admin)
- **Admission Control**: Rejection of privileged, hostPath, hostNetwork, runtime sockets
- **Network Isolation**: Namespace-scoped NetworkPolicies, IMDS blocking
- **Process Isolation**: Dropped Linux capabilities, non-root execution
