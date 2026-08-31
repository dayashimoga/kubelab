use kubelab_validation_engine::models::{
    DeclarativeLabDef, LabHint, LabTask, StateAssertion, TaskValidation, TaskValidationType,
    ValidationOperator,
};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_labs_from_disk() -> Vec<DeclarativeLabDef> {
    let mut candidate_paths = Vec::new();
    if let Ok(dir) = std::env::var("KUBELAB_LABS_DIR") {
        candidate_paths.push(PathBuf::from(dir));
    }
    candidate_paths.push(PathBuf::from("labs"));
    candidate_paths.push(PathBuf::from("../../labs"));
    candidate_paths.push(PathBuf::from("../labs"));

    let mut loaded_labs = Vec::new();

    for base_path in candidate_paths {
        if base_path.exists() && base_path.is_dir() {
            if let Ok(entries) = find_yaml_files(&base_path) {
                for file_path in entries {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        if let Ok(lab) = serde_yaml::from_str::<DeclarativeLabDef>(&content) {
                            loaded_labs.push(lab);
                        }
                    }
                }
            }
            if !loaded_labs.is_empty() {
                tracing::info!(
                    "Loaded {} declarative lab definitions from {:?}",
                    loaded_labs.len(),
                    base_path
                );
                return loaded_labs;
            }
        }
    }

    // Fallback to built-in catalog if running without filesystem mounts
    get_default_lab_catalog()
}

fn find_yaml_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_yaml_files(&path)?);
            } else if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    files.push(path);
                }
            }
        }
    }
    Ok(files)
}

pub fn get_default_lab_catalog() -> Vec<DeclarativeLabDef> {
    vec![
        DeclarativeLabDef {
            id: "k8s-pod-basics".to_string(),
            title: "Create and Configure Your First Pod".to_string(),
            difficulty: "beginner".to_string(),
            duration_minutes: 15,
            track: "kubernetes".to_string(),
            objectives: vec!["Deploy pod".to_string()],
            prerequisites: vec![],
            environment: None,
            initial_state: None,
            scenario: "Deploy an nginx web container named 'web-server' listening on port 80 with label app=frontend.".to_string(),
            tasks: vec![
                LabTask {
                    id: "task-deploy-pod".to_string(),
                    title: "Deploy the Pod".to_string(),
                    description: "Create pod web-server running nginx:alpine".to_string(),
                    points: 50,
                    validation: TaskValidation {
                        validation_type: TaskValidationType::K8sResource,
                        resource: Some("pods".to_string()),
                        name: Some("web-server".to_string()),
                        namespace: None,
                        assertions: vec![
                            StateAssertion {
                                field: "status.phase".to_string(),
                                operator: ValidationOperator::Equals,
                                expected: json!("Running"),
                                actual: None,
                                passed: None,
                                error_message: None,
                            },
                            StateAssertion {
                                field: "metadata.labels.app".to_string(),
                                operator: ValidationOperator::Equals,
                                expected: json!("frontend"),
                                actual: None,
                                passed: None,
                                error_message: None,
                            },
                        ],
                    },
                },
                LabTask {
                    id: "task-verify-port".to_string(),
                    title: "Verify Container Port".to_string(),
                    description: "Ensure container specifies containerPort 80".to_string(),
                    points: 50,
                    validation: TaskValidation {
                        validation_type: TaskValidationType::K8sResource,
                        resource: Some("pods".to_string()),
                        name: Some("web-server".to_string()),
                        namespace: None,
                        assertions: vec![
                            StateAssertion {
                                field: "spec.containers[0].ports[0].containerPort".to_string(),
                                operator: ValidationOperator::Equals,
                                expected: json!(80),
                                actual: None,
                                passed: None,
                                error_message: None,
                            },
                        ],
                    },
                },
            ],
            hints: vec![
                LabHint {
                    text: "Use: kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend".to_string(),
                    penalty_points: 10,
                },
            ],
            solution: "kubectl run web-server --image=nginx:alpine --port=80 --labels=app=frontend --restart=Always".to_string(),
            cleanup: None,
            limits: None,
            security: None,
            resources: None,
            tested_versions: None,
        },
        DeclarativeLabDef {
            id: "k8s-deployments-scaling".to_string(),
            title: "Deployments, Scaling and Rolling Updates".to_string(),
            difficulty: "intermediate".to_string(),
            duration_minutes: 20,
            track: "kubernetes".to_string(),
            objectives: vec!["Scale deployment".to_string()],
            prerequisites: vec!["k8s-pod-basics".to_string()],
            environment: None,
            initial_state: None,
            scenario: "Create a Deployment named 'api-deployment' with 3 replicas and rolling update strategy.".to_string(),
            tasks: vec![
                LabTask {
                    id: "task-scale".to_string(),
                    title: "Scale to 3 replicas".to_string(),
                    description: "Ensure exactly 3 replicas are ready".to_string(),
                    points: 100,
                    validation: TaskValidation {
                        validation_type: TaskValidationType::K8sResource,
                        resource: Some("deployments".to_string()),
                        name: Some("api-deployment".to_string()),
                        namespace: None,
                        assertions: vec![
                            StateAssertion {
                                field: "status.readyReplicas".to_string(),
                                operator: ValidationOperator::Equals,
                                expected: json!(3),
                                actual: None,
                                passed: None,
                                error_message: None,
                            },
                        ],
                    },
                },
            ],
            hints: vec![],
            solution: "kubectl create deployment api-deployment --image=nginx:alpine --replicas=3".to_string(),
            cleanup: None,
            limits: None,
            security: None,
            resources: None,
            tested_versions: None,
        },
    ]
}
