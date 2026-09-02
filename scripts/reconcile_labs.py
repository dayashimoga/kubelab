#!/usr/bin/env python3
"""
KubeLab Lab Catalog Reconciliation Script

Reconciles the authoritative curriculum.json (154 lessons across 15 tracks)
against the labs/ directory (287 lab.yaml files across 16 track directories).

Produces:
  - LEARNER_LABS: count of curriculum-mapped labs
  - AUX_MANIFESTS: count of non-curriculum labs (kept, not deleted)
  - DUPLICATES: labs mapping to the same lesson ID (must be 0)
  - ORPHANS: curriculum lessons with no matching lab (must be 0)
  - Writes labs/lab_catalog.yaml with authoritative classification

Exit code 0 = all checks pass; non-zero = reconciliation failure.
"""

import json
import os
import sys
import yaml
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
ROOT_DIR = SCRIPT_DIR.parent
CURRICULUM_JSON = ROOT_DIR / "packages" / "curriculum" / "src" / "curriculum.json"
LABS_DIR = ROOT_DIR / "labs"
CATALOG_OUTPUT = LABS_DIR / "lab_catalog.yaml"

# platform-multicluster is absorbed into platform-eng per production decision
TRACK_ALIASES = {
    "platform-multicluster": "platform-eng",
}


def load_curriculum_lesson_ids():
    """Extract all lesson lab IDs from the authoritative curriculum.json."""
    with open(CURRICULUM_JSON, "r", encoding="utf-8") as f:
        data = json.load(f)

    tracks = data.get("tracks", [])
    lesson_ids = set()
    track_lesson_map = defaultdict(list)

    for track in tracks:
        track_id = track.get("id", "")
        for module in track.get("modules", []):
            for lesson in module.get("lessons", []):
                # Primary: associatedLabId is the canonical link to lab.yaml
                lid = (
                    lesson.get("associatedLabId")
                    or lesson.get("labId")
                    or lesson.get("lab_id")
                    or lesson.get("slug")
                    or lesson.get("id", "")
                )
                if lid:
                    lesson_ids.add(lid)
                    track_lesson_map[track_id].append(lid)

    return lesson_ids, track_lesson_map, len(tracks)


def scan_lab_yaml_files():
    """Scan labs/ directory for all lab.yaml files and extract their IDs."""
    lab_files = {}
    duplicates = []

    for yaml_file in sorted(LABS_DIR.rglob("lab.yaml")):
        try:
            with open(yaml_file, "r", encoding="utf-8") as f:
                lab_def = yaml.safe_load(f)
            if not lab_def or not isinstance(lab_def, dict):
                continue

            lab_id = lab_def.get("id", "")
            if not lab_id:
                continue

            rel_path = yaml_file.relative_to(ROOT_DIR).as_posix()

            if lab_id in lab_files:
                duplicates.append({
                    "lab_id": lab_id,
                    "file1": lab_files[lab_id]["path"],
                    "file2": rel_path,
                })
            else:
                lab_files[lab_id] = {
                    "path": rel_path,
                    "track": lab_def.get("track", "unknown"),
                    "title": lab_def.get("title", ""),
                    "difficulty": lab_def.get("difficulty", ""),
                    "tasks": len(lab_def.get("tasks", [])),
                }
        except Exception as e:
            print(f"  WARNING: Failed to parse {yaml_file}: {e}", file=sys.stderr)

    return lab_files, duplicates


def reconcile():
    """Main reconciliation logic."""
    print("=" * 70)
    print("  KUBELAB LAB CATALOG RECONCILIATION")
    print("=" * 70)

    # 1. Load curriculum
    print("\n--> Loading authoritative curriculum from curriculum.json...")
    lesson_ids, track_lesson_map, track_count = load_curriculum_lesson_ids()
    print(f"    Curriculum: {track_count} tracks, {len(lesson_ids)} lessons")

    # 2. Scan lab files
    print("\n--> Scanning labs/ directory for lab.yaml files...")
    lab_files, duplicates = scan_lab_yaml_files()
    print(f"    Found: {len(lab_files)} unique lab.yaml files")

    if duplicates:
        print(f"\n    [ERROR] {len(duplicates)} DUPLICATE lab IDs found:")
        for d in duplicates:
            print(f"      - {d['lab_id']}: {d['file1']} vs {d['file2']}")

    # 3. Classify: learner (curriculum-mapped) vs auxiliary
    learner_labs = {}
    auxiliary_labs = {}
    orphan_lessons = set()

    for lid in lesson_ids:
        if lid in lab_files:
            learner_labs[lid] = lab_files[lid]
        else:
            orphan_lessons.add(lid)

    for lab_id, info in lab_files.items():
        if lab_id not in lesson_ids:
            auxiliary_labs[lab_id] = info

    # 4. Report
    print(f"\n{'=' * 70}")
    print(f"  RECONCILIATION RESULTS")
    print(f"{'=' * 70}")
    print(f"  LEARNER_LABS    = {len(learner_labs)}")
    print(f"  AUX_MANIFESTS   = {len(auxiliary_labs)}")
    print(f"  DUPLICATES      = {len(duplicates)}")
    print(f"  ORPHANS         = {len(orphan_lessons)}")

    if orphan_lessons:
        print(f"\n  [ERROR] Curriculum lessons with NO matching lab.yaml:")
        for lid in sorted(orphan_lessons):
            print(f"    - {lid}")

    if auxiliary_labs:
        print(f"\n  [INFO] Auxiliary labs (not in curriculum, kept in place):")
        for lab_id in sorted(auxiliary_labs.keys()):
            info = auxiliary_labs[lab_id]
            print(f"    - {lab_id} ({info['path']})")

    # 5. Write catalog
    catalog = {
        "metadata": {
            "generated_by": "scripts/reconcile_labs.py",
            "curriculum_source": "packages/curriculum/src/curriculum.json",
            "total_learner_labs": len(learner_labs),
            "total_auxiliary_labs": len(auxiliary_labs),
            "total_lab_yamls": len(lab_files),
            "duplicates": len(duplicates),
            "orphans": len(orphan_lessons),
        },
        "learner_labs": {
            lid: {
                "path": info["path"],
                "track": info["track"],
                "title": info["title"],
                "classification": "learner",
            }
            for lid, info in sorted(learner_labs.items())
        },
        "auxiliary_labs": {
            lid: {
                "path": info["path"],
                "track": info["track"],
                "title": info["title"],
                "classification": "auxiliary",
            }
            for lid, info in sorted(auxiliary_labs.items())
        },
    }

    with open(CATALOG_OUTPUT, "w", encoding="utf-8") as f:
        yaml.dump(catalog, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    print(f"\n  Wrote authoritative catalog to: {CATALOG_OUTPUT.relative_to(ROOT_DIR)}")

    # 6. Exit status
    errors = len(duplicates) + len(orphan_lessons)
    if errors > 0:
        print(f"\n  [FAIL] {errors} reconciliation error(s). Fix before release.")
        return 1

    print(f"\n  [PASS] Lab catalog reconciliation complete. 0 errors.")
    return 0


if __name__ == "__main__":
    sys.exit(reconcile())
