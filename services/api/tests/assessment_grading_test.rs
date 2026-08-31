use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_assessment_quiz_fetching_and_deterministic_grading() {
    let state = AppState::new("secret-key-32-chars-minimum-length!".to_string());
    let app = create_routes(state.clone());

    // 1. Fetch questions for lesson 'k8s-pod-architecture'
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/lessons/k8s-pod-architecture/quiz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let questions: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let q_array = questions.as_array().unwrap();
    assert_eq!(q_array.len(), 1);
    assert_eq!(q_array[0]["id"], "q-k8s-pod-pause");
    // Verify answers are NOT leaked in public question payload
    assert!(q_array[0].get("correct_answer").is_none());

    // 2. Submit correct answer ("opt-b")
    let correct_submission = json!({
        "quiz_id": "quiz-k8s-pods",
        "answers": {
            "q-k8s-pod-pause": "opt-b"
        }
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/quizzes/submit")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&correct_submission).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(result["passed"], true);
    assert_eq!(result["score"], 50);
    assert_eq!(result["percentage"], 100.0);
    assert_eq!(result["xp_earned"], 100); // 50 * 2 for passing

    // 3. Submit wrong answer ("opt-a")
    let wrong_submission = json!({
        "quiz_id": "quiz-k8s-pods",
        "answers": {
            "q-k8s-pod-pause": "opt-a"
        }
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/quizzes/submit")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&wrong_submission).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(result["passed"], false);
    assert_eq!(result["score"], 0);
    assert_eq!(result["percentage"], 0.0);
    assert_eq!(result["xp_earned"], 10); // 10 XP participation for failing
}
