use super::EventBus;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabStartedEvent {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub lab_id: String,
    pub namespace: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabCompletedEvent {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub lab_id: String,
    pub score: i32,
    pub xp_earned: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdatedEvent {
    pub user_id: Uuid,
    pub new_total_xp: i32,
    pub new_level: i32,
    pub streak_days: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlertEvent {
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

pub struct DomainEventPublisher<'a> {
    bus: &'a EventBus,
}

impl<'a> DomainEventPublisher<'a> {
    pub fn new(bus: &'a EventBus) -> Self {
        Self { bus }
    }

    pub async fn emit_lab_started(&self, event: &LabStartedEvent) -> Result<(), async_nats::Error> {
        self.bus.publish_event("events.lab.started", event).await
    }

    pub async fn emit_lab_completed(&self, event: &LabCompletedEvent) -> Result<(), async_nats::Error> {
        self.bus.publish_event("events.lab.completed", event).await
    }

    pub async fn emit_progress_updated(&self, event: &ProgressUpdatedEvent) -> Result<(), async_nats::Error> {
        self.bus.publish_event("events.progress.updated", event).await
    }

    pub async fn emit_security_alert(&self, event: &SecurityAlertEvent) -> Result<(), async_nats::Error> {
        self.bus.publish_event("events.security.alert", event).await
    }
}
