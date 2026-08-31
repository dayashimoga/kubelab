use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Provisioning,
    Ready,
    Running,
    Completed,
    Failed,
    Expired,
    Destroyed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabSession {
    pub id: Uuid,
    pub lab_id: String,
    pub user_id: String,
    pub status: SessionStatus,
    pub namespace: String,
    pub cluster_endpoint: String,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub score: u32,
    pub max_score: u32,
    pub task_results: std::collections::HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartLabRequest {
    pub lab_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateLabRequest {
    pub task_id: String,
    pub live_state: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabSummary {
    pub id: String,
    pub title: String,
    pub difficulty: String,
    pub duration_minutes: u32,
    pub track: String,
    pub task_count: usize,
    pub total_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyManifestRequest {
    pub yaml_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedResource {
    pub kind: String,
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub action: String, // "created", "configured", "unchanged"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyManifestResponse {
    pub success: bool,
    pub applied_resources: Vec<AppliedResource>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sResourceSummary {
    pub kind: String,
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub age: String,
    pub details: String,
}
