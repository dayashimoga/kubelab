use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use tower::ServiceExt;

#[tokio::test]
async fn test_progress_service_lifecycle_and_milestones() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    let user_id = "test-learner-progress-01";

    // 1. Initial state must start at 0 XP, Level 1, 0 streak, 0 completed items
    let initial_progress = state.progress.get_user_progress(user_id).await;
    assert_eq!(initial_progress.total_xp, 0);
    assert_eq!(initial_progress.level, 1);
    assert_eq!(initial_progress.current_streak_days, 0);
    assert!(initial_progress.completed_lesson_ids.is_empty());
    assert!(initial_progress.completed_lab_ids.is_empty());
    assert!(initial_progress.unlocked_badges.is_empty());

    // 2. Complete lesson -> earns XP and increments level when threshold crossed
    let after_lesson = state
        .progress
        .complete_lesson(user_id, "k8s-pod-architecture", 200)
        .await;
    assert_eq!(after_lesson.total_xp, 200);
    assert_eq!(after_lesson.level, 1);
    assert_eq!(
        after_lesson.completed_lesson_ids,
        vec!["k8s-pod-architecture"]
    );

    // 3. Complete first lab -> earns lab XP + first-pod badge bonus (+100 XP)
    let after_lab = state
        .progress
        .complete_lab(user_id, "k8s-pod-basics", 300)
        .await;
    // 200 (lesson) + 300 (lab) + 100 (badge reward) = 600 XP -> Level 2
    assert_eq!(after_lab.total_xp, 600);
    assert_eq!(after_lab.level, 2);
    assert_eq!(after_lab.completed_lab_ids, vec!["k8s-pod-basics"]);
    assert_eq!(after_lab.unlocked_badges.len(), 1);
    assert_eq!(after_lab.unlocked_badges[0].slug, "first-pod");

    // 4. Update skill mastery level
    let after_skill = state
        .progress
        .update_skill(user_id, "skill-k8s-workloads", 3)
        .await;
    assert_eq!(after_skill.skills.get("skill-k8s-workloads"), Some(&3));

    // 5. Query progress via HTTP API
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/progress/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(res_json["total_xp"], 600);
    assert_eq!(res_json["level"], 2);

    // 6. Query skill DAG via HTTP API
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/skills/graph")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let graph_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!graph_json["nodes"].as_array().unwrap().is_empty());
}
