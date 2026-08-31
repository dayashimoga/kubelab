use kubelab_validation_engine::evaluator::LabEvaluator;
use kubelab_validation_engine::models::{
    LabTask, StateAssertion, TaskValidation, TaskValidationType, ValidationOperator,
};
use serde_json::json;

#[test]
fn test_evaluator_never_auto_passes_unavailable_state() {
    let task = LabTask {
        id: "task-verify-deployment".to_string(),
        title: "Deploy NGINX with 3 replicas".to_string(),
        description: "Scale frontend deployment to 3 replicas".to_string(),
        points: 50,
        validation: TaskValidation {
            validation_type: TaskValidationType::K8sResource,
            resource: Some("deployments".to_string()),
            name: Some("frontend".to_string()),
            namespace: None,
            assertions: vec![StateAssertion {
                field: "$.spec.replicas".to_string(),
                operator: ValidationOperator::Equals,
                expected: json!(3),
                actual: None,
                passed: None,
                error_message: None,
            }],
        },
    };

    // 1. Unavailable state -> MUST FAIL
    let unavailable_state = json!({
        "status": {
            "phase": "Unavailable",
            "error": "No live Kubernetes cluster client configured for sandbox"
        }
    });
    let result = LabEvaluator::evaluate_task(&task, &unavailable_state);
    assert!(!result.passed, "Unavailable cluster state must NOT pass");

    // 2. Resource Not Found state -> MUST FAIL
    let not_found_state = json!({
        "status": {
            "phase": "NotFound"
        }
    });
    let result = LabEvaluator::evaluate_task(&task, &not_found_state);
    assert!(!result.passed, "NotFound resource state must NOT pass");

    // 3. Incorrect replicas state (e.g. 1 instead of 3) -> MUST FAIL
    let wrong_state = json!({
        "spec": {
            "replicas": 1
        }
    });
    let result = LabEvaluator::evaluate_task(&task, &wrong_state);
    assert!(!result.passed, "Incorrect replica count must NOT pass");

    // 4. Exact matching state -> MUST PASS
    let correct_state = json!({
        "spec": {
            "replicas": 3
        }
    });
    let result = LabEvaluator::evaluate_task(&task, &correct_state);
    assert!(result.passed, "Exact matching replica count must pass");
}
