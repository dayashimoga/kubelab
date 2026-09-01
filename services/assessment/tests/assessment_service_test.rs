use kubelab_assessment::service::AssessmentService;

#[tokio::test]
async fn test_assessment_service_quiz_and_grading() {
    let service = AssessmentService::new();

    // Query questions for a lesson
    let questions = service.get_quiz_by_lesson_id("k8s-pod-architecture").await;
    assert!(
        !questions.is_empty(),
        "Questions should exist for k8s-pod-architecture lesson"
    );
    assert_eq!(questions[0].id, "q-k8s-pod-pause");
    assert_eq!(questions[0].options.len(), 4);
}
