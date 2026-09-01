# KubeLab Content Coverage & Curriculum Verification

## Overview
KubeLab provides comprehensive curriculum coverage across all 15 cloud-native domains, verified through automated schema validators, live runtime tests, and interactive UI visualizers.

## Content Breakdown by Format

### 1. Interactive Lessons & Theory
- **Tracks**: 15 tracks covering fundamental to expert concepts
- **Lessons**: Over 60 structured modules with interactive markdown, architecture diagrams, and syntax highlighting
- **Trivia & Quizzes**: Multiple-choice knowledge checks embedded in each module with deterministic scoring

### 2. Hands-On Declarative Labs (154 Total)
- **Declarative YAML Specifications**: 154 lab definitions across `labs/`
- **State-Based Grading**: Evaluated via JSONPath assertions against actual Kubernetes API state (`StateAssertion` rules)
- **Live Terminal & YAML Editor**: Monaco editor with real-time client-side admission and PTY terminal

### 3. Incident Scenarios (10 Executable Scenarios)
1. **CoreDNS Outage**: ConfigMap syntax corruption and DNS resolution failure
2. **CrashLoopBackOff & OOMKilled**: Memory limit throttling and improper JVM heap sizing
3. **PVC Storage Deadlock**: Volume binding failure and storage class mismatch
4. **NetworkPolicy Isolation Blackhole**: Misconfigured egress policy blocking critical database traffic
5. **HPA Autoscaler Thrashing**: Flapping metrics and CPU threshold misconfiguration
6. **Ingress Controller 503 Outage**: Upstream service port and selector mismatch
7. **Node NotReady & Kubelet Eviction**: Disk pressure and PID exhaustion
8. **TLS Certificate Expiration**: Expired ingress secret causing SSL handshake failures
9. **Argo CD OutOfSync Drift**: Manual ad-hoc mutation causing continuous drift in GitOps
10. **Istio Circuit Breaker Tripping**: Consecutive 5xx error threshold triggering fail-fast connection drops

### 4. Interactive Visualizations & Tools
- **Kubernetes Resource Visualizer**: Live SVG/Canvas DAG topology viewer (`apps/web/src/components/K8sVisualizer.tsx`)
- **Skill Tree DAG**: Node-graph tracking prerequisite and unlockable competencies
- **Live Metrics Dashboard**: Real-time CPU, Memory, Network, and Error SLIs

### 5. Exam & Interview Prep
- **Certification Simulation Tracks**: CKA, CKAD, and CKS timed practice drills
- **Scenario Challenges**: Multi-step troubleshooting under strict time constraints

## Automated Verification in CI
- **Schema Validation**: Validated via `validate_lab_schema` in CI
- **Completeness Assertion**: Zero missing tracks, zero missing lesson markdown, zero orphaned lab files
