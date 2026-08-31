import { DifficultyLevel } from './curriculum.js';

export type IncidentSeverity = 'SEV-1' | 'SEV-2' | 'SEV-3' | 'SEV-4';

export interface IncidentAlert {
  id: string;
  timestamp: string;
  source: 'prometheus' | 'opentelemetry' | 'argocd' | 'kubernetes';
  summary: string;
  severity: IncidentSeverity;
  labels: Record<string, string>;
}

export interface IncidentScenario {
  id: string;
  title: string;
  severity: IncidentSeverity;
  difficulty: DifficultyLevel;
  timeLimitMinutes: number;
  briefing: string;
  topology: {
    services: Array<{
      name: string;
      tier: 'frontend' | 'backend' | 'database' | 'cache' | 'ingress';
      status: 'healthy' | 'degraded' | 'failing';
    }>;
  };
  injectedChaos: Array<{
    type: string;
    target: string;
    description: string;
  }>;
  initialAlerts: IncidentAlert[];
  resolutionCriteria: {
    recoveredService: string;
    maxAllowedErrorsPerSec: number;
    requiredUptimeSeconds: number;
  };
}
