export interface SkillNode {
  id: string;
  name: string;
  slug: string;
  category: string;
  description: string;
  level: number; // 0 to 5 mastery
  xp: number;
  icon: string;
  prerequisites: string[];
}

export interface SkillGraph {
  nodes: SkillNode[];
  edges: Array<{
    source: string;
    target: string;
  }>;
}

export interface Badge {
  id: string;
  slug: string;
  name: string;
  description: string;
  icon: string;
  unlockedAt?: string;
  xpReward: number;
}

export interface UserProgress {
  userId: string;
  totalXp: number;
  level: number;
  currentStreakDays: number;
  longestStreakDays: number;
  lastActiveDate: string;
  completedLessonIds: string[];
  completedLabIds: string[];
  unlockedBadges: Badge[];
  skills: Record<string, number>; // skillId -> level (0-5)
}
