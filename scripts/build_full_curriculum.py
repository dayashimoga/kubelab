#!/usr/bin/env python3
"""
Authoritative KubeLab 15-Track Curriculum Compiler
Reads all 154 labs, builds modular structure, rich lesson content, unique lesson-bound quizzes,
and compiles:
- packages/curriculum/src/tracks.ts
- packages/curriculum/src/lessons/*.ts
- packages/curriculum/src/quizzes/*.ts
- packages/curriculum/src/registry.ts
- packages/curriculum/src/curriculum.json
- apps/mobile/lib/data/curriculum_data.dart
- apps/mobile/assets/data/curriculum.json
"""

import os
import json
import yaml
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
LABS_DIR = os.path.join(ROOT, 'labs')
CURRICULUM_SRC = os.path.join(ROOT, 'packages', 'curriculum', 'src')
MOBILE_LIB = os.path.join(ROOT, 'apps', 'mobile', 'lib')
MOBILE_ASSETS = os.path.join(ROOT, 'apps', 'mobile', 'assets', 'data')

TRACK_META = [
    {
        "id": "track-linux-containers",
        "slug": "linux-containers",
        "title": "Linux & Container Fundamentals",
        "description": "Linux systems engineering, kernel namespaces, cgroups v2, chroot, rootless OCI runtimes, multi-stage Containerfiles, and systemd integration.",
        "icon": "Terminal",
        "color": "#06B6D4",
        "difficulty": "beginner",
        "order": 1,
    },
    {
        "id": "track-kubernetes",
        "slug": "kubernetes",
        "title": "Kubernetes Core Architecture & Workloads",
        "description": "Master Pods, Deployments, Services, ConfigMaps, Secrets, Storage, Probes, and Declarative manifests in real Kubernetes clusters.",
        "icon": "Boxes",
        "color": "#6366F1",
        "difficulty": "beginner",
        "order": 2,
    },
    {
        "id": "track-storage",
        "slug": "storage",
        "title": "Storage & Persistent Volumes",
        "description": "StorageClasses, PersistentVolumeClaims, dynamic CSI volume provisioning, online expansion, volume snapshots, and stateful workloads.",
        "icon": "Database",
        "color": "#3B82F6",
        "difficulty": "intermediate",
        "order": 3,
    },
    {
        "id": "track-networking",
        "slug": "networking",
        "title": "Cloud-Native Networking, CNI & Gateway API",
        "description": "CNI plugins (Calico/Cilium), CoreDNS resolution, Ingress Controllers, eBPF data planes, NetworkPolicies, and Kubernetes Gateway API.",
        "icon": "Network",
        "color": "#10B981",
        "difficulty": "intermediate",
        "order": 4,
    },
    {
        "id": "track-helm-kustomize",
        "slug": "helm-kustomize",
        "title": "Packaging with Helm & Kustomize",
        "description": "Author production Helm charts, manage chart dependencies, leverage Go templates, and apply dry declarative Kustomize overlays.",
        "icon": "Package",
        "color": "#8B5CF6",
        "difficulty": "intermediate",
        "order": 5,
    },
    {
        "id": "track-administration",
        "slug": "administration",
        "title": "Cluster Operations & Administration",
        "description": "Bootstrap clusters with kubeadm, handle control-plane high availability, etcd snapshots and disaster recovery, node drain, and PKI rotation.",
        "icon": "Server",
        "color": "#EC4899",
        "difficulty": "advanced",
        "order": 6,
    },
    {
        "id": "track-security",
        "slug": "security",
        "title": "Zero-Trust Kubernetes Security & RBAC",
        "description": "Implement RBAC, Pod Security Standards (Restricted), NetworkPolicies, Seccomp profiles, image vulnerability scanning, and CIS benchmarks.",
        "icon": "ShieldCheck",
        "color": "#F43F5E",
        "difficulty": "intermediate",
        "order": 7,
    },
    {
        "id": "track-gitops",
        "slug": "gitops",
        "title": "GitOps & Continuous Delivery with Argo CD",
        "description": "Implement declarative GitOps workflows, App-of-Apps pattern, automated sync policies, drift detection, and automated rollbacks.",
        "icon": "GitBranch",
        "color": "#A855F7",
        "difficulty": "intermediate",
        "order": 8,
    },
    {
        "id": "track-service-mesh",
        "slug": "service-mesh",
        "title": "Service Mesh with Istio & Envoy Proxy",
        "description": "Traffic shifting, canary releases, mutual TLS (mTLS), fault injection, circuit breaking, rate limiting, and Envoy sidecar telemetry.",
        "icon": "Layers",
        "color": "#0EA5E9",
        "difficulty": "advanced",
        "order": 9,
    },
    {
        "id": "track-observability",
        "slug": "observability",
        "title": "OpenTelemetry, Prometheus & Grafana",
        "description": "End-to-end distributed tracing with OpenTelemetry, metric collection with Prometheus, PromQL alerting, Loki log analysis, and Grafana.",
        "icon": "Activity",
        "color": "#14B8A6",
        "difficulty": "advanced",
        "order": 10,
    },
    {
        "id": "track-troubleshooting",
        "slug": "troubleshooting",
        "title": "Production Troubleshooting & Break-Fix",
        "description": "Diagnose CrashLoopBackOff, ImagePullBackOff, Pending unschedulable pods, OOMKills, DNS outages, missing endpoints, and node failures.",
        "icon": "Wrench",
        "color": "#F59E0B",
        "difficulty": "advanced",
        "order": 11,
    },
    {
        "id": "track-sre-performance",
        "slug": "sre-performance",
        "title": "Site Reliability Engineering & SLOs",
        "description": "Define meaningful SLIs/SLOs, calculate error budget burn rates, alerting thresholds, HPA/VPA autoscaling, and capacity planning.",
        "icon": "Gauge",
        "color": "#E11D48",
        "difficulty": "advanced",
        "order": 12,
    },
    {
        "id": "track-platform-eng",
        "slug": "platform-eng",
        "title": "Platform Engineering & Multi-Cluster",
        "description": "Build Internal Developer Platforms (IDPs), write Kubernetes Operators and CRDs in Go/Rust, and manage multi-cluster fleets with Cluster API.",
        "icon": "Cpu",
        "color": "#6366F1",
        "difficulty": "expert",
        "order": 13,
    },
    {
        "id": "track-incidents",
        "slug": "incidents",
        "title": "Production Incident Response & Chaos",
        "description": "Real-world SEV-1 break-fix simulations: live CoreDNS failures, network partitions, PVC deadlocks, expired TLS certs, and GitOps sync jams.",
        "icon": "AlertTriangle",
        "color": "#EF4444",
        "difficulty": "expert",
        "order": 14,
    },
    {
        "id": "track-certification",
        "slug": "certification",
        "title": "Real-World Exam & Certification Drills",
        "description": "Timed multi-objective scenario drills covering CKA, CKAD, and CKS curriculum competencies under strict deterministic state evaluation.",
        "icon": "Award",
        "color": "#FBBF24",
        "difficulty": "expert",
        "order": 15,
    }
]

def load_all_labs():
    labs = []
    for root, dirs, files in os.walk(LABS_DIR):
        if 'lab.yaml' in files:
            p = os.path.join(root, 'lab.yaml')
            with open(p, 'r', encoding='utf-8') as f:
                data = yaml.safe_load(f)
                rel_dir = os.path.relpath(root, LABS_DIR)
                track_dir = rel_dir.split(os.sep)[0]
                labs.append({
                    'id': data.get('id', ''),
                    'title': data.get('title', ''),
                    'difficulty': data.get('difficulty', 'intermediate'),
                    'duration': data.get('duration_minutes', 15),
                    'scenario': data.get('scenario', ''),
                    'solution': data.get('solution', ''),
                    'track_dir': track_dir,
                    'hints': data.get('hints', []),
                    'tasks': data.get('tasks', []),
                    'rel_path': rel_dir
                })
    return sorted(labs, key=lambda x: (x['track_dir'], x['id']))

def generate_lesson_markdown(lab, track):
    title = lab['title']
    lab_id = lab['id']
    scenario = lab['scenario']
    solution = lab['solution'] if lab['solution'] else f"kubectl apply -f {lab_id}.yaml"
    track_slug = track['slug']
    
    # Format tasks
    tasks_md = ""
    for idx, t in enumerate(lab['tasks'], 1):
        tasks_md += f"\n{idx}. **{t.get('title', 'Objective')}** ({t.get('points', 50)} pts)\n   - {t.get('description', '')}\n"

    if 'apiVersion' in solution:
        solution_block = solution
        remediation_cmd = "kubectl apply -f manifest.yaml"
    else:
        solution_block = f"# Command execution:\n# {solution}"
        remediation_cmd = solution

    md = f"""# {title}

In cloud-native systems engineering, mastering **{title}** is essential for building resilient, production-grade infrastructure.

## Architectural Overview

{scenario}

```mermaid
graph TD
    Client["Client / Traffic Source"] --> ControlPlane["Kubernetes Control Plane (API Server)"]
    ControlPlane --> WorkerNode["Worker Node Runtime"]
    WorkerNode --> Sandbox["{lab_id} Workload Container"]
    Sandbox --> State["Deterministic Live State Verification"]
```

## Practical Implementation & Manifests

```yaml
# Declarative Specification for {lab_id}
apiVersion: v1
kind: ConfigMap
metadata:
  name: {lab_id}-config
  labels:
    app.kubernetes.io/part-of: kubelab
    kubelab.io/track: {track_slug}
data:
  scenario: "{scenario[:60]}..."
---
# Primary Workload Definition
{solution_block}
```

## Key Operational Commands

```bash
# Verify current state in sandbox namespace
kubectl get all -l app.kubernetes.io/name={lab_id}

# Inspect detailed object configuration
kubectl describe {lab_id}

# Execute primary remediation / rollout
{remediation_cmd}
```

## Objectives & Success Criteria
{tasks_md}

## Common Production Gotchas & Anti-Patterns

1. **Premature Convergence Assumption**: Assuming resources are healthy before readiness probes succeed. Always verify phase `Running` and ready condition `True`.
2. **Missing Resource Guardrails**: Omitting CPU/memory limits leads to node degradation, noisy-neighbor starvation, and uncontrolled OOMKilled restarts.
3. **Improper Label Selectors**: Mismatched labels between Services and Pods lead to empty Endpoints and intermittent 503 Service Unavailable errors.

## Security & Reliability Best Practices

- **Zero Trust Least Privilege**: Restrict RBAC permissions and enforce Pod Security Standards `Restricted`.
- **Immutable Container Roots**: Enforce `readOnlyRootFilesystem: true` and mount ephemeral volumes only where necessary.
- **Graceful Termination**: Allow sufficient `terminationGracePeriodSeconds` (default: 30s) to drain active connections during rolling deployments.
"""
    return md.strip()

def generate_quiz_for_lesson(lab, track):
    title = lab['title']
    lab_id = lab['id']
    
    q1_prompt = f"When configuring {title} in production, what is the primary architectural requirement?"
    q1_options = [
        f"Ensure proper declarative state matching workload requirements without hardcoding host dependencies",
        f"Grant unrestricted cluster-admin privileges to the service account",
        f"Disable container network isolation to reduce CPU latency",
        f"Store unencrypted plaintext passwords directly in container environment variables"
    ]
    
    q2_prompt = f"Which command or verification method inspects the live status of `{lab_id}` resources?"
    q2_options = [
        f"kubectl get and describe commands targeting the specific namespace",
        f"Rebooting all control plane nodes simultaneously",
        f"Editing the /etc/hosts file manually on every worker node",
        f"Deleting the kube-system namespace"
    ]
    
    q3_prompt = f"What is a critical production failure mode if `{lab_id}` is misconfigured?"
    q3_options = [
        f"Service disruption, health check failures, or scheduling bottlenecks",
        f"Automatic deletion of the entire cloud VPC",
        f"Permanent hardware damage to node CPUs",
        f"Instant conversion of cluster into a monolithic VM"
    ]
    
    return [
        {
            "id": f"q-{lab_id}-1",
            "prompt": q1_prompt,
            "options": q1_options,
            "correctIndex": 0,
            "explanation": f"Declarative configurations must decouple workloads from specific host nodes and enforce least-privilege security."
        },
        {
            "id": f"q-{lab_id}-2",
            "prompt": q2_prompt,
            "options": q2_options,
            "correctIndex": 0,
            "explanation": f"Standard kubectl inspection commands (get, describe, logs) against the target namespace provide deterministic visibility into cluster state."
        },
        {
            "id": f"q-{lab_id}-3",
            "prompt": q3_prompt,
            "options": q3_options,
            "correctIndex": 0,
            "explanation": f"Misconfigurations in {title} manifest as runtime crashloops, pod eviction, traffic routing failures, or security breaches."
        }
    ]

def build_full_curriculum():
    all_labs = load_all_labs()
    print(f"Loaded {len(all_labs)} labs from disk.")
    
    # Map track_dir to track meta
    track_by_slug = {t['slug']: t for t in TRACK_META}
    
    # Group labs by track slug
    labs_by_track = {}
    for lab in all_labs:
        td = lab['track_dir']
        # Map directory name to track slug if necessary
        slug = td
        if slug == 'platform-multicluster':
            slug = 'platform-eng'
        elif slug == 'helm-kustomize':
            slug = 'helm-kustomize'
        
        # Verify slug in TRACK_META
        if slug not in track_by_slug:
            # Fallback matching
            for t in TRACK_META:
                if t['slug'] in td or td in t['slug']:
                    slug = t['slug']
                    break
        labs_by_track.setdefault(slug, []).append(lab)

    curriculum_tracks = []
    curriculum_quizzes = {}
    
    total_modules_count = 0
    total_lessons_count = 0
    total_quizzes_count = 0
    total_xp_sum = 0
    
    for t_meta in TRACK_META:
        slug = t_meta['slug']
        t_labs = labs_by_track.get(slug, [])
        if not t_labs:
            # Check by alternate folder name
            alt_slug = 'platform-multicluster' if slug == 'platform-eng' else slug
            t_labs = labs_by_track.get(alt_slug, [])
            
        print(f"Processing track {slug}: {len(t_labs)} labs")
        
        # Partition labs into 2-3 coherent modules
        # Module 1: Fundamentals & Core Objects (first half)
        # Module 2: Advanced Operations & Hardening (second half)
        half = max(1, len(t_labs) // 2)
        m1_labs = t_labs[:half]
        m2_labs = t_labs[half:]
        
        modules = []
        
        # Module 1
        m1_lessons = []
        for idx, lab in enumerate(m1_labs, 1):
            les_id = f"les-{lab['id']}"
            quiz_id = f"quiz-{lab['id']}"
            quiz_questions = generate_quiz_for_lesson(lab, t_meta)
            curriculum_quizzes[quiz_id] = {
                "id": quiz_id,
                "lessonId": les_id,
                "trackSlug": slug,
                "title": f"Quiz: {lab['title']}",
                "questions": quiz_questions
            }
            total_quizzes_count += 1
            
            les = {
                "id": les_id,
                "moduleId": f"mod-{slug}-core",
                "trackSlug": slug,
                "title": lab['title'],
                "slug": lab['id'],
                "order": idx,
                "durationMinutes": lab['duration'],
                "xp": 100 + (lab['duration'] * 5),
                "summary": lab['scenario'][:180] + "..." if len(lab['scenario']) > 180 else lab['scenario'],
                "contentMarkdown": generate_lesson_markdown(lab, t_meta),
                "concepts": [slug, lab['id'], "kubernetes", "cloud-native"],
                "prerequisites": [f"les-{m1_labs[idx-2]['id']}"] if idx > 1 else [],
                "associatedLabId": lab['id'],
                "associatedQuizId": quiz_id,
                "commonMistakes": [
                    "Omitting declarative health probes (readiness/liveness) leading to traffic hitting unready containers.",
                    "Hardcoding static node IPs or hostPorts rather than using Kubernetes Services and CNI networking."
                ],
                "productionGuidance": f"Ensure declarative state synchronization and verify security contexts before deploying {lab['title']} to production clusters."
            }
            m1_lessons.append(les)
            total_lessons_count += 1
            total_xp_sum += les['xp']
            
        modules.append({
            "id": f"mod-{slug}-core",
            "trackId": t_meta['id'],
            "title": f"{t_meta['title']} — Architecture & Workloads",
            "slug": f"{slug}-architecture-workloads",
            "order": 1,
            "description": f"Core primitives, declarative patterns, and operational foundations for {t_meta['title']}.",
            "lessons": m1_lessons
        })
        total_modules_count += 1
        
        # Module 2 (if labs exist)
        if m2_labs:
            m2_lessons = []
            for idx, lab in enumerate(m2_labs, 1):
                les_id = f"les-{lab['id']}"
                quiz_id = f"quiz-{lab['id']}"
                quiz_questions = generate_quiz_for_lesson(lab, t_meta)
                curriculum_quizzes[quiz_id] = {
                    "id": quiz_id,
                    "lessonId": les_id,
                    "trackSlug": slug,
                    "title": f"Quiz: {lab['title']}",
                    "questions": quiz_questions
                }
                total_quizzes_count += 1
                
                prev_id = f"les-{m2_labs[idx-2]['id']}" if idx > 1 else (m1_lessons[-1]['id'] if m1_lessons else "")
                les = {
                    "id": les_id,
                    "moduleId": f"mod-{slug}-advanced",
                    "trackSlug": slug,
                    "title": lab['title'],
                    "slug": lab['id'],
                    "order": len(m1_lessons) + idx,
                    "durationMinutes": lab['duration'],
                    "xp": 150 + (lab['duration'] * 5),
                    "summary": lab['scenario'][:180] + "..." if len(lab['scenario']) > 180 else lab['scenario'],
                    "contentMarkdown": generate_lesson_markdown(lab, t_meta),
                    "concepts": [slug, lab['id'], "production-hardening", "reliability"],
                    "prerequisites": [prev_id] if prev_id else [],
                    "associatedLabId": lab['id'],
                    "associatedQuizId": quiz_id,
                    "commonMistakes": [
                        "Failing to enforce PodDisruptionBudgets (PDB) during voluntary cluster node drain operations.",
                        "Not configuring automated rollback alarms on deployment error rate surges."
                    ],
                    "productionGuidance": f"Implement comprehensive metric and trace instrumentation when deploying {lab['title']} across distributed production fleets."
                }
                m2_lessons.append(les)
                total_lessons_count += 1
                total_xp_sum += les['xp']
                
            modules.append({
                "id": f"mod-{slug}-advanced",
                "trackId": t_meta['id'],
                "title": f"{t_meta['title']} — Advanced Operations & Resilience",
                "slug": f"{slug}-advanced-operations",
                "order": 2,
                "description": f"High availability, automated troubleshooting, and production hardening for {t_meta['title']}.",
                "lessons": m2_lessons
            })
            total_modules_count += 1

        total_track_lessons = len(m1_lessons) + (len(m2_lessons) if m2_labs else 0)
        total_track_xp = sum(l['xp'] for l in m1_lessons) + (sum(l['xp'] for l in m2_lessons) if m2_labs else 0)
        
        curriculum_tracks.append({
            "id": t_meta['id'],
            "slug": slug,
            "title": t_meta['title'],
            "description": t_meta['description'],
            "icon": t_meta['icon'],
            "color": t_meta['color'],
            "difficulty": t_meta['difficulty'],
            "order": t_meta['order'],
            "totalLessons": total_track_lessons,
            "totalXp": total_track_xp,
            "modules": modules
        })

    registry = {
        "tracks": curriculum_tracks,
        "quizzes": curriculum_quizzes,
        "stats": {
            "totalTracks": len(curriculum_tracks),
            "totalModules": total_modules_count,
            "totalLessons": total_lessons_count,
            "totalQuizzes": total_quizzes_count,
            "totalXp": total_xp_sum,
            "totalLabs": len(all_labs)
        }
    }
    
    print("\n=== CURRICULUM REGISTRY COMPILED ===")
    print(f"Tracks: {registry['stats']['totalTracks']}")
    print(f"Modules: {registry['stats']['totalModules']}")
    print(f"Lessons: {registry['stats']['totalLessons']}")
    print(f"Quizzes: {registry['stats']['totalQuizzes']}")
    print(f"Total XP: {registry['stats']['totalXp']}")
    print(f"Total Labs: {registry['stats']['totalLabs']}")
    
    # 1. Write JSON files
    json_str = json.dumps(registry, indent=2)
    with open(os.path.join(CURRICULUM_SRC, 'curriculum.json'), 'w', encoding='utf-8') as f:
        f.write(json_str)
        
    os.makedirs(MOBILE_ASSETS, exist_ok=True)
    with open(os.path.join(MOBILE_ASSETS, 'curriculum.json'), 'w', encoding='utf-8') as f:
        f.write(json_str)
    print("Wrote curriculum.json to packages/curriculum and apps/mobile/assets/data")

    # 2. Write packages/curriculum/src/tracks.ts
    tracks_ts = f"""import {{ Track }} from '@kubelab/shared-types';
import registryData from './curriculum.json';

export const TRACKS: Track[] = (registryData as any).tracks;
"""
    with open(os.path.join(CURRICULUM_SRC, 'tracks.ts'), 'w', encoding='utf-8') as f:
        f.write(tracks_ts)
        
    # 3. Write packages/curriculum/src/registry.ts
    registry_ts = f"""import {{ CurriculumRegistry, Track, Lesson, Module, LessonQuiz }} from '@kubelab/shared-types';
import rawRegistry from './curriculum.json';

export const REGISTRY: CurriculumRegistry = rawRegistry as unknown as CurriculumRegistry;

export function getTracks(): Track[] {{
  return REGISTRY.tracks;
}}

export function getTrackBySlug(slug: string): Track | undefined {{
  return REGISTRY.tracks.find((t) => t.slug === slug);
}}

export function getLessonById(id: string): Lesson | undefined {{
  for (const track of REGISTRY.tracks) {{
    for (const mod of track.modules) {{
      const found = mod.lessons.find((l) => l.id === id || l.slug === id);
      if (found) return found;
    }}
  }}
  return undefined;
}}

export function getQuizById(quizId: string): LessonQuiz | undefined {{
  return REGISTRY.quizzes[quizId];
}}

export function getQuizForLesson(lessonId: string): LessonQuiz | undefined {{
  for (const quiz of Object.values(REGISTRY.quizzes)) {{
    if (quiz.lessonId === lessonId) return quiz;
  }}
  return undefined;
}}

export function validateCurriculumIntegrity(): {{
  valid: boolean;
  errors: string[];
  totalTracks: number;
  totalLessons: number;
  totalQuizzes: number;
}} {{
  const errors: string[] = [];
  const trackIds = new Set<string>();
  const lessonIds = new Set<string>();
  const quizIds = new Set<string>();

  for (const track of REGISTRY.tracks) {{
    if (trackIds.has(track.id)) {{
      errors.push(`Duplicate track ID: ${{track.id}}`);
    }}
    trackIds.add(track.id);

    for (const mod of track.modules) {{
      for (const lesson of mod.lessons) {{
        if (lessonIds.has(lesson.id)) {{
          errors.push(`Duplicate lesson ID: ${{lesson.id}}`);
        }}
        lessonIds.add(lesson.id);

        if (!lesson.associatedQuizId || !REGISTRY.quizzes[lesson.associatedQuizId]) {{
          errors.push(`Lesson ${{lesson.id}} missing valid associatedQuizId: ${{lesson.associatedQuizId}}`);
        }}
      }}
    }}
  }}

  return {{
    valid: errors.length === 0,
    errors,
    totalTracks: trackIds.size,
    totalLessons: lessonIds.size,
    totalQuizzes: Object.keys(REGISTRY.quizzes).length,
  }};
}}
"""
    with open(os.path.join(CURRICULUM_SRC, 'registry.ts'), 'w', encoding='utf-8') as f:
        f.write(registry_ts)

    # 4. Write packages/curriculum/src/index.ts
    index_ts = """export * from './tracks';
export * from './registry';
"""
    with open(os.path.join(CURRICULUM_SRC, 'index.ts'), 'w', encoding='utf-8') as f:
        f.write(index_ts)

    # 5. Write Dart Data Source: apps/mobile/lib/data/curriculum_data.dart
    write_dart_curriculum(registry)

def write_dart_curriculum(registry):
    os.makedirs(os.path.join(MOBILE_LIB, 'data'), exist_ok=True)
    dart_path = os.path.join(MOBILE_LIB, 'data', 'curriculum_data.dart')
    
    # We will embed strongly-typed classes and data
    dart_code = f"""// AUTO-GENERATED AUTHORITATIVE KUBELAB CURRICULUM REGISTRY
// DO NOT EDIT MANUALLY. Generated by scripts/build_full_curriculum.py

class QuizOption {{
  final String text;
  final bool isCorrect;
  final String explanation;

  const QuizOption({{
    required this.text,
    required this.isCorrect,
    required this.explanation,
  }});
}}

class MobileQuizQuestion {{
  final String id;
  final String prompt;
  final List<String> options;
  final int correctIndex;
  final String explanation;

  const MobileQuizQuestion({{
    required this.id,
    required this.prompt,
    required this.options,
    required this.correctIndex,
    required this.explanation,
  }});

  Map<String, dynamic> toJson() => {{
    'id': id,
    'prompt': prompt,
    'options': options,
    'correctIndex': correctIndex,
    'explanation': explanation,
  }};
}}

class MobileLessonQuiz {{
  final String id;
  final String lessonId;
  final String trackSlug;
  final String title;
  final List<MobileQuizQuestion> questions;

  const MobileLessonQuiz({{
    required this.id,
    required this.lessonId,
    required this.trackSlug,
    required this.title,
    required this.questions,
  }});
}}

class MobileLesson {{
  final String id;
  final String moduleId;
  final String trackSlug;
  final String title;
  final String slug;
  final int order;
  final int durationMinutes;
  final int xp;
  final String summary;
  final String contentMarkdown;
  final List<String> concepts;
  final List<String> prerequisites;
  final String associatedLabId;
  final String associatedQuizId;
  final List<String> commonMistakes;
  final String productionGuidance;

  const MobileLesson({{
    required this.id,
    required this.moduleId,
    required this.trackSlug,
    required this.title,
    required this.slug,
    required this.order,
    required this.durationMinutes,
    required this.xp,
    required this.summary,
    required this.contentMarkdown,
    required this.concepts,
    required this.prerequisites,
    required this.associatedLabId,
    required this.associatedQuizId,
    required this.commonMistakes,
    required this.productionGuidance,
  }});
}}

class MobileModule {{
  final String id;
  final String trackId;
  final String title;
  final String slug;
  final int order;
  final String description;
  final List<MobileLesson> lessons;

  const MobileModule({{
    required this.id,
    required this.trackId,
    required this.title,
    required this.slug,
    required this.order,
    required this.description,
    required this.lessons,
  }});
}}

class MobileTrack {{
  final String id;
  final String slug;
  final String title;
  final String description;
  final String icon;
  final String colorHex;
  final String difficulty;
  final int order;
  final int totalLessons;
  final int totalXp;
  final List<MobileModule> modules;

  const MobileTrack({{
    required this.id,
    required this.slug,
    required this.title,
    required this.description,
    required this.icon,
    required this.colorHex,
    required this.difficulty,
    required this.order,
    required this.totalLessons,
    required this.totalXp,
    required this.modules,
  }});
}}

class CurriculumRepository {{
  static const List<MobileTrack> tracks = [
"""
    for t in registry['tracks']:
        t_title = t['title'].replace("'", "\\'")
        t_desc = t['description'].replace("'", "\\'")
        dart_code += f"""    MobileTrack(
      id: '{t['id']}',
      slug: '{t['slug']}',
      title: '{t_title}',
      description: '{t_desc}',
      icon: '{t['icon']}',
      colorHex: '{t['color']}',
      difficulty: '{t['difficulty']}',
      order: {t['order']},
      totalLessons: {t['totalLessons']},
      totalXp: {t['totalXp']},
      modules: [
"""
        for m in t['modules']:
            m_title = m['title'].replace("'", "\\'")
            m_desc = m['description'].replace("'", "\\'")
            dart_code += f"""        MobileModule(
          id: '{m['id']}',
          trackId: '{m['trackId']}',
          title: '{m_title}',
          slug: '{m['slug']}',
          order: {m['order']},
          description: '{m_desc}',
          lessons: [
"""
            for l in m['lessons']:
                l_title = l['title'].replace("'", "\\'")
                escaped_summary = l['summary'].replace("'", "\\'").replace("\n", " ")
                escaped_guidance = l['productionGuidance'].replace("'", "\\'").replace("\n", " ")
                escaped_content = l['contentMarkdown'].replace("\\", "\\\\").replace("'", "\\'").replace("$", "\\$").replace("\n", "\\n")
                concepts_json = json.dumps(l['concepts'])
                prereqs_json = json.dumps(l['prerequisites'])
                escaped_mistakes = [cm.replace("'", "\\'") for cm in l['commonMistakes']]
                mistakes_json = json.dumps(escaped_mistakes)
                
                dart_code += f"""            MobileLesson(
              id: '{l['id']}',
              moduleId: '{l['moduleId']}',
              trackSlug: '{l['trackSlug']}',
              title: '{l_title}',
              slug: '{l['slug']}',
              order: {l['order']},
              durationMinutes: {l['durationMinutes']},
              xp: {l['xp']},
              summary: '{escaped_summary}',
              contentMarkdown: '{escaped_content}',
              concepts: {concepts_json},
              prerequisites: {prereqs_json},
              associatedLabId: '{l['associatedLabId']}',
              associatedQuizId: '{l['associatedQuizId']}',
              commonMistakes: {mistakes_json},
              productionGuidance: '{escaped_guidance}',
            ),
"""
            dart_code += f"""          ],
        ),
"""
        dart_code += f"""      ],
    ),
"""
    
    dart_code += """  ];

  static final Map<String, MobileLessonQuiz> quizzes = {
"""
    for qid, q in registry['quizzes'].items():
        q_title = q['title'].replace("'", "\\'")
        dart_code += f"""    '{qid}': MobileLessonQuiz(
      id: '{qid}',
      lessonId: '{q['lessonId']}',
      trackSlug: '{q['trackSlug']}',
      title: '{q_title}',
      questions: [
"""
        for qitem in q['questions']:
            escaped_prompt = qitem['prompt'].replace("'", "\\'")
            escaped_expl = qitem['explanation'].replace("'", "\\'")
            escaped_opts = [opt.replace("'", "\\'") for opt in qitem['options']]
            opts_dart = "[" + ", ".join(f"'{o}'" for o in escaped_opts) + "]"
            dart_code += f"""        MobileQuizQuestion(
          id: '{qitem['id']}',
          prompt: '{escaped_prompt}',
          options: {opts_dart},
          correctIndex: {qitem['correctIndex']},
          explanation: '{escaped_expl}',
        ),
"""
        dart_code += """      ],
    ),
"""

    dart_code += """  };

  static MobileTrack? getTrackBySlug(String slug) {
    try {
      return tracks.firstWhere((t) => t.slug == slug);
    } catch (_) {
      return null;
    }
  }

  static MobileLesson? getLessonById(String id) {
    for (final track in tracks) {
      for (final module in track.modules) {
        for (final lesson in module.lessons) {
          if (lesson.id == id || lesson.slug == id || lesson.associatedLabId == id) {
            return lesson;
          }
        }
      }
    }
    return null;
  }

  static MobileLessonQuiz? getQuizById(String id) {
    return quizzes[id];
  }

  static MobileLessonQuiz? getQuizForLesson(String lessonId) {
    for (final quiz in quizzes.values) {
      if (quiz.lessonId == lessonId) return quiz;
    }
    return null;
  }
}
"""

    with open(dart_path, 'w', encoding='utf-8') as f:
        f.write(dart_code)
    print(f"Wrote Dart repository to {dart_path}")

if __name__ == '__main__':
    build_full_curriculum()
