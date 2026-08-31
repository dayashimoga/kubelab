use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AdmissionError {
    #[error("Privileged container execution is strictly forbidden in sandbox environments")]
    PrivilegedContainerForbidden,

    #[error("Host namespace sharing (hostNetwork, hostPID, hostIPC) is prohibited")]
    HostNamespaceForbidden,

    #[error("hostPath volume mounts are prohibited; use emptyDir or PersistentVolumeClaim")]
    HostPathMountForbidden,

    #[error("Container runtime socket mounts (docker/containerd/crio/podman) are prohibited")]
    RuntimeSocketMountForbidden,

    #[error("Elevated Linux capabilities (SYS_ADMIN, NET_ADMIN, PTRACE) are prohibited")]
    ElevatedCapabilitiesForbidden,

    #[error("Targeting system namespaces (kube-system, default, cert-manager) is prohibited")]
    SystemNamespaceForbidden,

    #[error(
        "Cluster-scoped privilege escalation (cluster-admin ClusterRoleBinding) is prohibited"
    )]
    ClusterAdminPrivilegeEscalation,

    #[error("Empty manifest provided")]
    EmptyManifest,
}

pub fn validate_manifest_admission(yaml_content: &str) -> Result<(), AdmissionError> {
    let trimmed = yaml_content.trim();
    if trimmed.is_empty() {
        return Err(AdmissionError::EmptyManifest);
    }

    // 1. Check for privileged container execution
    if regex::Regex::new(r"(?i)privileged:\s*true")
        .unwrap()
        .is_match(trimmed)
    {
        return Err(AdmissionError::PrivilegedContainerForbidden);
    }

    // 2. Check for host namespace sharing
    if regex::Regex::new(r"(?i)(hostNetwork|hostPID|hostIPC):\s*true")
        .unwrap()
        .is_match(trimmed)
    {
        return Err(AdmissionError::HostNamespaceForbidden);
    }

    // 3. Check for hostPath volume mounts
    if regex::Regex::new(r"(?i)hostPath:")
        .unwrap()
        .is_match(trimmed)
    {
        return Err(AdmissionError::HostPathMountForbidden);
    }

    // 4. Check for runtime socket mounts
    if regex::Regex::new(r"(?i)(/var/run/docker\.sock|/run/containerd/|/var/run/crio|/run/podman/)")
        .unwrap()
        .is_match(trimmed)
    {
        return Err(AdmissionError::RuntimeSocketMountForbidden);
    }

    // 5. Check for elevated Linux capabilities
    if regex::Regex::new(r"(?i)(SYS_ADMIN|NET_ADMIN|SYS_PTRACE|SYS_RAWIO|DAC_OVERRIDE)")
        .unwrap()
        .is_match(trimmed)
    {
        return Err(AdmissionError::ElevatedCapabilitiesForbidden);
    }

    // 6. Check for targeting restricted/system namespaces
    if regex::Regex::new(r"(?i)namespace:\s*(kube-system|default|kube-public|kube-node-lease|cert-manager|kubelab-system)").unwrap().is_match(trimmed) {
        return Err(AdmissionError::SystemNamespaceForbidden);
    }

    // 7. Check for cluster-admin privilege escalation
    if regex::Regex::new(r"(?i)kind:\s*ClusterRoleBinding")
        .unwrap()
        .is_match(trimmed)
        && regex::Regex::new(r"(?i)name:\s*cluster-admin")
            .unwrap()
            .is_match(trimmed)
    {
        return Err(AdmissionError::ClusterAdminPrivilegeEscalation);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_manifest_passes_admission() {
        let manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: nginx-secure
spec:
  containers:
  - name: web
    image: nginx:alpine
    ports:
    - containerPort: 80
"#;
        assert!(validate_manifest_admission(manifest).is_ok());
    }

    #[test]
    fn test_privileged_pod_rejected() {
        let manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: privileged-pod
spec:
  containers:
  - name: root-shell
    image: alpine
    securityContext:
      privileged: true
"#;
        assert_eq!(
            validate_manifest_admission(manifest),
            Err(AdmissionError::PrivilegedContainerForbidden)
        );
    }

    #[test]
    fn test_hostpath_rejected() {
        let manifest = r#"
apiVersion: v1
kind: Pod
metadata:
  name: hostpath-pod
spec:
  volumes:
  - name: host-root
    hostPath:
      path: /
"#;
        assert_eq!(
            validate_manifest_admission(manifest),
            Err(AdmissionError::HostPathMountForbidden)
        );
    }

    #[test]
    fn test_system_namespace_override_rejected() {
        let manifest = r#"
apiVersion: v1
kind: Secret
metadata:
  name: stolen-secret
  namespace: kube-system
data:
  token: c3VwZXJzZWNyZXQ=
"#;
        assert_eq!(
            validate_manifest_admission(manifest),
            Err(AdmissionError::SystemNamespaceForbidden)
        );
    }
}
