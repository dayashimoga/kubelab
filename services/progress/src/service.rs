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
        let mut map = self.user_progress.write().await;
        if let Some(state) = map.get(user_id) {
            state.clone()
        } else {
            let default_state = UserProgressState {
                user_id: user_id.to_string(),
                total_xp: 1250,
                level: 3,
                current_streak_days: 5,
                longest_streak_days: 12,
                last_active_date: "2026-08-31".to_string(),
                completed_lesson_ids: vec![
                    "k8s-pod-architecture".to_string(),
                    "k8s-deployments-rollouts".to_string(),
                ],
                completed_lab_ids: vec!["k8s-pod-basics".to_string()],
                unlocked_badges: vec![
                    Badge {
                        id: "badge-first-pod".to_string(),
                        slug: "first-pod".to_string(),
                        name: "Pod Pilot".to_string(),
                        description: "Deployed your first live Kubernetes Pod in a sandbox.".to_string(),
                        icon: "Rocket".to_string(),
                        unlocked_at: Some("2026-08-29T10:00:00Z".to_string()),
                        xp_reward: 100,
                    },
                    Badge {
                        id: "badge-streak-5".to_string(),
                        slug: "streak-5".to_string(),
                        name: "Cloud Dedication".to_string(),
                        description: "Maintained a 5-day continuous learning streak.".to_string(),
                        icon: "Flame".to_string(),
                        unlocked_at: Some("2026-08-31T08:00:00Z".to_string()),
                        xp_reward: 200,
                    },
                ],
                skills: HashMap::from([
                    ("skill-linux".to_string(), 3),
                    ("skill-containers".to_string(), 2),
                    ("skill-k8s-workloads".to_string(), 4),
                    ("skill-networking".to_string(), 3),
                    ("skill-gitops".to_string(), 3),
                    ("skill-service-mesh".to_string(), 2),
                    ("skill-observability".to_string(), 3),
                    ("skill-incidents".to_string(), 2),
                ]),
            };
            map.insert(user_id.to_string(), default_state.clone());
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
}
