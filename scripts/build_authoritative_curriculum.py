#!/usr/bin/env python3
"""
Authoritative Content & Lab Knowledge Base Generator for KubeLab
Contains deep, domain-specific architecture diagrams, manifests, gotchas, production tips,
and 10+ question banks per lesson for all 154 Kubernetes topics across 15 tracks.
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

# Import our content engine modules
from curriculum_knowledge_base import KNOWLEDGE_BASE, TRACK_META, generate_question_bank, generate_lab_definition

def build_all():
    print("--> Starting Authoritative Curriculum & Lab Compilation for 15 Tracks / 154 Labs...")
    
    tracks_data = []
    all_quizzes = {}
    all_lessons_flat = []
    lab_counts = 0

    for t_idx, track_meta in enumerate(TRACK_META, 1):
        track_slug = track_meta['slug']
        track_id = track_meta['id']
        track_dir = os.path.join(LABS_DIR, track_slug)
        
        modules = track_meta['modules']
        compiled_modules = []
        track_total_xp = 0
        track_total_lessons = 0
        
        for m_idx, mod_meta in enumerate(modules, 1):
            mod_id = mod_meta['id']
            mod_slug = mod_meta['slug']
            mod_lessons = []
            
            for l_idx, lesson_spec in enumerate(mod_meta['lessons'], 1):
                lab_id = lesson_spec['lab_id']
                kb = KNOWLEDGE_BASE.get(lab_id)
                if not kb:
                    raise ValueError(f"Missing knowledge base entry for lab: {lab_id}")
                
                # Update lab.yaml with authoritative tasks, validation, initial_state and solution
                lab_file_dir = os.path.join(track_dir, lab_id)
                os.makedirs(lab_file_dir, exist_ok=True)
                lab_yaml_path = os.path.join(lab_file_dir, 'lab.yaml')
                
                lab_data = generate_lab_definition(lab_id, kb, track_slug)
                with open(lab_yaml_path, 'w', encoding='utf-8') as f:
                    yaml.dump(lab_data, f, sort_keys=False, default_flow_style=False)
                lab_counts += 1
                
                # Generate 10+ Question Bank for this lesson
                question_bank = generate_question_bank(lab_id, kb, track_slug)
                quiz_id = f"quiz-{lab_id}"
                quiz_title = f"Assessment Bank • {kb['title']}"
                
                all_quizzes[quiz_id] = {
                    "id": quiz_id,
                    "lessonId": f"les-{lab_id}",
                    "trackSlug": track_slug,
                    "title": quiz_title,
                    "description": f"Comprehensive 10-question evaluation bank covering {kb['title']} architecture, syntax, CLI, and debugging.",
                    "questionCount": len(question_bank),
                    "questions": question_bank,
                    "passingScore": 80,
                    "xpReward": kb.get('xp', 150) // 2
                }
                
                lesson_obj = {
                    "id": f"les-{lab_id}",
                    "moduleId": mod_id,
                    "trackSlug": track_slug,
                    "title": kb['title'],
                    "slug": lab_id,
                    "order": l_idx,
                    "durationMinutes": kb.get('duration_minutes', 15),
                    "xp": kb.get('xp', 150),
                    "summary": kb['summary'],
                    "contentMarkdown": kb['content_markdown'],
                    "concepts": kb.get('concepts', [track_slug, lab_id]),
                    "prerequisites": kb.get('prerequisites', []),
                    "associatedLabId": lab_id,
                    "associatedQuizId": quiz_id,
                    "commonMistakes": kb['common_mistakes'],
                    "productionGuidance": kb['production_guidance'],
                    "diagram": kb['mermaid_diagram']
                }
                
                mod_lessons.append(lesson_obj)
                all_lessons_flat.append(lesson_obj)
                track_total_xp += lesson_obj['xp']
                track_total_lessons += 1
                
            compiled_modules.append({
                "id": mod_id,
                "trackId": track_id,
                "title": mod_meta['title'],
                "slug": mod_slug,
                "order": m_idx,
                "description": mod_meta['description'],
                "lessons": mod_lessons
            })
            
        tracks_data.append({
            "id": track_id,
            "slug": track_slug,
            "title": track_meta['title'],
            "description": track_meta['description'],
            "icon": track_meta['icon'],
            "color": track_meta['color'],
            "difficulty": track_meta['difficulty'],
            "order": t_idx,
            "totalLessons": track_total_lessons,
            "totalXp": track_total_xp,
            "modules": compiled_modules
        })

    # Master curriculum JSON
    curriculum_json = {
        "tracks": tracks_data,
        "quizzes": all_quizzes,
        "metadata": {
            "version": "2.0.0",
            "totalTracks": len(tracks_data),
            "totalModules": sum(len(t['modules']) for t in tracks_data),
            "totalLessons": len(all_lessons_flat),
            "totalQuizzes": len(all_quizzes),
            "totalQuestions": sum(len(q['questions']) for q in all_quizzes.values()),
            "totalLabs": lab_counts
        }
    }
    
    # Write packages/curriculum/src/curriculum.json
    os.makedirs(CURRICULUM_SRC, exist_ok=True)
    curriculum_json_path = os.path.join(CURRICULUM_SRC, 'curriculum.json')
    with open(curriculum_json_path, 'w', encoding='utf-8') as f:
        json.dump(curriculum_json, f, indent=2)
    print(f"[OK] Written {curriculum_json_path} ({len(all_lessons_flat)} lessons, {sum(len(q['questions']) for q in all_quizzes.values())} questions)")

    # Write apps/mobile/assets/data/curriculum.json
    os.makedirs(MOBILE_ASSETS, exist_ok=True)
    mobile_json_path = os.path.join(MOBILE_ASSETS, 'curriculum.json')
    with open(mobile_json_path, 'w', encoding='utf-8') as f:
        json.dump(curriculum_json, f, indent=2)
    print(f"[OK] Written {mobile_json_path}")

    # Write packages/curriculum/src/tracks.ts
    tracks_ts_path = os.path.join(CURRICULUM_SRC, 'tracks.ts')
    with open(tracks_ts_path, 'w', encoding='utf-8') as f:
        f.write("import { Track } from '@kubelab/shared-types';\nimport curriculumData from './curriculum.json';\n\n")
        f.write("export const TRACKS: Track[] = curriculumData.tracks as unknown as Track[];\n")
    print(f"[OK] Written {tracks_ts_path}")

    # Write packages/curriculum/src/registry.ts
    registry_ts_path = os.path.join(CURRICULUM_SRC, 'registry.ts')
    with open(registry_ts_path, 'w', encoding='utf-8') as f:
        f.write("""import curriculumData from './curriculum.json';
import { Track, Lesson, LessonQuiz } from '@kubelab/shared-types';

export const REGISTRY = {
  tracks: curriculumData.tracks as unknown as Track[],
  quizzes: curriculumData.quizzes as unknown as Record<string, LessonQuiz>,
  metadata: curriculumData.metadata
};

export function getTrackBySlug(slug: string): Track | undefined {
  return REGISTRY.tracks.find(t => t.slug === slug);
}

export function getLessonBySlug(trackSlug: string, lessonSlug: string): Lesson | undefined {
  const track = getTrackBySlug(trackSlug);
  if (!track) return undefined;
  for (const mod of track.modules) {
    const l = mod.lessons.find(l => l.slug === lessonSlug || l.id === lessonSlug || l.associatedLabId === lessonSlug);
    if (l) return l;
  }
  return undefined;
}

export function getQuizForLesson(lessonId: string): LessonQuiz | undefined {
  return Object.values(REGISTRY.quizzes).find(q => q.lessonId === lessonId || q.id === `quiz-${lessonId.replace(/^les-/, '')}`);
}

export function getQuizById(quizId: string): LessonQuiz | undefined {
  return REGISTRY.quizzes[quizId];
}
""")
    print(f"[OK] Written {registry_ts_path}")

    # Write packages/curriculum/src/index.ts
    index_ts_path = os.path.join(CURRICULUM_SRC, 'index.ts')
    with open(index_ts_path, 'w', encoding='utf-8') as f:
        f.write("""export * from './tracks';
export * from './registry';
export { default as curriculumData } from './curriculum.json';
""")
    print(f"[OK] Written {index_ts_path}")

    # Write apps/mobile/lib/data/curriculum_data.dart
    write_dart_curriculum(tracks_data, all_quizzes)
    print("--> Curriculum & Lab Generation Completed Successfully!")

def write_dart_curriculum(tracks_data, all_quizzes):
    dart_path = os.path.join(MOBILE_LIB, 'data', 'curriculum_data.dart')
    os.makedirs(os.path.dirname(dart_path), exist_ok=True)
    
    with open(dart_path, 'w', encoding='utf-8') as f:
        f.write("""// Generated by scripts/generate_authoritative_curriculum.py - DO NOT EDIT DIRECTLY
import 'dart:convert';

class MobileQuestion {
  final String id;
  final String prompt;
  final List<String> options;
  final int correctIndex;
  final String explanation;
  final String category;

  const MobileQuestion({
    required this.id,
    required this.prompt,
    required this.options,
    required this.correctIndex,
    required this.explanation,
    this.category = 'concept',
  });

  Map<String, dynamic> toJson() => {
    'id': id,
    'prompt': prompt,
    'options': options,
    'correctIndex': correctIndex,
    'explanation': explanation,
    'category': category,
  };

  factory MobileQuestion.fromJson(Map<String, dynamic> json) => MobileQuestion(
    id: json['id'] as String,
    prompt: json['prompt'] as String,
    options: List<String>.from(json['options'] as List),
    correctIndex: json['correctIndex'] as int,
    explanation: json['explanation'] as String,
    category: json['category'] as String? ?? 'concept',
  );
}

class MobileLessonQuiz {
  final String id;
  final String lessonId;
  final String trackSlug;
  final String title;
  final String description;
  final int questionCount;
  final List<MobileQuestion> questions;
  final int passingScore;
  final int xpReward;

  const MobileLessonQuiz({
    required this.id,
    required this.lessonId,
    required this.trackSlug,
    required this.title,
    required this.description,
    required this.questionCount,
    required this.questions,
    this.passingScore = 80,
    this.xpReward = 75,
  });

  Map<String, dynamic> toJson() => {
    'id': id,
    'lessonId': lessonId,
    'trackSlug': trackSlug,
    'title': title,
    'description': description,
    'questionCount': questionCount,
    'questions': questions.map((q) => q.toJson()).toList(),
    'passingScore': passingScore,
    'xpReward': xpReward,
  };

  factory MobileLessonQuiz.fromJson(Map<String, dynamic> json) => MobileLessonQuiz(
    id: json['id'] as String,
    lessonId: json['lessonId'] as String,
    trackSlug: json['trackSlug'] as String,
    title: json['title'] as String,
    description: json['description'] as String,
    questionCount: json['questionCount'] as int? ?? (json['questions'] as List).length,
    questions: (json['questions'] as List).map((q) => MobileQuestion.fromJson(q as Map<String, dynamic>)).toList(),
    passingScore: json['passingScore'] as int? ?? 80,
    xpReward: json['xpReward'] as int? ?? 75,
  );
}

class MobileLesson {
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
  final String diagram;

  const MobileLesson({
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
    this.diagram = '',
  });

  Map<String, dynamic> toJson() => {
    'id': id,
    'moduleId': moduleId,
    'trackSlug': trackSlug,
    'title': title,
    'slug': slug,
    'order': order,
    'durationMinutes': durationMinutes,
    'xp': xp,
    'summary': summary,
    'contentMarkdown': contentMarkdown,
    'concepts': concepts,
    'prerequisites': prerequisites,
    'associatedLabId': associatedLabId,
    'associatedQuizId': associatedQuizId,
    'commonMistakes': commonMistakes,
    'productionGuidance': productionGuidance,
    'diagram': diagram,
  };

  factory MobileLesson.fromJson(Map<String, dynamic> json) => MobileLesson(
    id: json['id'] as String,
    moduleId: json['moduleId'] as String,
    trackSlug: json['trackSlug'] as String,
    title: json['title'] as String,
    slug: json['slug'] as String,
    order: json['order'] as int,
    durationMinutes: json['durationMinutes'] as int,
    xp: json['xp'] as int,
    summary: json['summary'] as String,
    contentMarkdown: json['contentMarkdown'] as String,
    concepts: List<String>.from(json['concepts'] as List),
    prerequisites: List<String>.from(json['prerequisites'] as List),
    associatedLabId: json['associatedLabId'] as String,
    associatedQuizId: json['associatedQuizId'] as String,
    commonMistakes: List<String>.from(json['commonMistakes'] as List),
    productionGuidance: json['productionGuidance'] as String,
    diagram: json['diagram'] as String? ?? '',
  );
}

class MobileModule {
  final String id;
  final String trackId;
  final String title;
  final String slug;
  final int order;
  final String description;
  final List<MobileLesson> lessons;

  const MobileModule({
    required this.id,
    required this.trackId,
    required this.title,
    required this.slug,
    required this.order,
    required this.description,
    required this.lessons,
  });

  Map<String, dynamic> toJson() => {
    'id': id,
    'trackId': trackId,
    'title': title,
    'slug': slug,
    'order': order,
    'description': description,
    'lessons': lessons.map((l) => l.toJson()).toList(),
  };

  factory MobileModule.fromJson(Map<String, dynamic> json) => MobileModule(
    id: json['id'] as String,
    trackId: json['trackId'] as String,
    title: json['title'] as String,
    slug: json['slug'] as String,
    order: json['order'] as int,
    description: json['description'] as String,
    lessons: (json['lessons'] as List).map((l) => MobileLesson.fromJson(l as Map<String, dynamic>)).toList(),
  );
}

class MobileTrack {
  final String id;
  final String slug;
  final String title;
  final String description;
  final String icon;
  final String color;
  final String difficulty;
  final int order;
  final int totalLessons;
  final int totalXp;
  final List<MobileModule> modules;

  String get colorHex => color;

  const MobileTrack({
    required this.id,
    required this.slug,
    required this.title,
    required this.description,
    required this.icon,
    required this.color,
    required this.difficulty,
    required this.order,
    required this.totalLessons,
    required this.totalXp,
    required this.modules,
  });

  Map<String, dynamic> toJson() => {
    'id': id,
    'slug': slug,
    'title': title,
    'description': description,
    'icon': icon,
    'color': color,
    'difficulty': difficulty,
    'order': order,
    'totalLessons': totalLessons,
    'totalXp': totalXp,
    'modules': modules.map((m) => m.toJson()).toList(),
  };

  factory MobileTrack.fromJson(Map<String, dynamic> json) => MobileTrack(
    id: json['id'] as String,
    slug: json['slug'] as String,
    title: json['title'] as String,
    description: json['description'] as String,
    icon: json['icon'] as String,
    color: json['color'] as String,
    difficulty: json['difficulty'] as String,
    order: json['order'] as int,
    totalLessons: json['totalLessons'] as int,
    totalXp: json['totalXp'] as int,
    modules: (json['modules'] as List).map((m) => MobileModule.fromJson(m as Map<String, dynamic>)).toList(),
  );
}

class CurriculumRepository {
  static List<MobileTrack> tracks = [];
  static Map<String, MobileLessonQuiz> quizzes = {};

  static void initializeFromJson(String rawJson) {
    final data = jsonDecode(rawJson) as Map<String, dynamic>;
    final trackList = (data['tracks'] as List)
        .map((t) => MobileTrack.fromJson(t as Map<String, dynamic>))
        .toList();
    tracks = trackList;

    final quizMap = (data['quizzes'] as Map<String, dynamic>).map(
      (k, v) => MapEntry(k, MobileLessonQuiz.fromJson(v as Map<String, dynamic>)),
    );
    quizzes = quizMap;
  }

  static MobileTrack? getTrackBySlug(String slug) {
    try {
      return tracks.firstWhere((t) => t.slug == slug);
    } catch (_) {
      return null;
    }
  }

  static MobileLesson? getLesson(String trackSlug, String lessonSlug) {
    final track = getTrackBySlug(trackSlug);
    if (track == null) return null;
    for (final mod in track.modules) {
      for (final l in mod.lessons) {
        if (l.slug == lessonSlug || l.id == lessonSlug || l.associatedLabId == lessonSlug) {
          return l;
        }
      }
    }
    return null;
  }

  static MobileLessonQuiz? getQuizForLesson(String lessonId) {
    final cleanId = lessonId.replaceFirst('les-', '');
    return quizzes['quiz-$cleanId'] ??
        quizzes.values.cast<MobileLessonQuiz?>().firstWhere(
              (q) => q?.lessonId == lessonId,
              orElse: () => null,
            );
  }

  static MobileLessonQuiz? getQuizById(String quizId) {
    return quizzes[quizId];
  }
}
""")
    print(f"[OK] Written {dart_path}")

if __name__ == '__main__':
    build_all()
