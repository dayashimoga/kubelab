export type TrackSlug =
  | 'linux-containers'
  | 'kubernetes'
  | 'storage'
  | 'networking'
  | 'helm-kustomize'
  | 'administration'
  | 'security'
  | 'gitops'
  | 'service-mesh'
  | 'observability'
  | 'troubleshooting'
  | 'sre-performance'
  | 'platform-eng'
  | 'incidents'
  | 'certification'
  | 'foundations'
  | 'k8s-admin'
  | 'helm'
  | 'sre';

export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced' | 'expert';

export interface Concept {
  id: string;
  name: string;
  slug: string;
  category: string;
  description: string;
}

export interface QuizQuestionItem {
  id: string;
  prompt: string;
  options: string[];
  correctIndex: number;
  explanation: string;
  codeSnippet?: string;
}

export interface LessonQuiz {
  id: string;
  lessonId: string;
  trackSlug: TrackSlug;
  title: string;
  questions: QuizQuestionItem[];
}

export interface LessonPracticeWidget {
  type: 'yaml_editor' | 'cli_sandbox' | 'architecture_diagram' | 'troubleshoot_picker';
  initialCode?: string;
  solutionCode?: string;
  hints?: string[];
  explanation?: string;
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
  practiceWidget?: LessonPracticeWidget;
  commonMistakes?: string[];
  productionGuidance?: string;
}

export interface Module {
  id: string;
  trackId: string;
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
  color: string;
  difficulty: DifficultyLevel;
  order: number;
  totalLessons: number;
  totalXp: number;
  modules: Module[];
}

export interface CurriculumRegistry {
  tracks: Track[];
  quizzes: Record<string, LessonQuiz>;
  stats: {
    totalTracks: number;
    totalModules: number;
    totalLessons: number;
    totalQuizzes: number;
    totalXp: number;
  };
}
