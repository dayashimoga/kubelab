use kubelab_api::rate_limiter::RateLimiter;
use std::time::Duration;

#[tokio::test]
async fn test_in_memory_rate_limiter_burst_and_recovery() {
    // 5 requests allowed per second, window 1 second
    let limiter = RateLimiter::new(5, 1);
    let client_ip = "192.168.1.100";

    // 1. Initial 5 requests must all be allowed
    for i in 0..5 {
        assert!(
            limiter.check_rate_limit(client_ip).await,
            "Request {} within burst limit should be permitted",
            i + 1
        );
    }

    // 2. 6th immediate request must be REJECTED (rate limit exceeded)
    assert!(
        !limiter.check_rate_limit(client_ip).await,
        "Request exceeding burst capacity should be rejected"
    );

    // 3. Different IP should NOT be blocked (IP isolation)
    let other_ip = "192.168.1.200";
    assert!(
        limiter.check_rate_limit(other_ip).await,
        "Independent IP should have its own rate limit bucket"
    );

    // 4. After wait duration, bucket should replenish and allow requests again
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(
        limiter.check_rate_limit(client_ip).await,
        "Request after replenish window should be allowed"
    );
}
