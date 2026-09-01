use kubelab_progress::service::ProgressService;
use kubelab_progress::skill_graph::generate_cloud_native_skill_graph;

#[tokio::test]
async fn test_progress_service_xp_and_badges() {
    let service = ProgressService::new();
    let user_id = "test-user-001";

    // 1. Initial progress
    let initial = service.get_user_progress(user_id).await;
    assert_eq!(initial.user_id, user_id);
    assert_eq!(initial.total_xp, 0);
    assert_eq!(initial.level, 1);

    // 2. Award XP
    let updated = service.add_xp(user_id, 1250).await;
    assert_eq!(updated.total_xp, 1250);
    assert!(updated.level >= 2, "Level should advance with 1250 XP");

    // 3. Mark lab completed
    let after_lab = service.complete_lab(user_id, "k8s-pod-basics", 200).await;
    assert!(after_lab
        .completed_lab_ids
        .contains(&"k8s-pod-basics".to_string()));
    assert_eq!(after_lab.total_xp, 1550); // 1250 + 200 lab XP + 100 first-lab badge bonus

    // 4. Verify skill tree nodes
    let skill_tree = generate_cloud_native_skill_graph();
    assert!(
        !skill_tree.nodes.is_empty(),
        "Skill tree DAG should contain competency nodes"
    );
}
