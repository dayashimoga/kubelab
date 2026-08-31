# Product & Engineering Requirements Specification

## 1. Executive Summary
KubeLab is an open-source, production-grade learning and certification platform purpose-built for cloud-native engineering. The platform replaces passive video/mock-based learning with **real live infrastructure, deterministic state grading, and production incident response simulation**.

## 2. Core Capabilities Matrix

| Capability | Requirement | Validation Method |
|---|---|---|
| **Real Shell & Terminal** | Non-root xterm.js terminal connected to live sandboxed containers | WebSocket ping/pong, stdout stream assertion |
| **Real Kubernetes Sandbox** | Ephemeral, isolated namespace with custom NetworkPolicies per session | `kubectl get ns`, network egress test |
| **Monaco Editor Integration** | YAML/JSON editor with live Kubernetes schema linting and 1-click apply | Monaco worker schema validation |
| **State-Based Grading** | Assertions on live Kubernetes object state, endpoints, logs, and metrics | Direct k8s API client query and assertions |
| **Incident Simulation** | Multi-tier microservice apps with chaos injection (crashloops, drift, cert expiry) | Scenario timeline engine |
| **Skill Graph Engine** | Competency DAG calculating mastery levels 0–5, streaks, and XP | Graph traversal unit & integration tests |
| **Mobile Learning** | Cross-platform Flutter client with offline lessons and Desktop handoff | Flutter driver / widget tests |
| **Zero Host Setup** | Complete system runs via Podman/containers without local dependencies | Containerfile & podman-compose tests |
