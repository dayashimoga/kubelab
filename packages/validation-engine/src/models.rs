use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOperator {
    Equals,
    Contains,
    MatchesRegex,
    GreaterThan,
    LessThan,
    HttpGet,
    JsonpathMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateAssertion {
    pub field: String,
    pub operator: ValidationOperator,
    pub expected: serde_json::Value,
    #[serde(default)]
    pub actual: Option<serde_json::Value>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskValidationType {
    K8sResource,
    NetworkEndpoint,
    LogMatch,
    MetricQuery,
    CustomScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskValidation {
    #[serde(rename = "type")]
    pub validation_type: TaskValidationType,
    pub resource: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub assertions: Vec<StateAssertion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub points: u32,
    pub validation: TaskValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabHint {
    pub text: String,
    #[serde(default)]
    pub penalty_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarativeLabDef {
    pub id: String,
    pub title: String,
    pub difficulty: String,
    pub duration_minutes: u32,
    pub track: String,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub environment: Option<serde_json::Value>,
    #[serde(default)]
    pub initial_state: Option<serde_json::Value>,
    pub scenario: String,
    pub tasks: Vec<LabTask>,
    #[serde(default)]
    pub hints: Vec<LabHint>,
    #[serde(default)]
    pub solution: String,
    #[serde(default)]
    pub cleanup: Option<serde_json::Value>,
    #[serde(default)]
    pub limits: Option<serde_json::Value>,
    #[serde(default)]
    pub security: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<serde_json::Value>,
    #[serde(default)]
    pub tested_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub task_id: String,
    pub passed: bool,
    pub score: u32,
    pub max_score: u32,
    pub assertion_results: Vec<StateAssertion>,
}
