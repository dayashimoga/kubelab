use crate::data::{get_default_lessons, get_default_tracks};
use crate::models::{LessonDetail, TrackSummary};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum LearningError {
    #[error("Track not found")]
    TrackNotFound,
    #[error("Lesson not found")]
    LessonNotFound,
}

pub struct LearningService {
    tracks: Arc<RwLock<Vec<TrackSummary>>>,
    lessons: Arc<RwLock<HashMap<String, LessonDetail>>>,
}

impl Default for LearningService {
    fn default() -> Self {
        Self::new()
    }
}

impl LearningService {
    pub fn new() -> Self {
        let tracks = get_default_tracks();
        let default_lessons = get_default_lessons();

        let mut lesson_map = HashMap::new();
        for l in default_lessons {
            lesson_map.insert(l.id.clone(), l);
        }

        Self {
            tracks: Arc::new(RwLock::new(tracks)),
            lessons: Arc::new(RwLock::new(lesson_map)),
        }
    }

    pub async fn list_tracks(&self) -> Vec<TrackSummary> {
        self.tracks.read().await.clone()
    }

    pub async fn get_track_by_slug(&self, slug: &str) -> Result<TrackSummary, LearningError> {
        let tracks = self.tracks.read().await;
        tracks
            .iter()
            .find(|t| t.slug == slug)
            .cloned()
            .ok_or(LearningError::TrackNotFound)
    }

    pub async fn get_lesson_by_id(&self, id: &str) -> Result<LessonDetail, LearningError> {
        let lessons = self.lessons.read().await;
        lessons
            .get(id)
            .cloned()
            .ok_or(LearningError::LessonNotFound)
    }

    pub async fn list_lessons_by_track(&self, track_slug: &str) -> Vec<LessonDetail> {
        let lessons = self.lessons.read().await;
        lessons
            .values()
            .filter(|l| l.track_slug == track_slug)
            .cloned()
            .collect()
    }
}
