use chrono::Utc;
use futures::StreamExt;
use kubelab_api::events::publisher::{DomainEventPublisher, LabStartedEvent};
use kubelab_api::events::EventBus;
use uuid::Uuid;

#[tokio::test]
#[ignore = "Requires live NATS — run with: cargo test -- --ignored"]
async fn test_nats_domain_events_pub_sub() {
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());

    let bus = EventBus::connect(&nats_url)
        .await
        .expect("NATS must be reachable for this integration test");

    // Subscribe to events.lab.started
    let mut subscriber = bus
        .client()
        .subscribe("events.lab.started".to_string())
        .await
        .expect("Subscribe must succeed");

    let publisher = DomainEventPublisher::new(&bus);
    let session_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let event = LabStartedEvent {
        session_id,
        user_id,
        lab_id: "k8s-deployments-scaling".to_string(),
        namespace: "sandbox-test".to_string(),
        timestamp: Utc::now(),
    };

    publisher
        .emit_lab_started(&event)
        .await
        .expect("Emit lab started must succeed");

    // Receive message
    let msg = tokio::time::timeout(std::time::Duration::from_secs(3), subscriber.next())
        .await
        .expect("NATS message must be received within timeout")
        .expect("Message stream should not be closed");

    let received: LabStartedEvent =
        serde_json::from_slice(&msg.payload).expect("Deserialize event payload");
    assert_eq!(received.session_id, session_id);
    assert_eq!(received.lab_id, "k8s-deployments-scaling");
}
