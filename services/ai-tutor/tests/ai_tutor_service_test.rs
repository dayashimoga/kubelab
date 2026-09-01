use kubelab_ai_tutor::models::{TutorMode, TutorRequest};
use kubelab_ai_tutor::service::AiTutorService;

#[tokio::test]
async fn test_ai_tutor_service_prompt_and_hints() {
    let service = AiTutorService::new();

    let request = TutorRequest {
        mode: TutorMode::Explain,
        user_prompt: "How does the pause container work in Kubernetes?".to_string(),
        context_lesson_id: Some("k8s-pod-architecture".to_string()),
        context_lab_id: None,
        current_error_log: None,
        current_yaml: None,
    };

    let response = service.query_tutor(&request).await;
    assert!(!response.reply_markdown.is_empty(), "AI Tutor should provide a non-empty explanation");
    assert!(!response.suggested_followups.is_empty());
}
