import { invoke } from '@tauri-apps/api';
import { logger } from '~/utils/logger';

export class ViewTrack {
  constructor(public raw: RawTrack) {
  }

  key(): string {
    return this.raw.path;
  }

  name(): string {
    return this.raw.title ?? this.raw.path.split(/[\\/]/).pop() ?? this.raw.path;
  }

  artist(): string {
    return this.raw.artist ?? '';
  }

  album(): string {
    return this.raw.album ?? '';
  }

  genre(): string {
    return this.raw.genre ?? '';
  }

  duration(): number {
    return this.raw.duration;
  }

  year(): number | undefined {
    return this.raw.year;
  }
}

export interface LocalFolder {
  path: string
  updating: boolean
  tracks: RawTrack[]
}

export interface RawTrack {
  path: string
  duration: number
  artist?: string
  album?: string
  title?: string
  genre?: string
  year?: number
}

export async function updateFolder(path: string): Promise<RawTrack[]> {
  return (await invoke('update_folder', { dir: path }) as RawTrack[]);
}

export async function readTrack(path: string): Promise<Blob> {
  logger.trace('read track from', path);
  const data = Uint8Array.from(await invoke('read_track', { path }));
  return new Blob([data.buffer], { type: 'audio/mpeg' });
}
