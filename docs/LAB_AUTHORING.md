# KubeLab Lab Authoring Guide

## Creating a New Lab

### 1. Create the lab directory
```bash
mkdir -p labs/<track-slug>/<lab-id>
```

### 2. Define `lab.yaml`
```yaml
id: "my-new-lab"
title: "My New Lab"
track: "kubernetes-core"
difficulty: "beginner"    # beginner | intermediate | advanced | expert
duration_minutes: 20
scenario: "Description of what the learner will accomplish"
tasks:
  - id: "task-1"
    title: "First Task"
    points: 50
    instructions: "Step-by-step instructions for the learner"
    hint: "A hint if they're stuck"
    solution: "The exact command or action to complete the task"
    validation:
      type: "kubernetes_state"
      assertions:
        - resource: "deployment"
          name: "my-app"
          namespace: "{{session_namespace}}"
          field: "spec.replicas"
          operator: "equals"
          expected: "3"
```

### 3. Validate your lab
```bash
cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path labs/
```

### 4. Test the lab
```bash
# Start a local Kind cluster
./scripts/k8s-up.ps1

# Run the lab manually and verify assertions
kubectl create namespace test-lab
# ... execute lab steps ...

# Tear down
./scripts/k8s-down.ps1
```

## Validation Operators

| Operator | Description | Example |
|---|---|---|
| `equals` | Exact match | `status.phase` equals `Running` |
| `not_equals` | Not equal | `status.phase` not_equals `Failed` |
| `contains` | Substring match | `metadata.labels` contains `app=web` |
| `exists` | Field exists | `spec.containers[0].resources.limits` exists |
| `greater_than` | Numeric comparison | `spec.replicas` greater_than `1` |
| `less_than` | Numeric comparison | `spec.replicas` less_than `10` |
| `regex` | Pattern match | `metadata.name` regex `^web-.*` |

## Best Practices

1. **Start simple**: One task per concept, build complexity gradually
2. **Use deterministic assertions**: Query K8s API state, not shell output
3. **Provide meaningful hints**: Guide without giving away the answer
4. **Include solutions**: Complete commands for each task
5. **Test on a clean cluster**: Ensure labs work from a fresh namespace
6. **Set realistic durations**: Time yourself completing the lab
7. **Use namespace templates**: `{{session_namespace}}` is replaced at runtime

## Debugging Labs

```bash
# Validate schema
cargo run -p kubelab-validation-engine --bin validate_lab_schema -- --path labs/my-track/my-lab

# Run validation engine tests
cargo test -p kubelab-validation-engine

# Check assertion logic
cargo test -p kubelab-validation-engine --test evaluator_negative_test
```
