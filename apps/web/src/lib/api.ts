const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export interface User {
  id: string;
  email: string;
  name: string;
  role: 'learner' | 'instructor' | 'admin';
  avatar_url?: string;
}

export interface AuthResponse {
  user: User;
  tokens: {
    access_token: string;
    token_type: string;
    expires_in: number;
  };
}

export interface TrackSummary {
  id: string;
  slug: string;
  title: string;
  description: string;
  icon: string;
  difficulty: string;
  order: number;
  total_lessons: number;
  total_xp: number;
}

export interface LessonDetail {
  id: string;
  track_slug: string;
  title: string;
  slug: string;
  order: number;
  duration_minutes: number;
  xp: number;
  summary: string;
  content_markdown: string;
  concepts: string[];
  prerequisites: string[];
  associated_lab_id?: string;
  associated_quiz_id?: string;
}

export interface UserProgressState {
  user_id: string;
  total_xp: number;
  level: number;
  current_streak_days: number;
  longest_streak_days: number;
  last_active_date: string;
  completed_lesson_ids: string[];
  completed_lab_ids: string[];
  unlocked_badges: Array<{
    id: string;
    slug: string;
    name: string;
    description: string;
    icon: string;
    unlocked_at?: string;
    xp_reward: number;
  }>;
  skills: Record<string, number>;
}

export interface LabSummary {
  id: string;
  title: string;
  difficulty: string;
  duration_minutes: number;
  track: string;
  task_count: number;
  total_points: number;
}

export interface IncidentScenario {
  id: string;
  title: string;
  severity: string;
  services: Array<{ name: string; tier: string; status: string; errors: string }>;
  max_bounty_xp: number;
  time_limit_seconds: number;
}

export interface QuizQuestion {
  id: string;
  topic: string;
  difficulty: string;
  points: number;
  prompt: string;
  options: Array<{ id: string; text: string }>;
}

export interface Certification {
  id: string;
  title: string;
  difficulty: string;
  duration: string;
  questions: string;
  status: string;
  desc: string;
}

export interface DocSection {
  id: string;
  title: string;
  content: string;
}

export interface SkillNode {
  id: string;
  name: string;
  category: string;
  level: number;
  xp: number;
  description: string;
  prerequisites: string[];
  recommended_lab: string;
}

class ApiClient {
  private getToken(): string | null {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('kubelab_jwt_token');
    }
    return null;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const token = this.getToken();
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string>),
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const errData = await response.json().catch(() => ({ error: response.statusText }));
      throw new Error(errData.error || `HTTP error ${response.status}`);
    }

    return response.json();
  }

  // Auth APIs
  async register(email: string, name: string, password: string, role?: string): Promise<AuthResponse> {
    return this.request<AuthResponse>('/v1/auth/register', {
      method: 'POST',
      body: JSON.stringify({ email, name, password, role }),
    });
  }

  async login(email: string, password: string): Promise<AuthResponse> {
    return this.request<AuthResponse>('/v1/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  async getMe(): Promise<User> {
    return this.request<User>('/v1/auth/me');
  }

  // Curriculum APIs
  async getTracks(): Promise<TrackSummary[]> {
    return this.request<TrackSummary[]>('/v1/tracks');
  }

  async getTrackBySlug(slug: string): Promise<TrackSummary> {
    return this.request<TrackSummary>(`/v1/tracks/${slug}`);
  }

  async getLessonsByTrack(trackSlug: string): Promise<LessonDetail[]> {
    return this.request<LessonDetail[]>(`/v1/tracks/${trackSlug}/lessons`);
  }

  async getLesson(id: string): Promise<LessonDetail> {
    return this.request<LessonDetail>(`/v1/lessons/${id}`);
  }

  // Progress APIs
  async getProgress(userId = 'test-learner-1'): Promise<UserProgressState> {
    return this.request<UserProgressState>(`/v1/progress/${userId}`);
  }

  async getSkillGraph(): Promise<{ nodes: any[]; edges: any[] }> {
    return this.request<{ nodes: any[]; edges: any[] }>('/v1/skills/graph');
  }

  // Lab APIs
  async getLabs(): Promise<LabSummary[]> {
    return this.request<LabSummary[]>('/v1/labs');
  }

  async getLab(id: string): Promise<any> {
    return this.request<any>(`/v1/labs/${id}`);
  }

  async startLab(labId: string): Promise<any> {
    return this.request<any>('/v1/labs/start', {
      method: 'POST',
      body: JSON.stringify({ lab_id: labId }),
    });
  }

  async applyManifest(sessionId: string, yamlContent: string): Promise<any> {
    return this.request<any>(`/v1/labs/sessions/${sessionId}/apply`, {
      method: 'POST',
      body: JSON.stringify({ yaml_content: yamlContent }),
    });
  }

  async getLabResources(sessionId: string): Promise<any[]> {
    return this.request<any[]>(`/v1/labs/sessions/${sessionId}/resources`);
  }

  async validateLab(sessionId: string, taskId: string, liveState?: any): Promise<any> {
    return this.request<any>(`/v1/labs/sessions/${sessionId}/validate`, {
      method: 'POST',
      body: JSON.stringify({ task_id: taskId, live_state: liveState }),
    });
  }

  // AI Tutor API
  async queryTutor(mode: string, userPrompt: string, errorLog?: string): Promise<any> {
    return this.request<any>('/v1/ai-tutor/query', {
      method: 'POST',
      body: JSON.stringify({
        mode,
        user_prompt: userPrompt,
        current_error_log: errorLog,
      }),
    });
  }

  // Incidents API
  async getIncidents(): Promise<IncidentScenario[]> {
    return this.request<IncidentScenario[]>('/v1/incidents');
  }

  // Quiz/Practice API
  async getQuizQuestions(topic?: string): Promise<QuizQuestion[]> {
    const qs = topic ? `?topic=${encodeURIComponent(topic)}` : '';
    return this.request<QuizQuestion[]>(`/v1/assessments/questions${qs}`);
  }

  async submitQuiz(answers: Record<string, string>): Promise<{ score: number; total: number; xp_earned: number }> {
    return this.request('/v1/assessments/submit', {
      method: 'POST',
      body: JSON.stringify({ answers }),
    });
  }

  // Certifications API
  async getCertifications(): Promise<Certification[]> {
    return this.request<Certification[]>('/v1/certifications');
  }

  // Docs API
  async getDocSections(): Promise<DocSection[]> {
    return this.request<DocSection[]>('/v1/docs/sections');
  }

  // Skills API (already have getSkillGraph, add detailed nodes)
  async getSkillNodes(): Promise<SkillNode[]> {
    return this.request<SkillNode[]>('/v1/skills/nodes');
  }
}

export const api = new ApiClient();
