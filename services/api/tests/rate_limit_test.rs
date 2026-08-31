use kubelab_api::rate_limiter::RateLimiter;

#[tokio::test]
async fn test_rate_limiter_allows_under_threshold_and_blocks_bursts() {
    // 5 requests per 10 seconds limit
    let limiter = RateLimiter::new(5, 10);
    let client_ip = "192.168.1.50";

    // First 5 requests must pass
    for _ in 0..5 {
        assert!(limiter.check_rate_limit(client_ip).await);
    }

    // 6th request must be rejected (rate limit exceeded)
    assert!(!limiter.check_rate_limit(client_ip).await);

    // Another client IP should still be allowed
    let other_client = "192.168.1.99";
    assert!(limiter.check_rate_limit(other_client).await);
}
