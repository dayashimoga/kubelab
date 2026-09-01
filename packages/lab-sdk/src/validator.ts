import { DeclarativeLabSchema, ValidatedDeclarativeLab } from './schema';
import YAML from 'yaml';

export function parseAndValidateLabYaml(yamlString: string): {
  success: boolean;
  data?: ValidatedDeclarativeLab;
  error?: string;
} {
  try {
    const raw = YAML.parse(yamlString);
    const parsed = DeclarativeLabSchema.safeParse(raw);
    if (!parsed.success) {
      return {
        success: false,
        error: parsed.error.issues.map((i) => `${i.path.join('.')}: ${i.message}`).join(', '),
      };
    }
    return {
      success: true,
      data: parsed.data,
    };
  } catch (err: any) {
    return {
      success: false,
      error: `YAML syntax error: ${err.message}`,
    };
  }
}
