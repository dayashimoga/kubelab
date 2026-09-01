use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Barrier;
use tower::ServiceExt;

#[tokio::test]
async fn test_concurrency_load_and_latency_measurement() {
    let state = AppState::new("test-secret-key-32-characters-long!".to_string());
    let app = Arc::new(create_routes(state));

    let total_requests = 500;
    let concurrency = 25;
    let requests_per_worker = total_requests / concurrency;

    let barrier = Arc::new(Barrier::new(concurrency));
    let mut handles = Vec::new();

    let benchmark_start = Instant::now();

    for _ in 0..concurrency {
        let app_clone = app.clone();
        let barrier_clone = barrier.clone();

        handles.push(tokio::spawn(async move {
            barrier_clone.wait().await;
            let mut worker_latencies = Vec::with_capacity(requests_per_worker);
            let mut worker_success = 0;

            for _ in 0..requests_per_worker {
                let start = Instant::now();
                let response = (*app_clone)
                    .clone()
                    .oneshot(
                        Request::builder()
                            .uri("/healthz")
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                let elapsed = start.elapsed();
                let status = response.status();
                let _ = response.into_body().collect().await;

                if status == StatusCode::OK {
                    worker_success += 1;
                }
                worker_latencies.push(elapsed.as_micros() as f64 / 1000.0); // ms
            }

            (worker_success, worker_latencies)
        }));
    }

    let mut all_latencies = Vec::with_capacity(total_requests);
    let mut total_success = 0;

    for handle in handles {
        let (success, latencies) = handle.await.unwrap();
        total_success += success;
        all_latencies.extend(latencies);
    }

    let total_duration = benchmark_start.elapsed();
    all_latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let total_duration_secs = total_duration.as_secs_f64();
    let throughput = total_requests as f64 / total_duration_secs;
    let p50 = all_latencies[all_latencies.len() / 2];
    let p95 = all_latencies[(all_latencies.len() as f64 * 0.95) as usize];
    let p99 = all_latencies[(all_latencies.len() as f64 * 0.99) as usize];
    let avg = all_latencies.iter().sum::<f64>() / all_latencies.len() as f64;

    println!("\n=== REAL CONCURRENCY LOAD TEST MEASUREMENT ===");
    println!("Total Requests: {}", total_requests);
    println!("Concurrent Workers: {}", concurrency);
    println!("Total Duration: {:.2}s", total_duration_secs);
    println!("Measured Throughput: {:.2} req/sec", throughput);
    println!("Latency Average: {:.3}ms", avg);
    println!("Latency p50: {:.3}ms", p50);
    println!("Latency p95: {:.3}ms", p95);
    println!("Latency p99: {:.3}ms", p99);
    println!(
        "Success Rate: {:.2}% ({}/{})",
        (total_success as f64 / total_requests as f64) * 100.0,
        total_success,
        total_requests
    );
    println!("==============================================\n");

    assert_eq!(total_success, total_requests, "All requests must succeed");
    assert!(p95 < 100.0, "p95 latency must be under 100ms");
    assert!(throughput > 100.0, "Throughput must exceed 100 req/s");
}
