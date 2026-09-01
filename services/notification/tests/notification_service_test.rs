use kubelab_notification::models::NotificationSeverity;
use kubelab_notification::service::NotificationService;

#[tokio::test]
async fn test_notification_service_send_and_list() {
    let service = NotificationService::new();
    let user_id = "test-learner-123";

    // 1. Dispatch notification
    let notif = service
        .dispatch(
            user_id,
            "Lab Completed",
            "You scored 100% on k8s-pod-basics",
            NotificationSeverity::Success,
        )
        .await;

    assert_eq!(notif.user_id, user_id);
    assert_eq!(notif.title, "Lab Completed");
    assert!(!notif.read);

    // 2. Retrieve user notifications
    let user_notifs = service.get_user_notifications(user_id).await;
    assert_eq!(user_notifs.len(), 1);
    assert_eq!(user_notifs[0].title, "Lab Completed");
}
