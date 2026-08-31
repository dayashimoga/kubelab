# KubeLab — Requirements & Specification

## 1. System Objectives
1. **Real Infrastructure**: Provide real Linux containers, Kubernetes clusters, and cloud-native toolchains for hands-on learning and assessment.
2. **Zero Fakes**: No fake terminals, simulated metrics, fake cluster states, or command-string validation. All grading evaluates actual live system state.
3. **Zero Host Installation**: Everything required to build, test, and run the platform executes in Podman/OCI containers.
4. **Comprehensive Cloud-Native Scope**: Cover Linux, Docker/Podman, Kubernetes, Advanced K8s, Networking/CNI/Gateway API, Security, Helm/Kustomize, Istio/Service Mesh, OpenTelemetry, Prometheus/Grafana, Argo CD/GitOps, SRE, Platform Engineering, Multi-Cluster, and Incident Response.
5. **Multi-Platform Access**: Responsive Web (320px to 2560px+, WCAG 2.2 AA) + Mobile app (Android/iOS) with cross-device session continuation.

## 2. Functional Requirements

### 2.1 Learning Experience (Learn)
- 12 comprehensive tracks, 100+ lessons with progressive prerequisites.
- Rich interactive lesson view: MDX, D3/React Flow architecture diagrams, syntax highlighting, concept glossaries, trivia, and quizzes.
- Deep integration between lesson concepts and matching hands-on lab exercises.

### 2.2 Hands-on Labs & Disposable Clusters (Labs)
- Declarative lab specification schema (`id`, `title`, `difficulty`, `duration`, `prerequisites`, `environment`, `initial_state`, `scenario`, `tasks`, `hints`, `validation`, `solution`, `cleanup`, `limits`).
- Lifecycle management: `REQUEST` → `SCHEDULE` → `PROVISION` → `BOOTSTRAP` → `HEALTHCHECK` → `READY` → `LEARN` → `VALIDATE` → `SCORE` → `CHECKPOINT` → `DESTROY`.
- Multi-tenant tenant isolation with namespace sandboxing, dropped Linux capabilities, non-root users, seccomp profiles, and network policies.
- Real xterm.js terminal over WebSocket, Monaco YAML code editor with schema validation and one-click apply, real-time Kubernetes resource visualizer, log streaming, and pod events.

### 2.3 Assessment & Certification (Practice & Certs)
- State-based validation engine executing live assertions against Kubernetes API, network endpoints, logs, and telemetry.
- Question bank supporting multiple-choice, multi-select, ordering, matching, fill-in-the-blank, and YAML bug-hunt questions.
- Timed performance exams and practical hands-on certification tracks with verifiable cryptographic credentials.

### 2.4 Production Incident Simulator (Incidents)
- Real multi-tier microservice topologies deployed in disposable environments with injected chaos/faults.
- Diagnostic workflows using logs, metrics, distributed traces, and Argo CD GitOps drift inspection.
- Scoring based on diagnostic accuracy, mean time to detect (MTTD), and mean time to resolve (MTTR).

### 2.5 Skill Graph & Progression (Skill Tree & Progress)
- Directed acyclic graph (DAG) of cloud-native competencies with mastery levels (0 to 5).
- XP, streak calculations, level-ups, badge unlocks, and adaptive recommendation engine.

### 2.6 AI Tutor (Socratic & Assistive)
- Context-aware assistance modes: Explain, Socratic, Hint, Diagnose, and Review.
- Non-authoritative: AI tutor cannot grade or override deterministic validation.

## 3. Non-Functional Requirements
- **Performance**: Terminal latency < 100ms, API p95 < 50ms, lab provisioning < 15s.
- **Accessibility**: 100% WCAG 2.2 AA compliant, screen-reader support, keyboard navigable, 80%–200% zoom with no clipping or horizontal scroll.
- **Security**: Zero privileged learner containers, zero host socket exposure, strict egress rules, TLS 1.3, Argon2 password hashing, JWT + Redis session store.
- **Quality**: >90% overall test coverage (target >=95%), 100% mandatory test pass rate, no ignored failures.
