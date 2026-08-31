import { z } from 'zod';

export const StateAssertionSchema = z.object({
  field: z.string(),
  operator: z.enum([
    'equals',
    'contains',
    'matches_regex',
    'greater_than',
    'less_than',
    'http_get',
    'jsonpath_match',
  ]),
  expected: z.any(),
});

export const TaskValidationSchema = z.object({
  type: z.enum([
    'k8s_resource',
    'network_endpoint',
    'log_match',
    'metric_query',
    'custom_script',
  ]),
  resource: z.string().optional(),
  name: z.string().optional(),
  namespace: z.string().optional(),
  assertions: z.array(StateAssertionSchema),
});

export const LabTaskSchema = z.object({
  id: z.string(),
  title: z.string(),
  description: z.string(),
  points: z.number().int().positive(),
  validation: TaskValidationSchema,
});

export const DeclarativeLabSchema = z.object({
  id: z.string().regex(/^[a-z0-9-]+$/),
  title: z.string().min(3),
  difficulty: z.enum(['beginner', 'intermediate', 'advanced', 'expert']),
  duration_minutes: z.number().int().positive(),
  track: z.enum([
    'foundations',
    'kubernetes',
    'k8s-admin',
    'networking',
    'security',
    'helm',
    'gitops',
    'observability',
    'service-mesh',
    'sre',
    'platform-eng',
    'incidents',
  ]),
  prerequisites: z.array(z.string()).default([]),
  environment: z.object({
    type: z.enum(['kubernetes', 'podman', 'multi-node']).default('kubernetes'),
    cluster: z.string().default('disposable'),
    namespace_isolation: z.boolean().default(true),
    resources: z
      .object({
        cpu_limit: z.string().default('500m'),
        memory_limit: z.string().default('512Mi'),
      })
      .default({ cpu_limit: '500m', memory_limit: '512Mi' }),
  }),
  initial_state: z
    .object({
      manifests: z.array(z.string()).default([]),
    })
    .default({ manifests: [] }),
  scenario: z.string(),
  tasks: z.array(LabTaskSchema).min(1),
  hints: z
    .array(
      z.object({
        text: z.string(),
        penalty_points: z.number().int().nonnegative().default(10),
      }),
    )
    .default([]),
  solution: z.string(),
  cleanup: z
    .object({
      auto: z.boolean().default(true),
    })
    .default({ auto: true }),
  limits: z
    .object({
      max_attempts: z.number().int().positive().default(5),
      timeout_minutes: z.number().int().positive().default(30),
    })
    .default({ max_attempts: 5, timeout_minutes: 30 }),
});

export type ValidatedDeclarativeLab = z.infer<typeof DeclarativeLabSchema>;
