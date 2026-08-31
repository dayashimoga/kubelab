use kubelab_validation_engine::evaluator::LabEvaluator;
use kubelab_validation_engine::models::{
    LabTask, StateAssertion, TaskValidation, TaskValidationType, ValidationOperator,
};
use serde_json::json;

#[test]
fn test_evaluator_negative_cases_and_type_mismatches() {
    let task = LabTask {
        id: "test-task-1".to_string(),
        title: "Test Task".to_string(),
        description: "Task description".to_string(),
        points: 50,
        validation: TaskValidation {
            validation_type: TaskValidationType::K8sResource,
            resource: Some("deployment".to_string()),
            name: Some("my-app".to_string()),
            namespace: Some("default".to_string()),
            assertions: vec![
                StateAssertion {
                    field: "status.replicas".to_string(),
                    operator: ValidationOperator::Equals,
                    expected: json!(3),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                StateAssertion {
                    field: "spec.template.spec.containers[0].image".to_string(),
                    operator: ValidationOperator::Equals,
                    expected: json!("nginx:1.25"),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
            ],
        },
    };

    // 1. Completely empty / null state -> FAIL
    let empty_state = json!({});
    let empty_res = LabEvaluator::evaluate_task(&task, &empty_state);
    assert!(!empty_res.passed);
    let failed_count = empty_res
        .assertion_results
        .iter()
        .filter(|a| !a.passed.unwrap_or(false))
        .count();
    assert_eq!(failed_count, 2);

    // 2. Type mismatch: status.replicas is a string "3" instead of number 3 -> FAIL
    let type_mismatch_state = json!({
        "status": { "replicas": "3" },
        "spec": { "template": { "spec": { "containers": [{ "image": "nginx:1.25" }] } } }
    });
    let mismatch_res = LabEvaluator::evaluate_task(&task, &type_mismatch_state);
    assert!(!mismatch_res.passed);
    let failed_mismatch = mismatch_res
        .assertion_results
        .iter()
        .filter(|a| !a.passed.unwrap_or(false))
        .count();
    assert_eq!(failed_mismatch, 1);

    // 3. Image tag mismatch: nginx:1.24 instead of nginx:1.25 -> FAIL
    let wrong_image_state = json!({
        "status": { "replicas": 3 },
        "spec": { "template": { "spec": { "containers": [{ "image": "nginx:1.24" }] } } }
    });
    let wrong_img_res = LabEvaluator::evaluate_task(&task, &wrong_image_state);
    assert!(!wrong_img_res.passed);
    let failed_wrong_img = wrong_img_res
        .assertion_results
        .iter()
        .filter(|a| !a.passed.unwrap_or(false))
        .count();
    assert_eq!(failed_wrong_img, 1);

    // 4. Correct full state -> PASS
    let correct_state = json!({
        "status": { "replicas": 3 },
        "spec": { "template": { "spec": { "containers": [{ "image": "nginx:1.25" }] } } }
    });
    let correct_res = LabEvaluator::evaluate_task(&task, &correct_state);
    assert!(correct_res.passed);
    let passed_count = correct_res
        .assertion_results
        .iter()
        .filter(|a| a.passed.unwrap_or(false))
        .count();
    assert_eq!(passed_count, 2);
}
