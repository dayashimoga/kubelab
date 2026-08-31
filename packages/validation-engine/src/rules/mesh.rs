use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshValidationResult {
    pub is_valid: bool,
    pub has_canary_routing: bool,
    pub has_mtls_strict: bool,
    pub has_circuit_breaker: bool,
    pub has_retries_configured: bool,
    pub details: Vec<String>,
}

pub struct IstioMeshValidator;

impl IstioMeshValidator {
    /// Validates an Istio VirtualService + DestinationRule specification
    pub fn validate_service_mesh(
        virtual_service_yaml: &str,
        destination_rule_yaml: &str,
    ) -> MeshValidationResult {
        let mut details = Vec::new();
        let mut has_canary = false;
        let mut has_retries = false;
        let mut has_mtls = false;
        let mut has_cb = false;

        // Parse VirtualService
        if let Ok(vs) = serde_yaml::from_str::<serde_json::Value>(virtual_service_yaml) {
            if let Some(http) = vs["spec"]["http"].as_array() {
                for route in http {
                    if let Some(routes) = route["route"].as_array() {
                        if routes.len() > 1 {
                            has_canary = true;
                            details.push("Found multi-subset canary traffic weighting".to_string());
                        }
                    }
                    if route["retries"]["attempts"].as_i64().unwrap_or(0) > 0 {
                        has_retries = true;
                        details.push("Found automatic HTTP retry policy".to_string());
                    }
                }
            }
        }

        // Parse DestinationRule
        if let Ok(dr) = serde_yaml::from_str::<serde_json::Value>(destination_rule_yaml) {
            let tls_mode = dr["spec"]["trafficPolicy"]["tls"]["mode"]
                .as_str()
                .unwrap_or("");
            if tls_mode == "ISTIO_MUTUAL" || tls_mode == "MUTUAL" {
                has_mtls = true;
                details.push("Found STRICT mTLS policy".to_string());
            }

            if dr["spec"]["trafficPolicy"]["outlierDetection"]["consecutive5xxErrors"]
                .as_i64()
                .unwrap_or(0)
                > 0
            {
                has_cb = true;
                details.push("Found outlier detection circuit breaker".to_string());
            }
        }

        let is_valid = has_canary && has_mtls;

        MeshValidationResult {
            is_valid,
            has_canary_routing: has_canary,
            has_mtls_strict: has_mtls,
            has_circuit_breaker: has_cb,
            has_retries_configured: has_retries,
            details,
        }
    }
}
