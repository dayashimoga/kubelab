use kube::{
    api::{Api, DynamicObject},
    discovery::{Discovery, Scope},
    Client,
};
use serde_json::Value;
use tracing::{info, warn};

pub async fn fetch_live_k8s_resource(
    client: &Client,
    resource_type: &str,
    name: &str,
    namespace: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Querying live Kubernetes cluster for resource '{}' named '{}' in namespace '{}'...",
        resource_type, name, namespace
    );

    let discovery = Discovery::new(client.clone()).run().await?;

    // Try finding plural/singular matching resource
    let mut resolved_ar = None;
    let mut is_cluster_scoped = false;

    for group in discovery.groups() {
        for (ar, caps) in group.recommended_resources() {
            if ar.plural.eq_ignore_ascii_case(resource_type)
                || ar.kind.eq_ignore_ascii_case(resource_type)
            {
                resolved_ar = Some(ar);
                is_cluster_scoped = caps.scope == Scope::Cluster;
                break;
            }
        }
        if resolved_ar.is_some() {
            break;
        }
    }

    let ar = match resolved_ar {
        Some(ar) => ar,
        None => {
            warn!(
                "Could not discover GVR for resource type '{}'",
                resource_type
            );
            return Ok(Value::Null);
        }
    };

    let api: Api<DynamicObject> = if is_cluster_scoped {
        Api::all_with(client.clone(), &ar)
    } else {
        Api::namespaced_with(client.clone(), namespace, &ar)
    };

    match api.get(name).await {
        Ok(obj) => {
            let json_val = serde_json::to_value(obj)?;
            Ok(json_val)
        }
        Err(e) => {
            warn!(
                "Live Kubernetes get for {}/{} in namespace '{}' failed: {:?}",
                resource_type, name, namespace, e
            );
            Ok(Value::Null)
        }
    }
}
