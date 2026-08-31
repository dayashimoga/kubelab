use crate::catalog::get_default_lab_catalog;
use crate::models::{LabSession, LabSummary, SessionStatus, StartLabRequest, ValidateLabRequest};
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
}

pub struct LabService {
    labs: Arc<RwLock<HashMap<String, DeclarativeLabDef>>>,
    sessions: Arc<RwLock<HashMap<Uuid, LabSession>>>,
}

impl Default for LabService {
    fn default() -> Self {
        Self::new()
    }
}

impl LabService {
    pub fn new() -> Self {
        let catalog = get_default_lab_catalog();
        let mut map = HashMap::new();
        for lab in catalog {
            map.insert(lab.id.clone(), lab);
        }

        Self {
            labs: Arc::new(RwLock::new(map)),
            sessions: Arc::new(RwLock::new(HashMap::new())),
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

        Ok(session)
    }

    pub async fn get_session(&self, session_id: &Uuid) -> Result<LabSession, LabError> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned().ok_or(LabError::SessionNotFound)
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

        // If live_state is supplied in request, use it; otherwise evaluate against simulated running state
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
