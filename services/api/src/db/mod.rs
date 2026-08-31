use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::{info, warn};

pub mod labs;
pub mod progress;
pub mod users;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("Connecting to PostgreSQL database...");
        let pool = PgPoolOptions::new()
            .max_connections(25)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .connect(database_url)
            .await?;

        info!("PostgreSQL connected successfully. Running migrations...");
        // Automatically execute DDL migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| {
                warn!("Migration warning: {:?}", e);
                sqlx::Error::Migrate(Box::new(e))
            })?;

        info!("PostgreSQL schema migrations verified.");
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn users(&self) -> users::UserRepository<'_> {
        users::UserRepository::new(&self.pool)
    }

    pub fn progress(&self) -> progress::ProgressRepository<'_> {
        progress::ProgressRepository::new(&self.pool)
    }

    pub fn labs(&self) -> labs::LabRepository<'_> {
        labs::LabRepository::new(&self.pool)
    }

    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
