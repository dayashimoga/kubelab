use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TutorMode {
    Explain,
    Socratic,
    Hint,
    Diagnose,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorRequest {
    pub mode: TutorMode,
    pub user_prompt: String,
    pub context_lesson_id: Option<String>,
    pub context_lab_id: Option<String>,
    pub current_error_log: Option<String>,
    pub current_yaml: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorResponse {
    pub reply_markdown: String,
    pub suggested_followups: Vec<String>,
    pub references: Vec<String>,
}
