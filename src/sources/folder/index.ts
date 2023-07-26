import { invoke } from '@tauri-apps/api';
import { logger } from '~/utils/logger';

export class ViewTrack {
  path: string;
  constructor(raw: RawTrack) {
    this.path = raw.path;
  }

  name(): string {
    return this.path.split('\\').pop()!;
  }
}

export interface LocalFolder {
  path: string
  updating: boolean
  tracks: RawTrack[]
}

export interface RawTrack {
  path: string
}

export async function updateFolder(path: string): Promise<RawTrack[]> {
  return (await invoke('update_folder', { dir: path }) as RawTrack[]);
}

export async function readTrack(path: string): Promise<Blob> {
  logger.trace('read track from', path);
  const data = Uint8Array.from(await invoke('read_track', { path }));
  return new Blob([data.buffer], { type: 'audio/mpeg' });
}
