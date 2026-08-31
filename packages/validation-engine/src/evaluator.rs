use crate::assertions::evaluate_assertion;
use crate::models::{LabTask, ValidationResult};
use serde_json::Value;

pub struct LabEvaluator;

impl LabEvaluator {
    pub fn evaluate_task(task: &LabTask, actual_k8s_state: &Value) -> ValidationResult {
        let mut assertions = task.validation.assertions.clone();
        let mut all_passed = true;

        for assertion in assertions.iter_mut() {
            let passed = evaluate_assertion(assertion, actual_k8s_state);
            if !passed {
                all_passed = false;
            }
        }

        let score = if all_passed { task.points } else { 0 };

        ValidationResult {
            task_id: task.id.clone(),
            passed: all_passed,
            score,
            max_score: task.points,
            assertion_results: assertions,
        }
    }
}
