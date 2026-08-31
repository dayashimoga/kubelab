use crate::models::{TutorMode, TutorRequest, TutorResponse};

pub struct AiTutorService;

impl Default for AiTutorService {
    fn default() -> Self {
        Self::new()
    }
}

impl AiTutorService {
    pub fn new() -> Self {
        Self
    }

    pub async fn query_tutor(&self, req: &TutorRequest) -> TutorResponse {
        match req.mode {
            TutorMode::Explain => TutorResponse {
                reply_markdown: format!(
                    "### Concept Explanation\n\nIn cloud-native architecture, **{}** plays a fundamental role. Kubernetes decouples compute from physical infrastructure through declarative reconciliation loops. Let's break down the components:\n\n1. **Control Plane**: Evaluates desired vs actual state.\n2. **Kubelet & Worker Nodes**: Enforce container boundaries.\n3. **Network Plane**: Assigns unique routable IP per Pod.",
                    req.user_prompt
                ),
                suggested_followups: vec![
                    "How does kube-proxy route traffic to this Pod?".to_string(),
                    "What happens when a node fails?".to_string(),
                ],
                references: vec!["https://kubernetes.io/docs/concepts/workloads/pods/".to_string()],
            },
            TutorMode::Socratic => TutorResponse {
                reply_markdown: "Let's think through this step-by-step:\n\n1. What does the `kubectl describe pod` output reveal in the `Events` section?\n2. Is the container crashing on startup, or is it failing a readiness probe?\n3. What would you check first: image pull secrets or environment variable bindings?".to_string(),
                suggested_followups: vec![
                    "I see CrashLoopBackOff in the status".to_string(),
                    "The readiness probe failed on /healthz".to_string(),
                ],
                references: vec!["docs/troubleshooting/TROUBLESHOOTING.md".to_string()],
            },
            TutorMode::Hint => TutorResponse {
                reply_markdown: "💡 **Guiding Hint**: Check the label selectors on your Service definition. If `spec.selector` in your Service does not match `spec.template.metadata.labels` on your Deployment, no endpoints will be attached, and curl requests will hang.".to_string(),
                suggested_followups: vec![
                    "How do I view endpoints for a service?".to_string(),
                ],
                references: vec![],
            },
            TutorMode::Diagnose => {
                let error = req.current_error_log.as_deref().unwrap_or("No error provided");
                TutorResponse {
                    reply_markdown: format!(
                        "🔍 **Diagnostic Analysis**\n\nReceived error:\n```\n{}\n```\n\n**Root Cause Assessment**:\n- If you see `ImagePullBackOff`, check registry authentication or image name typo.\n- If you see `OOMKilled`, increase the container memory limit in `resources.limits.memory`.\n- If you see `ErrImageNeverPull`, verify the image exists in local Podman/Docker storage.",
                        error
                    ),
                    suggested_followups: vec![
                        "How do I increase memory limits in YAML?".to_string(),
                    ],
                    references: vec![],
                }
            }
            TutorMode::Review => TutorResponse {
                reply_markdown: "✅ **YAML Review**\n\nYour manifest structure follows Kubernetes best practices! Remember to specify both `requests` and `limits` to give the scheduler reliable placement constraints.".to_string(),
                suggested_followups: vec![],
                references: vec![],
            },
        }
    }
}
