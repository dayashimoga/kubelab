export type TrackSlug =
  | 'foundations'
  | 'kubernetes'
  | 'k8s-admin'
  | 'networking'
  | 'security'
  | 'helm'
  | 'gitops'
  | 'observability'
  | 'service-mesh'
  | 'sre'
  | 'platform-eng'
  | 'incidents';

export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced' | 'expert';

export interface Concept {
  id: string;
  name: string;
  slug: string;
  category: string;
  description: string;
}

export interface Lesson {
  id: string;
  moduleId: string;
  trackSlug: TrackSlug;
  title: string;
  slug: string;
  order: number;
  durationMinutes: number;
  xp: number;
  summary: string;
  contentMarkdown: string;
  diagramConfig?: Record<string, any>;
  concepts: string[];
  prerequisites: string[];
  associatedLabId?: string;
  associatedQuizId?: string;
}

export interface Module {
  id: string;
  courseId: string;
  title: string;
  slug: string;
  order: number;
  description: string;
  lessons: Lesson[];
}

export interface Track {
  id: string;
  slug: TrackSlug;
  title: string;
  description: string;
  icon: string;
  difficulty: DifficultyLevel;
  order: number;
  totalLessons: number;
  totalXp: number;
  modules: Module[];
}
