import { DeclarativeLabSchema, ValidatedDeclarativeLab } from './schema';
import YAML from 'yaml';

export class LabBuilder {
  private lab: Partial<ValidatedDeclarativeLab> = {
    prerequisites: [],
    hints: [],
    tasks: [],
    environment: {
      type: 'kubernetes',
      cluster: 'disposable',
      namespace_isolation: true,
      resources: { cpu_limit: '500m', memory_limit: '512Mi' },
    },
    initial_state: { manifests: [] },
    cleanup: { auto: true },
    limits: { max_attempts: 5, timeout_minutes: 30 },
  };

  setId(id: string): this {
    this.lab.id = id;
    return this;
  }

  setTitle(title: string): this {
    this.lab.title = title;
    return this;
  }

  setDifficulty(difficulty: 'beginner' | 'intermediate' | 'advanced' | 'expert'): this {
    this.lab.difficulty = difficulty;
    return this;
  }

  setTrack(track: any): this {
    this.lab.track = track;
    return this;
  }

  setDuration(minutes: number): this {
    this.lab.duration_minutes = minutes;
    return this;
  }

  setScenario(scenario: string): this {
    this.lab.scenario = scenario;
    return this;
  }

  addTask(task: any): this {
    this.lab.tasks = this.lab.tasks || [];
    this.lab.tasks.push(task);
    return this;
  }

  addHint(text: string, penaltyPoints = 10): this {
    this.lab.hints = this.lab.hints || [];
    this.lab.hints.push({ text, penalty_points: penaltyPoints });
    return this;
  }

  setSolution(solution: string): this {
    this.lab.solution = solution;
    return this;
  }

  build(): ValidatedDeclarativeLab {
    return DeclarativeLabSchema.parse(this.lab);
  }

  toYaml(): string {
    const validated = this.build();
    return YAML.stringify(validated);
  }
}
