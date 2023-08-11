import { invoke } from '@tauri-apps/api';

export class ViewTrack {
  static empty = new ViewTrack({
    path: '@undefined@',
    duration: 0,
  });

  url: string | undefined;
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

  picture_url(): string | undefined {
    if (this.url === undefined) {
      if (!this.raw.pictures || this.raw.pictures?.length === 0)
        return undefined;
      const p1 = this.raw.pictures[0];
      const blob = new Blob([Uint8Array.from(p1.data as unknown as number[]).buffer], { type: p1.mime_type });
      this.url = URL.createObjectURL(blob);
      return this.url;
    }
    return this.url;
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
  pictures?: Picture[]
  fullLoaded?: boolean
}

export interface Picture {
  mime_type: string
  picture_type: number
  description: string
  data: number[]
}

export async function updateFolder(path: string): Promise<RawTrack[]> {
  return (await invoke('update_folder', { dir: path }) as RawTrack[]);
}

export async function getTrackInfo(path: string, full: boolean): Promise<RawTrack> {
  const raw = (await invoke('get_track_info', { path, full }) as RawTrack);
  raw.fullLoaded = true;
  return raw;
}
