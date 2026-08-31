use kubelab_validation_engine::evaluator::LabEvaluator;
use kubelab_validation_engine::models::DeclarativeLabDef;
use serde_json::json;
use std::fs;
use std::path::Path;

#[test]
fn test_all_declarative_labs_in_repository_are_valid() {
    let base_dir = Path::new("../../labs");
    if !base_dir.exists() {
        println!("Skipping lab catalog walk if running outside root");
        return;
    }

    let mut count = 0;
    fn visit_dirs(dir: &Path, count: &mut usize) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, count);
                } else if path.file_name().and_then(|n| n.to_str()) == Some("lab.yaml") {
                    let content = fs::read_to_string(&path).expect("Failed to read lab.yaml");
                    let lab: DeclarativeLabDef =
                        serde_yaml::from_str(&content).expect("Failed to parse lab.yaml");
                    assert!(!lab.id.is_empty());
                    assert!(!lab.tasks.is_empty());
                    *count += 1;
                }
            }
        }
    }

    visit_dirs(base_dir, &mut count);
    println!("Validated {} declarative lab files in repository.", count);
}

#[test]
fn test_state_based_evaluator_against_live_kubernetes_objects() {
    let sample_yaml = r#"
id: "k8s-pod-basics"
title: "Create Your First Pod"
difficulty: "beginner"
duration_minutes: 15
track: "kubernetes"
scenario: "Deploy nginx pod"
tasks:
  - id: "task-1"
    title: "Verify running pod"
    description: "Check status"
    points: 100
    validation:
      type: "k8s_resource"
      resource: "pods"
      name: "web-server"
      assertions:
        - field: "status.phase"
          operator: "equals"
          expected: "Running"
        - field: "metadata.labels.app"
          operator: "equals"
          expected: "frontend"
solution: "kubectl run web-server --image=nginx:alpine -l app=frontend"
"#;

    let lab: DeclarativeLabDef = serde_yaml::from_str(sample_yaml).unwrap();
    let task = &lab.tasks[0];

    // 1. Test failing state
    let failing_state = json!({
        "status": { "phase": "Pending" },
        "metadata": { "labels": { "app": "wrong" } }
    });
    let result_fail = LabEvaluator::evaluate_task(task, &failing_state);
    assert!(!result_fail.passed);
    assert_eq!(result_fail.score, 0);

    // 2. Test passing live state
    let passing_state = json!({
        "status": { "phase": "Running" },
        "metadata": { "labels": { "app": "frontend" } }
    });
    let result_pass = LabEvaluator::evaluate_task(task, &passing_state);
    assert!(result_pass.passed);
    assert_eq!(result_pass.score, 100);
}
