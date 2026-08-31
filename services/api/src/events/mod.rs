use async_nats::Client;
use tracing::info;

pub mod publisher;

#[derive(Clone)]
pub struct EventBus {
    client: Client,
}

impl EventBus {
    pub async fn connect(nats_url: &str) -> Result<Self, async_nats::Error> {
        info!("Connecting to NATS at {}...", nats_url);
        let client = async_nats::connect(nats_url).await?;
        info!("NATS connection established successfully.");
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn publish_event<T: serde::Serialize>(&self, subject: &str, event: &T) -> Result<(), async_nats::Error> {
        let payload = serde_json::to_vec(event)?;
        self.client.publish(subject.to_string(), payload.into()).await?;
        Ok(())
    }
}
