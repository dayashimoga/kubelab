use kubelab_validation_engine::evaluator::LabEvaluator;
use kubelab_validation_engine::models::{
    LabTask, StateAssertion, TaskValidation, TaskValidationType, ValidationOperator,
};
use serde_json::json;

#[test]
fn test_evaluator_all_operators_and_nested_paths() {
    let actual_state = json!({
        "kind": "Deployment",
        "metadata": {
            "name": "payment-service",
            "labels": {
                "tier": "backend",
                "env": "production"
            }
        },
        "spec": {
            "replicas": 3,
            "template": {
                "spec": {
                    "containers": [
                        {
                            "name": "payment-api",
                            "image": "docker.io/kubelab/payment:v2.4.1",
                            "resources": {
                                "limits": {
                                    "memory": "512Mi",
                                    "cpu": 0.5
                                }
                            },
                            "env": ["DB_HOST", "REDIS_HOST"]
                        }
                    ]
                }
            }
        },
        "status": {
            "readyReplicas": 3,
            "availableReplicas": 3
        }
    });

    let task = LabTask {
        id: "task-advanced-verification".to_string(),
        title: "Verify Production Deployment Spec".to_string(),
        description: "Exhaustive check of replicas, limits, labels, and environment".to_string(),
        points: 100,
        validation: TaskValidation {
            validation_type: TaskValidationType::K8sResource,
            resource: Some("deployments".to_string()),
            name: Some("payment-service".to_string()),
            namespace: None,
            assertions: vec![
                // 1. Equals check on string
                StateAssertion {
                    field: "metadata.labels.env".to_string(),
                    operator: ValidationOperator::Equals,
                    expected: json!("production"),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                // 2. Contains check on array
                StateAssertion {
                    field: "spec.template.spec.containers[0].env".to_string(),
                    operator: ValidationOperator::Contains,
                    expected: json!("REDIS_HOST"),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                // 3. Contains check on substring
                StateAssertion {
                    field: "spec.template.spec.containers[0].image".to_string(),
                    operator: ValidationOperator::Contains,
                    expected: json!("payment:v2"),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                // 4. Regex match
                StateAssertion {
                    field: "spec.template.spec.containers[0].image".to_string(),
                    operator: ValidationOperator::MatchesRegex,
                    expected: json!(r"^docker\.io/kubelab/payment:v[0-9]+\.[0-9]+\.[0-9]+$"),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                // 5. Greater Than numeric
                StateAssertion {
                    field: "status.readyReplicas".to_string(),
                    operator: ValidationOperator::GreaterThan,
                    expected: json!(2),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
                // 6. Less Than numeric
                StateAssertion {
                    field: "spec.template.spec.containers[0].resources.limits.cpu".to_string(),
                    operator: ValidationOperator::LessThan,
                    expected: json!(1.0),
                    actual: None,
                    passed: None,
                    error_message: None,
                },
            ],
        },
    };

    let result = LabEvaluator::evaluate_task(&task, &actual_state);
    assert!(result.passed, "All 6 complex assertions should pass");
    assert_eq!(result.score, 100);
    assert_eq!(result.assertion_results.len(), 6);
    for a in result.assertion_results {
        assert_eq!(a.passed, Some(true));
        assert!(a.error_message.is_none());
    }
}
