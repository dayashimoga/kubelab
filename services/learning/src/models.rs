use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSummary {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub difficulty: String,
    pub order: u32,
    pub total_lessons: u32,
    pub total_xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewQuestion {
    pub question: String,
    pub answer: String,
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonDetail {
    pub id: String,
    pub track_slug: String,
    pub title: String,
    pub slug: String,
    pub order: u32,
    pub duration_minutes: u32,
    pub xp: u32,
    pub summary: String,
    pub content_markdown: String,
    pub concepts: Vec<String>,
    pub prerequisites: Vec<String>,
    pub associated_lab_id: Option<String>,
    pub associated_quiz_id: Option<String>,
    #[serde(default)]
    pub trivia: Vec<String>,
    #[serde(default)]
    pub interview_questions: Vec<InterviewQuestion>,
    #[serde(default)]
    pub mistakes_to_avoid: Vec<String>,
    #[serde(default)]
    pub production_tips: Vec<String>,
}
