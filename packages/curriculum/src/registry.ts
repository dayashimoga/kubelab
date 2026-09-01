import curriculumData from './curriculum.json';
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
