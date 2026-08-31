use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    clients: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
        }
    }

    pub async fn check_rate_limit(&self, client_key: &str) -> bool {
        let mut clients = self.clients.lock().await;
        let now = Instant::now();

        if let Some((count, last_reset)) = clients.get_mut(client_key) {
            if now.duration_since(*last_reset).as_secs() >= self.window_secs {
                *count = 1;
                *last_reset = now;
                true
            } else if *count < self.max_requests {
                *count += 1;
                true
            } else {
                false
            }
        } else {
            clients.insert(client_key.to_string(), (1, now));
            true
        }
    }
}
