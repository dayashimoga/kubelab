use redis::aio::ConnectionManager;
use redis::Client;
use tracing::info;

pub mod session_store;

#[derive(Clone)]
pub struct Cache {
    manager: ConnectionManager,
}

impl Cache {
    pub async fn connect(redis_url: &str) -> Result<Self, redis::RedisError> {
        info!("Connecting to Redis at {}...", redis_url);
        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;
        info!("Redis ConnectionManager established successfully.");
        Ok(Self { manager })
    }

    pub fn manager(&self) -> ConnectionManager {
        self.manager.clone()
    }
}
