use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatusCode {
    Synced,
    OutOfSync,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatusCode {
    Healthy,
    Progressing,
    Degraded,
    Suspended,
    Missing,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOpsApplicationStatus {
    pub name: String,
    pub project: String,
    pub repo_url: String,
    pub target_revision: String,
    pub sync_status: SyncStatusCode,
    pub health_status: HealthStatusCode,
    pub resources_synced: usize,
    pub resources_total: usize,
    pub drift_detected: bool,
}

pub struct GitOpsEvaluator;

impl GitOpsEvaluator {
    /// Parse raw Argo CD Application JSON/YAML status
    pub fn parse_status_json(
        raw_json: &serde_json::Value,
    ) -> Result<GitOpsApplicationStatus, String> {
        let name = raw_json["metadata"]["name"]
            .as_str()
            .unwrap_or("unnamed")
            .to_string();

        let project = raw_json["spec"]["project"]
            .as_str()
            .unwrap_or("default")
            .to_string();

        let repo_url = raw_json["spec"]["source"]["repoURL"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let target_revision = raw_json["spec"]["source"]["targetRevision"]
            .as_str()
            .unwrap_or("HEAD")
            .to_string();

        let sync_str = raw_json["status"]["sync"]["status"]
            .as_str()
            .unwrap_or("Unknown");

        let sync_status = match sync_str {
            "Synced" => SyncStatusCode::Synced,
            "OutOfSync" => SyncStatusCode::OutOfSync,
            _ => SyncStatusCode::Unknown,
        };

        let health_str = raw_json["status"]["health"]["status"]
            .as_str()
            .unwrap_or("Unknown");

        let health_status = match health_str {
            "Healthy" => HealthStatusCode::Healthy,
            "Progressing" => HealthStatusCode::Progressing,
            "Degraded" => HealthStatusCode::Degraded,
            "Suspended" => HealthStatusCode::Suspended,
            "Missing" => HealthStatusCode::Missing,
            _ => HealthStatusCode::Unknown,
        };

        let resources = raw_json["status"]["resources"].as_array();
        let resources_total = resources.map(|r| r.len()).unwrap_or(0);
        let resources_synced = if sync_status == SyncStatusCode::Synced {
            resources_total
        } else {
            resources
                .map(|r| {
                    r.iter()
                        .filter(|item| item["status"].as_str() == Some("Synced"))
                        .count()
                })
                .unwrap_or(0)
        };

        let drift_detected = sync_status == SyncStatusCode::OutOfSync;

        Ok(GitOpsApplicationStatus {
            name,
            project,
            repo_url,
            target_revision,
            sync_status,
            health_status,
            resources_synced,
            resources_total,
            drift_detected,
        })
    }
}
