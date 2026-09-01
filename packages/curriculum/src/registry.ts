import { CurriculumRegistry, Track, Lesson, Module, LessonQuiz } from '@kubelab/shared-types';
import rawRegistry from './curriculum.json';

export const REGISTRY: CurriculumRegistry = rawRegistry as unknown as CurriculumRegistry;

export function getTracks(): Track[] {
  return REGISTRY.tracks;
}

export function getTrackBySlug(slug: string): Track | undefined {
  return REGISTRY.tracks.find((t) => t.slug === slug);
}

export function getLessonById(id: string): Lesson | undefined {
  for (const track of REGISTRY.tracks) {
    for (const mod of track.modules) {
      const found = mod.lessons.find((l) => l.id === id || l.slug === id);
      if (found) return found;
    }
  }
  return undefined;
}

export function getQuizById(quizId: string): LessonQuiz | undefined {
  return REGISTRY.quizzes[quizId];
}

export function getQuizForLesson(lessonId: string): LessonQuiz | undefined {
  for (const quiz of Object.values(REGISTRY.quizzes)) {
    if (quiz.lessonId === lessonId) return quiz;
  }
  return undefined;
}

export function validateCurriculumIntegrity(): {
  valid: boolean;
  errors: string[];
  totalTracks: number;
  totalLessons: number;
  totalQuizzes: number;
} {
  const errors: string[] = [];
  const trackIds = new Set<string>();
  const lessonIds = new Set<string>();
  const quizIds = new Set<string>();

  for (const track of REGISTRY.tracks) {
    if (trackIds.has(track.id)) {
      errors.push(`Duplicate track ID: ${track.id}`);
    }
    trackIds.add(track.id);

    for (const mod of track.modules) {
      for (const lesson of mod.lessons) {
        if (lessonIds.has(lesson.id)) {
          errors.push(`Duplicate lesson ID: ${lesson.id}`);
        }
        lessonIds.add(lesson.id);

        if (!lesson.associatedQuizId || !REGISTRY.quizzes[lesson.associatedQuizId]) {
          errors.push(`Lesson ${lesson.id} missing valid associatedQuizId: ${lesson.associatedQuizId}`);
        }
      }
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    totalTracks: trackIds.size,
    totalLessons: lessonIds.size,
    totalQuizzes: Object.keys(REGISTRY.quizzes).length,
  };
}
