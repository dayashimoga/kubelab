# Lab Authoring SDK & Specification Schema

Every lab is defined in a declarative `lab.yaml` file validated against the KubeLab JSON Schema.

## Schema Structure

```yaml
id: "k8s-pod-basics"
title: "Create and Configure Your First Pod"
difficulty: "beginner"            # beginner | intermediate | advanced | expert
duration_minutes: 15
track: "kubernetes"
prerequisites:
  - "linux-cli-basics"

environment:
  type: "kubernetes"              # kubernetes | podman | multi-node
  cluster: "disposable"
  namespace_isolation: true
  resources:
    cpu_limit: "500m"
    memory_limit: "512Mi"

initial_state:
  manifests: []

scenario: |
  A microservice requires a stateless web container running in the cluster.
  Your task is to create a Pod named `web-server` running `nginx:alpine`
  listening on port 80 with the label `app=frontend`.

tasks:
  - id: "task-1"
    title: "Create the web-server Pod"
    points: 50
    validation:
      type: "k8s_resource"
      resource: "pods"
      name: "web-server"
      assertions:
        - field: "status.phase"
          operator: "equals"
          expected: "Running"
        - field: "metadata.labels.app"
          operator: "equals"
          expected: "frontend"

  - id: "task-2"
    title: "Verify Container Port"
    points: 50
    validation:
      type: "k8s_resource"
      resource: "pods"
      name: "web-server"
      assertions:
        - field: "spec.containers[0].ports[0].containerPort"
          operator: "equals"
          expected: 80

hints:
  - text: "You can use `kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend`"
    penalty_points: 15

solution: |
  kubectl run web-server --image=nginx:alpine --port=80 --labels=app=frontend --restart=Always

cleanup:
  auto: true

limits:
  max_attempts: 5
  timeout_minutes: 20
```

## Validation Operators

- `equals`: Exact value match (string, number, boolean)
- `contains`: Array or string contains substring/item
- `matches_regex`: Regular expression pattern match
- `greater_than` / `less_than`: Numeric threshold checks
- `http_get`: HTTP status code and response payload match against container endpoint
