use crate::catalog::load_labs_from_disk;
use crate::models::{
    AppliedResource, ApplyManifestRequest, ApplyManifestResponse, K8sResourceSummary, LabSession,
    LabSummary, SessionStatus, StartLabRequest, ValidateLabRequest,
};
use chrono::{Duration, Utc};
use kubelab_validation_engine::evaluator::LabEvaluator;
use kubelab_validation_engine::models::{DeclarativeLabDef, ValidationResult};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum LabError {
    #[error("Lab definition not found")]
    LabNotFound,
    #[error("Active session not found")]
    SessionNotFound,
    #[error("Task not found in lab")]
    TaskNotFound,
    #[error("Manifest parse error: {0}")]
    ManifestError(String),
}

pub struct LabService {
    labs: Arc<RwLock<HashMap<String, DeclarativeLabDef>>>,
    sessions: Arc<RwLock<HashMap<Uuid, LabSession>>>,
    session_resources: Arc<RwLock<HashMap<Uuid, Vec<K8sResourceSummary>>>>,
}

impl Default for LabService {
    fn default() -> Self {
        Self::new()
    }
}

impl LabService {
    pub fn new() -> Self {
        let catalog = load_labs_from_disk();
        let mut map = HashMap::new();
        for lab in catalog {
            map.insert(lab.id.clone(), lab);
        }

        Self {
            labs: Arc::new(RwLock::new(map)),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn list_labs(&self) -> Vec<LabSummary> {
        let labs = self.labs.read().await;
        labs.values()
            .map(|l| LabSummary {
                id: l.id.clone(),
                title: l.title.clone(),
                difficulty: l.difficulty.clone(),
                duration_minutes: l.duration_minutes,
                track: l.track.clone(),
                task_count: l.tasks.len(),
                total_points: l.tasks.iter().map(|t| t.points).sum(),
            })
            .collect()
    }

    pub async fn get_lab(&self, lab_id: &str) -> Result<DeclarativeLabDef, LabError> {
        let labs = self.labs.read().await;
        labs.get(lab_id).cloned().ok_or(LabError::LabNotFound)
    }

    pub async fn start_session(&self, user_id: &str, req: StartLabRequest) -> Result<LabSession, LabError> {
        let lab = self.get_lab(&req.lab_id).await?;
        let now = Utc::now();
        let expires_at = now + Duration::minutes(lab.duration_minutes as i64);
        let session_id = Uuid::new_v4();

        let session = LabSession {
            id: session_id,
            lab_id: lab.id.clone(),
            user_id: user_id.to_string(),
            status: SessionStatus::Ready,
            namespace: format!("lab-{}", session_id.simple()),
            cluster_endpoint: "https://k8s-local.kubelab.internal:6443".to_string(),
            started_at: now,
            expires_at,
            completed_at: None,
            score: 0,
            max_score: lab.tasks.iter().map(|t| t.points).sum(),
            task_results: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session.clone());

        // Initialize default resources in namespace
        let mut resources_map = self.session_resources.write().await;
        resources_map.insert(session_id, Vec::new());

        Ok(session)
    }

    pub async fn get_session(&self, session_id: &Uuid) -> Result<LabSession, LabError> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned().ok_or(LabError::SessionNotFound)
    }

    pub async fn apply_manifest(
        &self,
        session_id: &Uuid,
        req: ApplyManifestRequest,
    ) -> Result<ApplyManifestResponse, LabError> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id).ok_or(LabError::SessionNotFound)?;

        let mut applied = Vec::new();
        let mut new_resources = Vec::new();

        // Parse YAML documents in manifest
        for doc in req.yaml_content.split("---") {
            let doc_trimmed = doc.trim();
            if doc_trimmed.is_empty() {
                continue;
            }

            if let Ok(parsed) = serde_yaml::from_str::<serde_json::Value>(doc_trimmed) {
                let kind = parsed["kind"].as_str().unwrap_or("Resource").to_string();
                let name = parsed["metadata"]["name"]
                    .as_str()
                    .unwrap_or("unnamed")
                    .to_string();

                applied.push(AppliedResource {
                    kind: kind.clone(),
                    name: name.clone(),
                    namespace: session.namespace.clone(),
                    status: "Configured".to_string(),
                    action: "created".to_string(),
                });

                new_resources.push(K8sResourceSummary {
                    kind,
                    name,
                    namespace: session.namespace.clone(),
                    status: "Running".to_string(),
                    age: "10s".to_string(),
                    details: "Ready (1/1)".to_string(),
                });
            }
        }

        let mut res_map = self.session_resources.write().await;
        let list = res_map.entry(*session_id).or_default();
        list.extend(new_resources);

        Ok(ApplyManifestResponse {
            success: true,
            applied_resources: applied,
            message: "Applied 1 or more documents to sandbox namespace".to_string(),
        })
    }

    pub async fn get_namespace_resources(
        &self,
        session_id: &Uuid,
    ) -> Result<Vec<K8sResourceSummary>, LabError> {
        let _ = self.get_session(session_id).await?;
        let map = self.session_resources.read().await;
        Ok(map.get(session_id).cloned().unwrap_or_default())
    }

    pub async fn destroy_session(&self, session_id: &Uuid) -> Result<(), LabError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = SessionStatus::Destroyed;
        } else {
            return Err(LabError::SessionNotFound);
        }

        let mut res_map = self.session_resources.write().await;
        res_map.remove(session_id);

        Ok(())
    }

    pub async fn validate_task(
        &self,
        session_id: &Uuid,
        req: ValidateLabRequest,
    ) -> Result<ValidationResult, LabError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions.get_mut(session_id).ok_or(LabError::SessionNotFound)?;

        let lab = self.get_lab(&session.lab_id).await?;
        let task = lab
            .tasks
            .iter()
            .find(|t| t.id == req.task_id)
            .ok_or(LabError::TaskNotFound)?;

        let actual_state = req.live_state.unwrap_or_else(|| {
            json!({
                "status": { "phase": "Running", "readyReplicas": 3 },
                "metadata": { "labels": { "app": "frontend" } },
                "spec": { "containers": [{ "ports": [{ "containerPort": 80 }] }] }
            })
        });

        let result = LabEvaluator::evaluate_task(task, &actual_state);

        session.task_results.insert(req.task_id.clone(), result.passed);
        if result.passed {
            session.score = session
                .task_results
                .iter()
                .filter(|(_, &passed)| passed)
                .count() as u32
                * task.points;
            if session.score >= session.max_score {
                session.status = SessionStatus::Completed;
                session.completed_at = Some(Utc::now());
            }
        }

        Ok(result)
    }
}
