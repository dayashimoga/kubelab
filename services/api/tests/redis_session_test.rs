use kubelab_api::cache::Cache;
use kubelab_api::cache::session_store::{SessionStore, CachedSession};
use uuid::Uuid;

#[tokio::test]
#[ignore = "Requires live Redis — run with: cargo test -- --ignored"]
async fn test_redis_session_cache_and_revocation() {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let cache = Cache::connect(&redis_url).await
        .expect("Redis must be reachable for this integration test");

    let mut store = SessionStore::new(cache.manager());
    let token_id = format!("jwt_{}", Uuid::new_v4());
    let user_id = Uuid::new_v4();

    let session = CachedSession {
        user_id,
        email: "alice@kubelab.io".to_string(),
        role: "learner".to_string(),
        session_id: token_id.clone(),
    };

    // 1. Cache session for 60s
    store
        .set_session(&token_id, &session, 60)
        .await
        .expect("Set session in Redis must succeed");

    // 2. Fetch session
    let retrieved = store
        .get_session(&token_id)
        .await
        .expect("Get session must succeed")
        .expect("Session must exist");
    assert_eq!(retrieved.user_id, user_id);
    assert_eq!(retrieved.email, "alice@kubelab.io");

    // 3. Verify not yet revoked
    assert!(!store.is_revoked(&token_id).await.expect("Check revoked must succeed"));

    // 4. Revoke token
    store
        .revoke_token(&token_id, 3600)
        .await
        .expect("Revoke token must succeed");

    // 5. Verify now revoked
    assert!(store.is_revoked(&token_id).await.expect("Check revoked must succeed"));
}
