import { invoke } from '@tauri-apps/api';
import type { Track } from '~/generated/protos/musync';

export interface LocalFolder {
  path: string
  updating: boolean
  tracks: Track[]
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
