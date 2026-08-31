import { DifficultyLevel, TrackSlug } from './curriculum.js';

export type LabStatus =
  | 'provisioning'
  | 'bootstrapping'
  | 'ready'
  | 'running'
  | 'validating'
  | 'completed'
  | 'failed'
  | 'expired'
  | 'destroyed';

export type ValidationOperator =
  | 'equals'
  | 'contains'
  | 'matches_regex'
  | 'greater_than'
  | 'less_than'
  | 'http_get'
  | 'jsonpath_match';

export interface StateAssertion {
  field: string;
  operator: ValidationOperator;
  expected: any;
  actual?: any;
  passed?: boolean;
}

export interface TaskValidation {
  type: 'k8s_resource' | 'network_endpoint' | 'log_match' | 'metric_query' | 'custom_script';
  resource?: string;
  name?: string;
  namespace?: string;
  assertions: StateAssertion[];
}

export interface LabTask {
  id: string;
  title: string;
  description: string;
  points: number;
  validation: TaskValidation;
  status?: 'pending' | 'in_progress' | 'passed' | 'failed';
}

export interface LabHint {
  text: string;
  penaltyPoints: number;
}

export interface DeclarativeLab {
  id: string;
  title: string;
  difficulty: DifficultyLevel;
  durationMinutes: number;
  track: TrackSlug;
  prerequisites: string[];
  environment: {
    type: 'kubernetes' | 'podman' | 'multi-node';
    cluster: string;
    namespaceIsolation: boolean;
    resources: {
      cpuLimit: string;
      memoryLimit: string;
    };
  };
  initialState: {
    manifests: string[];
  };
  scenario: string;
  tasks: LabTask[];
  hints: LabHint[];
  solution: string;
  cleanup: {
    auto: boolean;
  };
  limits: {
    maxAttempts: number;
    timeoutMinutes: number;
  };
}

export interface LabSession {
  id: string;
  labId: string;
  userId: string;
  status: LabStatus;
  namespace: string;
  clusterEndpoint: string;
  startedAt: string;
  expiresAt: string;
  completedAt?: string;
  score: number;
  maxScore: number;
  taskResults: Record<string, boolean>;
}
