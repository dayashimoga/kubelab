import { Track } from '@kubelab/shared-types';
import curriculumData from './curriculum.json';

export const TRACKS: Track[] = curriculumData.tracks as unknown as Track[];
