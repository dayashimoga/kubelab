use crate::models::{Badge, SkillGraph, UserProgressState};
use crate::skill_graph::generate_cloud_native_skill_graph;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ProgressService {
    user_progress: Arc<RwLock<HashMap<String, UserProgressState>>>,
    skill_graph: SkillGraph,
}

impl Default for ProgressService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressService {
    pub fn new() -> Self {
        Self {
            user_progress: Arc::new(RwLock::new(HashMap::new())),
            skill_graph: generate_cloud_native_skill_graph(),
        }
    }

    pub async fn get_user_progress(&self, user_id: &str) -> UserProgressState {
        let map = self.user_progress.read().await;
        if let Some(state) = map.get(user_id) {
            state.clone()
        } else {
            drop(map);
            // New users start fresh — zero hardcoded progress
            let default_state = UserProgressState {
                user_id: user_id.to_string(),
                total_xp: 0,
                level: 1,
                current_streak_days: 0,
                longest_streak_days: 0,
                last_active_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                completed_lesson_ids: vec![],
                completed_lab_ids: vec![],
                unlocked_badges: vec![],
                skills: HashMap::new(),
            };
            let mut wmap = self.user_progress.write().await;
            wmap.insert(user_id.to_string(), default_state.clone());
            default_state
        }
    }

    pub fn get_skill_graph(&self) -> SkillGraph {
        self.skill_graph.clone()
    }

    pub async fn add_xp(&self, user_id: &str, xp: u32) -> UserProgressState {
        let mut progress = self.get_user_progress(user_id).await;
        progress.total_xp += xp;
        progress.level = (progress.total_xp / 500) + 1;

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());
        progress
    }

    pub async fn complete_lesson(
        &self,
        user_id: &str,
        lesson_id: &str,
        xp: u32,
    ) -> UserProgressState {
        let mut progress = self.get_user_progress(user_id).await;
        if !progress
            .completed_lesson_ids
            .contains(&lesson_id.to_string())
        {
            progress.completed_lesson_ids.push(lesson_id.to_string());
            progress.total_xp += xp;
            progress.level = (progress.total_xp / 500) + 1;
        }

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());
        progress
    }

    pub async fn complete_lab(&self, user_id: &str, lab_id: &str, xp: u32) -> UserProgressState {
        let mut progress = self.get_user_progress(user_id).await;
        if !progress.completed_lab_ids.contains(&lab_id.to_string()) {
            progress.completed_lab_ids.push(lab_id.to_string());
            progress.total_xp += xp;
            progress.level = (progress.total_xp / 500) + 1;

            // Award badges based on milestones
            if progress.completed_lab_ids.len() == 1 {
                progress.unlocked_badges.push(Badge {
                    id: "badge-first-pod".to_string(),
                    slug: "first-pod".to_string(),
                    name: "Pod Pilot".to_string(),
                    description: "Completed your first hands-on Kubernetes lab.".to_string(),
                    icon: "Rocket".to_string(),
                    unlocked_at: Some(chrono::Utc::now().to_rfc3339()),
                    xp_reward: 100,
                });
                progress.total_xp += 100;
            }
            if progress.completed_lab_ids.len() == 5 {
                progress.unlocked_badges.push(Badge {
                    id: "badge-lab-veteran".to_string(),
                    slug: "lab-veteran".to_string(),
                    name: "Lab Veteran".to_string(),
                    description: "Completed 5 hands-on labs.".to_string(),
                    icon: "Award".to_string(),
                    unlocked_at: Some(chrono::Utc::now().to_rfc3339()),
                    xp_reward: 500,
                });
                progress.total_xp += 500;
            }

            progress.level = (progress.total_xp / 500) + 1;
        }

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());
        progress
    }

    pub async fn update_skill(
        &self,
        user_id: &str,
        skill_id: &str,
        level: u32,
    ) -> UserProgressState {
        let mut progress = self.get_user_progress(user_id).await;
        let current = progress.skills.get(skill_id).copied().unwrap_or(0);
        if level > current {
            progress.skills.insert(skill_id.to_string(), level);
        }

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());
        progress
    }

    pub async fn update_streak(&self, user_id: &str) -> UserProgressState {
        let mut progress = self.get_user_progress(user_id).await;
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        if progress.last_active_date != today {
            // Check if yesterday — if so, increment streak
            progress.current_streak_days += 1;
            if progress.current_streak_days > progress.longest_streak_days {
                progress.longest_streak_days = progress.current_streak_days;

                // Streak badges
                if progress.longest_streak_days == 5 {
                    progress.unlocked_badges.push(Badge {
                        id: "badge-streak-5".to_string(),
                        slug: "streak-5".to_string(),
                        name: "Cloud Dedication".to_string(),
                        description: "Maintained a 5-day continuous learning streak.".to_string(),
                        icon: "Flame".to_string(),
                        unlocked_at: Some(chrono::Utc::now().to_rfc3339()),
                        xp_reward: 200,
                    });
                    progress.total_xp += 200;
                }
            }
            progress.last_active_date = today;
        }

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());
        progress
    }

    pub async fn sync_progress(
        &self,
        user_id: &str,
        req: crate::models::ProgressSyncRequest,
    ) -> crate::models::ProgressSyncResponse {
        let mut progress = self.get_user_progress(user_id).await;

        // Merge completed lessons (set union)
        for lid in req.completed_lessons {
            if !progress.completed_lesson_ids.contains(&lid) {
                progress.completed_lesson_ids.push(lid);
            }
        }

        // Merge completed labs (set union)
        for lab_id in req.completed_labs {
            if !progress.completed_lab_ids.contains(&lab_id) {
                progress.completed_lab_ids.push(lab_id);
            }
        }

        // Max XP
        if req.total_xp > progress.total_xp {
            progress.total_xp = req.total_xp;
            progress.level = (progress.total_xp / 500) + 1;
        }

        if let Some(la) = req.last_active {
            progress.last_active_date = la;
        }

        let mut map = self.user_progress.write().await;
        map.insert(user_id.to_string(), progress.clone());

        crate::models::ProgressSyncResponse {
            user_id: user_id.to_string(),
            merged_state: progress,
            synced_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
