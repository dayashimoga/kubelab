use crate::models::{
    Question, QuestionEvaluation, QuestionOption, QuestionPublic, QuestionType, QuizResult,
    QuizSubmission,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum AssessmentError {
    #[error("Quiz or Question not found")]
    NotFound,
}

pub struct AssessmentService {
    questions: Arc<RwLock<HashMap<String, Question>>>,
}

impl Default for AssessmentService {
    fn default() -> Self {
        Self::new()
    }
}

impl AssessmentService {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        // Sample real question for Pod architecture
        let q1 = Question {
            id: "q-k8s-pod-pause".to_string(),
            lesson_id: Some("k8s-pod-architecture".to_string()),
            question_type: QuestionType::MultipleChoice,
            difficulty: "easy".to_string(),
            points: 50,
            prompt: "What is the primary role of the hidden 'pause' container in a Kubernetes Pod?".to_string(),
            code_snippet: None,
            options: vec![
                QuestionOption {
                    id: "opt-a".to_string(),
                    text: "To throttle CPU and memory consumption of user containers".to_string(),
                    explanation: None,
                },
                QuestionOption {
                    id: "opt-b".to_string(),
                    text: "To create and hold the shared network namespace and IP address for all containers in the Pod".to_string(),
                    explanation: None,
                },
                QuestionOption {
                    id: "opt-c".to_string(),
                    text: "To restart crashed user containers automatically".to_string(),
                    explanation: None,
                },
                QuestionOption {
                    id: "opt-d".to_string(),
                    text: "To collect Prometheus metrics from the kubelet".to_string(),
                    explanation: None,
                },
            ],
            correct_answer: json!("opt-b"),
            explanation: "The pause container initializes first, creates the network namespace, and binds the Pod IP address. All application containers join its network namespace via --net=container:pause.".to_string(),
        };

        let q2 = Question {
            id: "q-k8s-deploy-surge".to_string(),
            lesson_id: Some("k8s-deployments-rollouts".to_string()),
            question_type: QuestionType::MultipleChoice,
            difficulty: "medium".to_string(),
            points: 50,
            prompt: "If a Deployment has replicas=4, maxSurge=1, and maxUnavailable=0 during a RollingUpdate, what is the maximum number of Pods running simultaneously?".to_string(),
            code_snippet: None,
            options: vec![
                QuestionOption {
                    id: "opt-1".to_string(),
                    text: "4 Pods".to_string(),
                    explanation: None,
                },
                QuestionOption {
                    id: "opt-2".to_string(),
                    text: "5 Pods".to_string(),
                    explanation: None,
                },
                QuestionOption {
                    id: "opt-3".to_string(),
                    text: "8 Pods".to_string(),
                    explanation: None,
                },
            ],
            correct_answer: json!("opt-2"),
            explanation: "replicas (4) + maxSurge (1) = maximum 5 Pods running simultaneously during the transition.".to_string(),
        };

        map.insert(q1.id.clone(), q1);
        map.insert(q2.id.clone(), q2);

        Self {
            questions: Arc::new(RwLock::new(map)),
        }
    }

    pub async fn get_quiz_by_lesson_id(&self, lesson_id: &str) -> Vec<QuestionPublic> {
        let questions = self.questions.read().await;
        questions
            .values()
            .filter(|q| q.lesson_id.as_deref() == Some(lesson_id))
            .map(|q| QuestionPublic {
                id: q.id.clone(),
                lesson_id: q.lesson_id.clone(),
                question_type: q.question_type.clone(),
                difficulty: q.difficulty.clone(),
                points: q.points,
                prompt: q.prompt.clone(),
                code_snippet: q.code_snippet.clone(),
                options: q.options.clone(),
            })
            .collect()
    }

    pub async fn evaluate_submission(
        &self,
        submission: &QuizSubmission,
    ) -> Result<QuizResult, AssessmentError> {
        let questions = self.questions.read().await;
        let mut total_score = 0;
        let mut max_score = 0;
        let mut breakdown = Vec::new();

        for (q_id, user_ans) in &submission.answers {
            if let Some(q) = questions.get(q_id) {
                max_score += q.points;
                let is_correct = user_ans == &q.correct_answer;
                let earned_points = if is_correct { q.points } else { 0 };
                total_score += earned_points;

                breakdown.push(QuestionEvaluation {
                    question_id: q.id.clone(),
                    is_correct,
                    earned_points,
                    explanation: q.explanation.clone(),
                });
            }
        }

        let percentage = if max_score > 0 {
            (total_score as f32 / max_score as f32) * 100.0
        } else {
            0.0
        };

        let passed = percentage >= 70.0;
        let xp_earned = if passed { total_score * 2 } else { 10 };

        Ok(QuizResult {
            score: total_score,
            max_score,
            percentage,
            passed,
            xp_earned,
            breakdown,
        })
    }
}
