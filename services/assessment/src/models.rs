use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    MultipleChoice,
    MultiSelect,
    TrueFalse,
    FillInBlank,
    YamlBugHunt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,
    pub text: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub lesson_id: Option<String>,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub difficulty: String,
    pub points: u32,
    pub prompt: String,
    pub code_snippet: Option<String>,
    pub options: Vec<QuestionOption>,
    pub correct_answer: serde_json::Value,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionPublic {
    pub id: String,
    pub lesson_id: Option<String>,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub difficulty: String,
    pub points: u32,
    pub prompt: String,
    pub code_snippet: Option<String>,
    pub options: Vec<QuestionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSubmission {
    pub quiz_id: String,
    pub answers: std::collections::HashMap<String, serde_json::Value>,
    pub time_spent_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionEvaluation {
    pub question_id: String,
    pub is_correct: bool,
    pub earned_points: u32,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub score: u32,
    pub max_score: u32,
    pub percentage: f32,
    pub passed: bool,
    pub xp_earned: u32,
    pub breakdown: Vec<QuestionEvaluation>,
}
