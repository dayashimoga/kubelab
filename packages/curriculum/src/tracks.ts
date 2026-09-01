import { Track } from '@kubelab/shared-types';
import registryData from './curriculum.json';

export const TRACKS: Track[] = (registryData as any).tracks;
