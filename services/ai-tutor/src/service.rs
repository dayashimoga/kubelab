use crate::models::{TutorMode, TutorRequest, TutorResponse};
use serde_json::json;
use std::time::Duration;

pub struct AiTutorService {
    http_client: reqwest::Client,
}

impl Default for AiTutorService {
    fn default() -> Self {
        Self::new()
    }
}

impl AiTutorService {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn query_tutor(&self, req: &TutorRequest) -> TutorResponse {
        // 1. Try querying local containerized Ollama if configured
        if let Ok(ollama_host) = std::env::var("OLLAMA_HOST") {
            if let Ok(resp) = self.query_ollama(&ollama_host, req).await {
                return resp;
            }
        }

        // 2. Try querying OpenAI-compatible endpoint if API key configured
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            if let Ok(resp) = self.query_openai_compatible(&api_key, req).await {
                return resp;
            }
        }

        // 3. Explicit service-unavailable response per production specification
        TutorResponse {
            reply_markdown: "⚠️ **AI Socratic Tutor Service Unconfigured / Offline**\n\nNo live LLM provider endpoint is currently reachable. To enable live interactive AI Socratic reasoning, configure an `OLLAMA_HOST` (e.g. `http://localhost:11434`) or `OPENAI_API_KEY` in your environment settings.\n\nIn the meantime, refer to the verified lesson documentation, operational commands, and architecture diagrams above.".to_string(),
            suggested_followups: vec![
                "Configure OLLAMA_HOST in Settings".to_string(),
                "Review Lesson Architecture Diagram".to_string(),
                "Launch Live Lab Workspace".to_string(),
            ],
            references: vec![
                "https://kubernetes.io/docs/concepts/".to_string(),
                "https://kubernetes.io/docs/reference/kubectl/cheatsheet/".to_string(),
            ],
        }
    }

    async fn query_ollama(
        &self,
        host: &str,
        req: &TutorRequest,
    ) -> Result<TutorResponse, Box<dyn std::error::Error>> {
        let endpoint = format!("{}/api/generate", host.trim_end_matches('/'));
        let system_prompt = self.build_system_prompt(&req.mode);
        let prompt = format!(
            "System: {}\nUser Prompt: {}\nContext: YAML: {:?}\nError: {:?}",
            system_prompt, req.user_prompt, req.current_yaml, req.current_error_log
        );

        let body = json!({
            "model": std::env::var("AI_MODEL_NAME").unwrap_or_else(|_| "llama3:8b".to_string()),
            "prompt": prompt,
            "stream": false
        });

        let res = self.http_client.post(&endpoint).json(&body).send().await?;
        let json_res: serde_json::Value = res.json().await?;
        let reply = json_res["response"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();

        Ok(TutorResponse {
            reply_markdown: reply,
            suggested_followups: self.get_suggested_followups(&req.mode),
            references: self.get_references(&req.mode),
        })
    }

    async fn query_openai_compatible(
        &self,
        api_key: &str,
        req: &TutorRequest,
    ) -> Result<TutorResponse, Box<dyn std::error::Error>> {
        let endpoint = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
        let system_prompt = self.build_system_prompt(&req.mode);

        let messages = vec![
            json!({ "role": "system", "content": system_prompt }),
            json!({
                "role": "user",
                "content": format!("Prompt: {}\nYAML: {:?}\nError: {:?}", req.user_prompt, req.current_yaml, req.current_error_log)
            }),
        ];

        let body = json!({
            "model": std::env::var("AI_MODEL_NAME").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
            "messages": messages,
            "temperature": 0.3
        });

        let res = self
            .http_client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let reply = json_res["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();

        Ok(TutorResponse {
            reply_markdown: reply,
            suggested_followups: self.get_suggested_followups(&req.mode),
            references: self.get_references(&req.mode),
        })
    }

    fn build_system_prompt(&self, mode: &TutorMode) -> &'static str {
        match mode {
            TutorMode::Explain => "You are an expert Kubernetes and Cloud-Native SRE tutor. Provide deep architectural explanations with clear ASCII diagrams, control plane mechanics, and practical implications.",
            TutorMode::Socratic => "You are a Socratic Kubernetes instructor. DO NOT give direct answers. Ask targeted, thought-provoking questions that guide the learner to inspect their own cluster state and discover the solution.",
            TutorMode::Hint => "You are a helpful Kubernetes lab assistant. Provide minimal, progressive, non-spoiling hints pointing directly to relevant kubectl commands, API fields, or Kubernetes documentation.",
            TutorMode::Diagnose => "You are a principal Kubernetes incident commander. Analyze the provided error logs and YAML manifests forensically. Identify the root cause (e.g. OOM, DNS, probe failure, NetworkPolicy) and propose diagnostic verification steps.",
            TutorMode::Review => "You are a Kubernetes security and reliability auditor. Review the provided YAML manifest against production best practices (resource requests/limits, securityContext, readinessProbes, PodDisruptionBudgets).",
        }
    }

    #[allow(dead_code)]
    fn pedagogical_contextual_response(&self, req: &TutorRequest) -> TutorResponse {
        match req.mode {
            TutorMode::Explain => TutorResponse {
                reply_markdown: format!(
                    "### Concept Explanation & Architecture Deep-Dive\n\nIn Kubernetes, **{}** represents a core cloud-native primitive. The control plane reconciles desired state against live cluster state via continuous control loops.\n\n```text\n[kubectl / API Request] ──> [kube-apiserver] ──> [etcd (Source of Truth)]\n                                  │\n                           [kube-controller-manager]\n                                  │\n                    [kubelet on Node Worker] ──> [Container Runtime (CRI)]\n```\n\n#### Key Mechanics:\n1. **Declarative Reconciliation**: You declare target state in YAML; Kubernetes continuously converges reality towards it.\n2. **Decoupled Architecture**: Workloads (Pods) communicate across flat overlay networks without host port collision.\n3. **Self-Healing**: Pods killed or nodes failing automatically trigger replacement replica scheduling.",
                    req.user_prompt
                ),
                suggested_followups: self.get_suggested_followups(&req.mode),
                references: self.get_references(&req.mode),
            },
            TutorMode::Socratic => TutorResponse {
                reply_markdown: "Let's think through this step-by-step:\n\n1. What does `kubectl get pods -o wide` show for the pod status and node assignment?\n2. In `kubectl describe pod <name>`, what exact message appears in the `Events` table?\n3. If a container is restarting repeatedly, what is the exit code reported in `Last State: Terminated` (e.g., exit code 137 for OOMKilled vs 1 for application exception)?".to_string(),
                suggested_followups: self.get_suggested_followups(&req.mode),
                references: self.get_references(&req.mode),
            },
            TutorMode::Hint => TutorResponse {
                reply_markdown: "💡 **Guiding Hint & Targeted Guidance**: Check the label selectors in your manifest. If `spec.selector.matchLabels` on your Deployment does not match `spec.template.metadata.labels`, the deployment controller cannot adopt the Pods, and `kubectl get pods` will show no instances.".to_string(),
                suggested_followups: self.get_suggested_followups(&req.mode),
                references: self.get_references(&req.mode),
            },
            TutorMode::Diagnose => {
                let error = req.current_error_log.as_deref().unwrap_or("Application startup failed");
                TutorResponse {
                    reply_markdown: format!(
                        "🔍 **Diagnostic Analysis & Forensic Root Cause**\n\n**Log Snippet Analyzed**:\n```\n{}\n```\n\n**Potential Failure Modes**:\n- **Exit Code 137 (OOMKilled)**: The Linux cgroup memory limit (`resources.limits.memory`) was exceeded by the container process.\n- **CrashLoopBackOff**: Container failed its startup or liveness probe, or crashed on initial entrypoint.\n- **ImagePullBackOff / ErrImagePull**: Typo in image name, missing tag, or unauthenticated private container registry.\n\n**Remediation Steps**:\n1. Inspect pod logs: `kubectl logs <pod-name> --previous`\n2. Describe events: `kubectl describe pod <pod-name>`",
                        error
                    ),
                    suggested_followups: self.get_suggested_followups(&req.mode),
                    references: self.get_references(&req.mode),
                }
            }
            TutorMode::Review => TutorResponse {
                reply_markdown: "🛡️ **YAML Review & Production Manifest Audit**\n\n1. **Resource Constraints**: Verify both `requests` and `limits` are configured to prevent node starvation.\n2. **Security Hardening**: Ensure `securityContext.runAsNonRoot: true` and `allowPrivilegeEscalation: false` are set.\n3. **Probes**: Ensure both `readinessProbe` and `livenessProbe` are configured with appropriate `initialDelaySeconds`.".to_string(),
                suggested_followups: self.get_suggested_followups(&req.mode),
                references: self.get_references(&req.mode),
            }
        }
    }

    fn get_suggested_followups(&self, mode: &TutorMode) -> Vec<String> {
        match mode {
            TutorMode::Explain => vec![
                "How does kube-proxy handle iptables vs IPVS mode?".to_string(),
                "What happens when etcd loses quorum?".to_string(),
            ],
            TutorMode::Socratic => vec![
                "I see CrashLoopBackOff with Exit Code 137".to_string(),
                "The readiness probe failed on port 8080".to_string(),
            ],
            TutorMode::Hint => vec!["How do I check endpoints with kubectl?".to_string()],
            TutorMode::Diagnose => vec![
                "How do I profile memory consumption inside the container?".to_string(),
                "How do I increase limits without restarting the deployment?".to_string(),
            ],
            TutorMode::Review => {
                vec!["How do I validate this manifest with kubeval or kubeconform?".to_string()]
            }
        }
    }

    fn get_references(&self, _mode: &TutorMode) -> Vec<String> {
        vec![
            "https://kubernetes.io/docs/concepts/workloads/pods/".to_string(),
            "https://kubernetes.io/docs/tasks/debug/debug-application/".to_string(),
        ]
    }
}
