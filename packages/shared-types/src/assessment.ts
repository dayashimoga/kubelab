export type QuestionType =
  | 'multiple_choice'
  | 'multi_select'
  | 'true_false'
  | 'ordering'
  | 'matching'
  | 'fill_in_blank'
  | 'yaml_bug_hunt';

export interface QuestionOption {
  id: string;
  text: string;
  explanation?: string;
}

export interface Question {
  id: string;
  lessonId?: string;
  conceptId?: string;
  type: QuestionType;
  difficulty: 'easy' | 'medium' | 'hard';
  points: number;
  prompt: string;
  codeSnippet?: string;
  options: QuestionOption[];
  correctAnswer: string | string[] | Record<string, string>;
  explanation: string;
}

export interface QuizSubmission {
  quizId: string;
  answers: Record<string, string | string[]>;
  timeSpentSeconds: number;
}

export interface QuizResult {
  score: number;
  maxScore: number;
  percentage: number;
  passed: boolean;
  xpEarned: number;
  breakdown: Array<{
    questionId: string;
    isCorrect: boolean;
    earnedPoints: number;
    explanation: string;
  }>;
}
