use kubelab_ai_tutor::AiTutorService;
use kubelab_assessment::AssessmentService;
use kubelab_auth::AuthService;
use kubelab_lab_orchestrator::LabProvisioner;
use kubelab_labs::LabService;
use kubelab_learning::LearningService;
use kubelab_notification::NotificationService;
use kubelab_progress::ProgressService;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub auth: Arc<AuthService>,
    pub learning: Arc<LearningService>,
    pub assessment: Arc<AssessmentService>,
    pub labs: Arc<LabService>,
    pub orchestrator: Arc<LabProvisioner>,
    pub progress: Arc<ProgressService>,
    pub notification: Arc<NotificationService>,
    pub ai_tutor: Arc<AiTutorService>,
}

impl AppState {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            auth: Arc::new(AuthService::new(jwt_secret)),
            learning: Arc::new(LearningService::new()),
            assessment: Arc::new(AssessmentService::new()),
            labs: Arc::new(LabService::new()),
            orchestrator: Arc::new(LabProvisioner::new()),
            progress: Arc::new(ProgressService::new()),
            notification: Arc::new(NotificationService::new()),
            ai_tutor: Arc::new(AiTutorService::new()),
        }
    }
}
