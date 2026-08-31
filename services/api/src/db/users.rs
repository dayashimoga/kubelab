use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub role: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        email: &str,
        name: &str,
        password_hash: &str,
        role: &str,
    ) -> Result<UserRow, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (email, name, password_hash, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, name, password_hash, role, avatar_url, created_at, updated_at
            "#
        )
        .bind(email)
        .bind(name)
        .bind(password_hash)
        .bind(role)
        .fetch_one(self.pool)
        .await?;

        // Initialize user progress row
        sqlx::query(
            r#"
            INSERT INTO user_progress (user_id, total_xp, level, current_streak_days, longest_streak_days)
            VALUES ($1, 0, 1, 0, 0)
            ON CONFLICT (user_id) DO NOTHING
            "#
        )
        .bind(user.id)
        .execute(self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, email, name, password_hash, role, avatar_url, created_at, updated_at
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, email, name, password_hash, role, avatar_url, created_at, updated_at
            FROM users
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
    }
}
