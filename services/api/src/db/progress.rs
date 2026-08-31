use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserProgressRow {
    pub user_id: Uuid,
    pub total_xp: i32,
    pub level: i32,
    pub current_streak_days: i32,
    pub longest_streak_days: i32,
    pub last_active_date: NaiveDate,
    pub completed_lessons: serde_json::Value,
    pub completed_labs: serde_json::Value,
    pub unlocked_badges: serde_json::Value,
    pub skills: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

pub struct ProgressRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ProgressRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_progress(&self, user_id: Uuid) -> Result<Option<UserProgressRow>, sqlx::Error> {
        sqlx::query_as::<_, UserProgressRow>(
            r#"
            SELECT user_id, total_xp, level, current_streak_days, longest_streak_days,
                   last_active_date, completed_lessons, completed_labs, unlocked_badges,
                   skills, updated_at
            FROM user_progress
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn add_xp(&self, user_id: Uuid, xp_to_add: i32) -> Result<UserProgressRow, sqlx::Error> {
        let current = self.get_progress(user_id).await?;
        let current_xp = current.as_ref().map(|p| p.total_xp).unwrap_or(0);
        let new_xp = current_xp + xp_to_add;
        let new_level = (new_xp / 500) + 1;

        let updated = sqlx::query_as::<_, UserProgressRow>(
            r#"
            INSERT INTO user_progress (user_id, total_xp, level, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (user_id) DO UPDATE
            SET total_xp = $2,
                level = $3,
                updated_at = NOW()
            RETURNING user_id, total_xp, level, current_streak_days, longest_streak_days,
                      last_active_date, completed_lessons, completed_labs, unlocked_badges,
                      skills, updated_at
            "#
        )
        .bind(user_id)
        .bind(new_xp)
        .bind(new_level)
        .fetch_one(self.pool)
        .await?;

        Ok(updated)
    }

    pub async fn record_lab_completion(&self, user_id: Uuid, lab_id: &str, xp_reward: i32) -> Result<UserProgressRow, sqlx::Error> {
        let current = self.get_progress(user_id).await?;
        let mut completed: Vec<String> = current
            .as_ref()
            .and_then(|p| serde_json::from_value(p.completed_labs.clone()).ok())
            .unwrap_or_default();

        if !completed.contains(&lab_id.to_string()) {
            completed.push(lab_id.to_string());
        }

        let completed_json = serde_json::to_value(completed).unwrap_or_default();
        let current_xp = current.as_ref().map(|p| p.total_xp).unwrap_or(0);
        let new_xp = current_xp + xp_reward;
        let new_level = (new_xp / 500) + 1;

        let updated = sqlx::query_as::<_, UserProgressRow>(
            r#"
            UPDATE user_progress
            SET completed_labs = $2,
                total_xp = $3,
                level = $4,
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING user_id, total_xp, level, current_streak_days, longest_streak_days,
                      last_active_date, completed_lessons, completed_labs, unlocked_badges,
                      skills, updated_at
            "#
        )
        .bind(user_id)
        .bind(completed_json)
        .bind(new_xp)
        .bind(new_level)
        .fetch_one(self.pool)
        .await?;

        Ok(updated)
    }
}
