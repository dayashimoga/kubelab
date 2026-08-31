use kubelab_validation_engine::rules::mesh::IstioMeshValidator;

#[test]
fn test_istio_service_mesh_manifest_validation() {
    let vs_yaml = include_str!("../../../infrastructure/mesh/istio/virtualservice-canary.yaml");
    let dr_yaml = include_str!("../../../infrastructure/mesh/istio/destinationrule-mtls.yaml");

    let result = IstioMeshValidator::validate_service_mesh(vs_yaml, dr_yaml);

    assert!(result.is_valid, "Service mesh manifests must be valid");
    assert!(result.has_canary_routing, "Must contain canary traffic routing");
    assert!(result.has_mtls_strict, "Must contain STRICT mTLS");
    assert!(result.has_circuit_breaker, "Must contain circuit breaker / outlier detection");
    assert!(result.has_retries_configured, "Must contain automatic retry policy");
}
