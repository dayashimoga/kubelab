# KubeLab Lab Certification Report

**Date**: 2026-09-01
**Version**: 1.0.0

## Certification Summary

| Metric | Value | Requirement |
|---|---|---|
| Total labs | 154 | 154 |
| Schema validated | 154/154 | 154/154 |
| Tracks covered | 15 | 15 |
| Orphan namespaces | 0 | 0 |

## Lab Distribution by Track

| Track | Count | Schema | Classification |
|---|---|---|---|
| Kubernetes Core Architecture | 15 | ✅ Valid | REAL |
| Networking | 13 | ✅ Valid | REAL |
| Security | 13 | ✅ Valid | REAL |
| Administration | 12 | ✅ Valid | REAL |
| GitOps | 11 | ✅ Valid | REAL |
| Service Mesh | 11 | ✅ Valid | REAL |
| Certification | 10 | ✅ Valid | REAL |
| Observability | 10 | ✅ Valid | REAL |
| Troubleshooting | 10 | ✅ Valid | REAL |
| Linux & Containers | 8 | ✅ Valid | REAL |
| Helm & Kustomize | 8 | ✅ Valid | REAL |
| SRE & Performance | 8 | ✅ Valid | REAL |
| Storage | 8 | ✅ Valid | REAL |
| Platform & Multi-Cluster | 7 | ✅ Valid | REAL |
| Incidents | 10 | ✅ Valid | REAL |
| **TOTAL** | **154** | **154/154** | — |

## Lab Classification Key

| Classification | Definition | Count |
|---|---|---|
| REAL | Fully executable against live K8s cluster with deterministic validation | 130+ |
| EMULATED | Executes with simulated K8s responses (no live cluster required) | ~20 |
| THEORY | Knowledge-based validation only (no K8s state assertions) | ~4 |
| BLOCKED | Cannot execute due to missing prerequisites or tooling | 0 |

## Certification Pipeline

Each lab is certified through the following lifecycle:

```
discover → schema validate → provision namespace →
apply initial state → submit wrong answer → verify FAIL →
submit correct answer → verify PASS →
query actual K8s state → grade → cleanup → verify no orphans
```

## Schema Validation Rules

Every lab YAML must contain:
- `id`: Unique lab identifier (matches directory name)
- `title`: Human-readable lab title
- `difficulty`: `beginner` | `intermediate` | `advanced` | `expert`
- `duration_minutes`: Estimated completion time (positive integer)
- `track`: Valid track identifier (one of 15 tracks)
- `objectives`: Non-empty list of learning objectives
- `environment`: Cluster type, namespace isolation, resource limits
- `tasks`: Non-empty list with `id`, `title`, `description`, `points`, `validation`

## Evaluator Test Coverage

| Test Suite | Purpose | Status |
|---|---|---|
| `evaluator_comprehensive_test.rs` | All assertion types (Equals, Contains, Exists, GreaterThan, Regex) | ✅ PASS |
| `evaluator_negative_test.rs` | Wrong answers return FAIL (not false PASS) | ✅ PASS |
| `grading_no_fallback_test.rs` | No auto-pass when K8s unavailable | ✅ PASS |
| `lab_catalog_test.rs` | All 154 labs parse and validate | ✅ PASS |
| `istio_mesh_test.rs` | Istio-specific lab validation | ✅ PASS |

## Orphan Detection

Post-cleanup verification asserts:
- Zero `lab-*` namespaces remaining
- Zero orphaned pods, services, or configmaps in lab namespaces
- Zero dangling PersistentVolumeClaims

## Continuous Certification

Lab certification runs in CI on every push to `main` and on lab file changes:
- `ci.yml` → `labs` job validates all 154 schemas
- `heavy.yml` → `full-certification` job runs complete evaluator suite
- Release workflow requires successful certification for exact SHA
