# KubeLab Product Guide

## What is KubeLab?

KubeLab is an open-source, production-grade Cloud-Native Engineering Learning Platform. It provides a hands-on learning environment where engineers practice real Kubernetes operations in disposable sandbox clusters — not simulators or mocks.

## Target Personas

| Persona | Description | Primary Workflows |
|---|---|---|
| **Learner** | Cloud-native engineer (beginner → advanced) | Learn → Practice → Lab → Grade → Certify |
| **Instructor** | Creates labs and curriculum content | Author labs → Define grading → Review progress |
| **Admin** | Platform operator | Deploy → Monitor → Scale → Maintain |

## Core Capabilities

### 1. Progressive Learning Curriculum
- 12 tracks covering Linux → Kubernetes → GitOps → SRE → Incident Response
- 145 hands-on labs with real infrastructure
- Interactive quizzes and assessments
- Skill tree DAG with prerequisite tracking
- XP-based progression with levels and badges

### 2. Real Lab Sandboxes
- Disposable Kubernetes namespaces per learner
- PSS `restricted` enforcement — learn in production-like constraints
- Real `kubectl`, Helm, and infrastructure tooling
- xterm.js terminal with WebSocket connectivity
- Monaco editor for YAML manifest editing

### 3. Deterministic Grading
- State-based validation via Kubernetes API queries
- JSONPath assertions against actual cluster state
- No regex-on-shell-output brittleness
- Immediate feedback with hints and solutions

### 4. Production Incident Simulation
- Injectable faults: DNS failures, crashloops, memory leaks, cert expiry
- Service mesh routing bugs and GitOps drift scenarios
- Real multi-tier microservices with observable symptoms
- SRE workflow practice with SLI/SLO monitoring

### 5. Mobile Companion
- Flutter app for Android and iOS
- Lesson reading, quiz taking, progress tracking
- "Continue on Desktop" lab handoff
- Offline reading support

## Limitations

- Labs require a running Kubernetes cluster (Kind/k3d for local, remote for production)
- Mobile app cannot run terminal/lab sessions directly (desktop handoff)
- AI tutor requires external LLM API configuration
- Signed iOS IPA distribution requires Apple Developer credentials
