# Lab Authoring & Lifecycle Guide

## Declarative Lab Structure
Every lab is defined in a `lab.yaml` file containing:
- `id`: unique identifier
- `title`, `difficulty`, `duration_minutes`, `track`
- `objectives`: learning goals
- `environment`: cluster configuration and limits
- `initial_state`: manifests to apply before user starts
- `tasks`: list of objectives and `StateAssertion` rules

## Grading Assertions
- `Equals`: Exact match on JSONPath
- `Contains`: Substring or array element match
- `Exists`: Resource or field presence check
- `GreaterThan`: Numeric comparison
- `Regex`: Pattern matching
