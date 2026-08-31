use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub level: u32, // 0 to 5
    pub xp: u32,
    pub icon: String,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEdge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGraph {
    pub nodes: Vec<SkillNode>,
    pub edges: Vec<SkillEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub unlocked_at: Option<String>,
    pub xp_reward: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgressState {
    pub user_id: String,
    pub total_xp: u32,
    pub level: u32,
    pub current_streak_days: u32,
    pub longest_streak_days: u32,
    pub last_active_date: String,
    pub completed_lesson_ids: Vec<String>,
    pub completed_lab_ids: Vec<String>,
    pub unlocked_badges: Vec<Badge>,
    pub skills: std::collections::HashMap<String, u32>,
}
