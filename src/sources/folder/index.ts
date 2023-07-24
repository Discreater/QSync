import { invoke } from '@tauri-apps/api';

export class Track {
  path: string;
  constructor(path: string) {
    this.path = path;
  }

  name(): string {
    return this.path.split('\\').pop()!;
  }
}

export interface LocalFolder {
  path: string
  updating: boolean
  tracks: Track[]
}

interface RawTrack {
  path: string
}

export async function updateFolder(path: string): Promise<Track[]> {
  return (await invoke('update_folder', { dir: path }) as RawTrack[]).map(v => new Track(v.path));
}
