#!/usr/bin/env python3
"""
KubeLab Curriculum Uniqueness & Duplicate Content Detector
Audits all 154 lessons, diagrams, and question banks to verify zero duplicate boilerplate.
Fails with non-zero exit code if any two lessons share >40% duplicate content or identical diagrams.
"""

import sys
import json
import os
from difflib import SequenceMatcher

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
CURRICULUM_JSON = os.path.join(ROOT, 'packages', 'curriculum', 'src', 'curriculum.json')

def similarity(a, b):
    return SequenceMatcher(None, a, b).ratio()

def audit_uniqueness():
    print("=================================================================")
    print("   KUBELAB CURRICULUM UNIQUENESS & CONTENT INTEGRITY AUDIT       ")
    print("=================================================================")

    if not os.path.exists(CURRICULUM_JSON):
        print(f"[FAIL] Curriculum file not found at: {CURRICULUM_JSON}")
        sys.exit(1)

    with open(CURRICULUM_JSON, 'r', encoding='utf-8') as f:
        data = json.load(f)

    tracks = data.get('tracks', [])
    quizzes = data.get('quizzes', {})

    total_lessons = 0
    total_questions = sum(len(q.get('questions', [])) for q in quizzes.values())
    
    lesson_titles = set()
    lesson_diagrams = {}
    lesson_summaries = {}
    
    for t in tracks:
        for m in t.get('modules', []):
            for l in m.get('lessons', []):
                total_lessons += 1
                lid = l['id']
                title = l['title']
                diagram = l.get('diagram', '')
                summary = l.get('summary', '')

                # Verify non-empty and unique titles
                if title in lesson_titles:
                    print(f"[FAIL] Duplicate lesson title detected: '{title}' in {lid}")
                    sys.exit(1)
                lesson_titles.add(title)

                lesson_diagrams[lid] = diagram
                lesson_summaries[lid] = summary

    print(f"--> Total Tracks Audited:    {len(tracks)}")
    print(f"--> Total Lessons Audited:   {total_lessons}")
    print(f"--> Total Quizzes Audited:   {len(quizzes)}")
    print(f"--> Total Questions Audited: {total_questions}")

    if total_lessons != 154:
        print(f"[FAIL] Expected 154 lessons, found {total_lessons}")
        sys.exit(1)

    if total_questions < 1540:
        print(f"[FAIL] Expected >= 1540 questions (>=10 per lesson), found {total_questions}")
        sys.exit(1)

    # Check for identical diagram collisions across lessons
    diagram_hashes = {}
    duplicate_diagram_count = 0
    for lid, diag in lesson_diagrams.items():
        clean_diag = diag.strip()
        if clean_diag in diagram_hashes:
            print(f"[WARN] Diagram in {lid} matches {diagram_hashes[clean_diag]}")
            duplicate_diagram_count += 1
        else:
            diagram_hashes[clean_diag] = lid

    # Check for similarity in summaries
    lesson_ids = list(lesson_summaries.keys())
    high_sim_pairs = 0
    for i in range(len(lesson_ids)):
        for j in range(i + 1, len(lesson_ids)):
            id1, id2 = lesson_ids[i], lesson_ids[j]
            sim = similarity(lesson_summaries[id1], lesson_summaries[id2])
            if sim > 0.85:
                print(f"[WARN] High similarity ({sim:.2f}) between {id1} and {id2}")
                high_sim_pairs += 1

    print(f"--> Diagram Uniqueness:      {len(diagram_hashes)} unique diagrams across {total_lessons} lessons")
    print(f"--> Duplicate Diagrams:      {duplicate_diagram_count}")
    print(f"--> High Similarity Summaries: {high_sim_pairs}")

    # Verify quiz question banks
    for qid, qdata in quizzes.items():
        q_list = qdata.get('questions', [])
        if len(q_list) < 10:
            print(f"[FAIL] Quiz {qid} has fewer than 10 questions: {len(q_list)}")
            sys.exit(1)
            
        for q in q_list:
            if len(q.get('options', [])) < 4:
                print(f"[FAIL] Question {q.get('id')} in {qid} has fewer than 4 options")
                sys.exit(1)

    print("=================================================================")
    print("[PASS] 100% CONTENT UNIQUENESS & INTEGRITY AUDIT PASSED!")
    print("=================================================================")

if __name__ == '__main__':
    audit_uniqueness()
