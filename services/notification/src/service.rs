use crate::models::{Notification, NotificationSeverity};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct NotificationService {
    notifications: Arc<RwLock<HashMap<String, Vec<Notification>>>>,
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn dispatch(
        &self,
        user_id: &str,
        title: &str,
        message: &str,
        severity: NotificationSeverity,
    ) -> Notification {
        let notification = Notification {
            id: Uuid::new_v4(),
            user_id: user_id.to_string(),
            title: title.to_string(),
            message: message.to_string(),
            severity,
            read: false,
            created_at: Utc::now(),
        };

        let mut map = self.notifications.write().await;
        map.entry(user_id.to_string())
            .or_default()
            .push(notification.clone());

        notification
    }

    pub async fn get_user_notifications(&self, user_id: &str) -> Vec<Notification> {
        let map = self.notifications.read().await;
        map.get(user_id).cloned().unwrap_or_default()
    }
}
