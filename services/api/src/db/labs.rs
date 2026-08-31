use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LabSessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lab_id: String,
    pub namespace: String,
    pub status: String,
    pub score: i32,
    pub max_score: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub struct LabRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> LabRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        lab_id: &str,
        namespace: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<LabSessionRow, sqlx::Error> {
        let session = sqlx::query_as::<_, LabSessionRow>(
            r#"
            INSERT INTO lab_sessions (user_id, lab_id, namespace, status, score, max_score, expires_at)
            VALUES ($1, $2, $3, 'running', 0, 100, $4)
            RETURNING id, user_id, lab_id, namespace, status, score, max_score, created_at, expires_at, completed_at
            "#
        )
        .bind(user_id)
        .bind(lab_id)
        .bind(namespace)
        .bind(expires_at)
        .fetch_one(self.pool)
        .await?;

        Ok(session)
    }

    pub async fn find_session(&self, id: Uuid) -> Result<Option<LabSessionRow>, sqlx::Error> {
        sqlx::query_as::<_, LabSessionRow>(
            r#"
            SELECT id, user_id, lab_id, namespace, status, score, max_score, created_at, expires_at, completed_at
            FROM lab_sessions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn update_score(&self, id: Uuid, score: i32, status: &str) -> Result<LabSessionRow, sqlx::Error> {
        let session = sqlx::query_as::<_, LabSessionRow>(
            r#"
            UPDATE lab_sessions
            SET score = $2,
                status = $3,
                completed_at = CASE WHEN $3 = 'completed' THEN NOW() ELSE completed_at END
            WHERE id = $1
            RETURNING id, user_id, lab_id, namespace, status, score, max_score, created_at, expires_at, completed_at
            "#
        )
        .bind(id)
        .bind(score)
        .bind(status)
        .fetch_one(self.pool)
        .await?;

        Ok(session)
    }
}
