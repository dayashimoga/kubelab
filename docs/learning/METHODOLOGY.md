# KubeLab Pedagogical & Learning Methodology

## The 5-Stage Active Learning Loop

Every concept in KubeLab adheres to the active learning pipeline:

```text
┌──────────────┐     ┌────────────────┐     ┌───────────────┐     ┌──────────────┐     ┌─────────────┐
│ 1. Concept   │ ──► │ 2. Interactive │ ──► │ 3. Hands-On   │ ──► │ 4. Real-State│ ──► │ 5. Adaptive │
│ & Architecture│    │ Visualization  │     │ Lab / Terminal│     │ Grading      │     │ Skill Level │
└──────────────┘     └────────────────┘     └───────────────┘     └──────────────┘     └─────────────┘
```

1. **Concept & Architecture**: Real-world context, architectural tradeoffs, failure modes, and production best practices.
2. **Interactive Visualizations**: Dynamic SVG/Canvas animations where learners manipulate nodes, traffic flows, and packet routes.
3. **Hands-On Real Lab**: Learners open a real xterm.js terminal with direct access to an ephemeral Kubernetes sandbox and Monaco editor.
4. **Deterministic State Grading**: Validation engine queries the actual live cluster state to verify requirements.
5. **Adaptive Progression**: XP, mastery score (0-5), streak updates, and unlocking of prerequisite nodes in the Skill Tree.
