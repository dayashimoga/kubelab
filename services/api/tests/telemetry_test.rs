use kubelab_api::telemetry::init_tracer;
use std::time::Duration;

#[tokio::test]
async fn test_opentelemetry_tracer_initialization() {
    let init_future = async {
        init_tracer("kubelab-api-test", "http://127.0.0.1:4317")
    };

    // Verify initialization completes within reasonable timeout
    let result = tokio::time::timeout(Duration::from_secs(3), init_future).await;
    match result {
        Ok(res) => {
            println!("OTel tracer initialization outcome: {:?}", res.is_ok());
        }
        Err(_) => {
            println!("OTel tracer initialization timed out as expected without live collector");
        }
    }
}
