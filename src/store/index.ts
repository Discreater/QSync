import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import type { LocalFolder, RawTrack } from '~/sources/folder';
import { ViewTrack, getTrackInfo, updateFolder } from '~/sources/folder';

import type { JellyfinClientOptions } from '~/sources/jellyfin';
import { mod, shuffle } from '~/utils';

import { generateDeviceId, getDeviceName } from '~/utils/apphost';
import { logger } from '~/utils/logger';

export type Track = RawTrack;

export function sameTrack(a: Track | undefined, b: Track | undefined): boolean {
  if (!a || !b)
    return false;
  return a.path === b.path;
}

interface JellyfinSource {
  deviceId: string
  deviceName: string
  clients: {
    jellyfin: JellyfinClientOptions[]
  }
}

const viewTracks = new Map<string, ViewTrack>();

export async function getShowTrack(path: string): Promise<ViewTrack> {
  const viewed = viewTracks.get(path);
  if (viewed)
    return viewed;
  let full: RawTrack;
  try {
    full = await getTrackInfo(path, true);
  } catch {
    logger.error("can't find track", path);
    return ViewTrack.empty;
  }
  const view = new ViewTrack(full);
  viewTracks.set(path, view);
  return view;
}

export interface PlayerStore {
  playing: boolean
  current: number
  // When not playing, it represents the progress directly. In milliseconds.
  startAt: number
  repeat: boolean
}

export const usePlayerStore = defineStore('playbackStatus', {
  state: () => ({
    playing: false,
    current: 0,
    startAt: 0,
    repeat: false,
  } as PlayerStore),
  getters: {
    progress: (state) => {
      return state.playing ? Date.now() - state.startAt : state.startAt;
    },
  },
  actions: {
    play(current: number) {
      this.$patch({
        current,
        playing: true,
        startAt: Date.now(),
      });
    },
    togglePlay() {
      logger.trace('toggle play');
      this.$patch((state) => {
        state.playing = !state.playing;
        state.startAt = Date.now() - state.startAt;
      });
    },
    pauseTrack() {
      logger.trace('pause track');
      if (this.playing) {
        this.$patch((state) => {
          state.playing = false;
          state.startAt = Date.now() - state.startAt;
        });
      }
    },
    resumeTrack() {
      logger.trace('resume track');
      if (!this.playing) {
        this.$patch((state) => {
          state.playing = true;
          state.startAt = Date.now() - state.startAt;
        });
      }
    },
    /** @param progress should be in milliseconds */
    updateProgress(progress: number) {
      if (!this.playing) {
        this.$patch({
          startAt: progress,
        });
      } else {
        this.$patch({
          startAt: Date.now() - progress,
        });
      }
    },
  },
  persist: true,
});

export const useConfigStore = defineStore('config', {
  state: () => ({
    volume: 100,
  }),
  persist: true,
});

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as LocalFolder[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    playbackQueue: [] as Track[],
    jellyfinSource: {
      deviceId: generateDeviceId(),
      deviceName: getDeviceName(),
    } as JellyfinSource,
  }),
  actions: {
    addMusicFolder(folder: string) {
      if (this.musicFolders.find(v => v.path === folder))
        return;
      this.musicFolders.push({
        path: folder,
        updating: true,
        tracks: [],
      });
      this.updateFolder(folder);
    },
    async updateFolder(folder: string) {
      const f = this.musicFolders.find(v => v.path === folder);
      if (!f)
        return;
      f.updating = true;
      const tracks = await updateFolder(folder);
      f.tracks = tracks;
      f.updating = false;
    },
    removeFolder(folder: string) {
      const f = this.musicFolders.findIndex(v => v.path === folder);
      if (f === -1)
        return;
      this.musicFolders.splice(f, 1);
    },
    play(tracks: Track[], current: number = 0) {
      logger.debug('play tracks');
      this.$patch({
        playbackQueue: tracks,
      });
      const playerStatus = usePlayerStore();
      playerStatus.play(current);
    },
    nextTrack() {
      logger.trace('next track');
      const playerStore = usePlayerStore();
      playerStore.$patch((state) => {
        const nextCurrent = mod(state.current + 1, this.playbackQueue.length);
        state.current = nextCurrent;
        state.startAt = Date.now();
      });
    },
    previousTrack() {
      logger.trace('previous track');
      const playerStore = usePlayerStore();
      playerStore.$patch((state) => {
        const nextCurrent = mod(state.current - 1, this.playbackQueue.length);
        state.current = nextCurrent;
        state.startAt = Date.now();
      });
    },
    shufflePLayback() {
      this.playbackQueue = shuffle([...this.playbackQueue]);
      const playerStore = usePlayerStore();
      playerStore.$patch({
        current: 0,
        startAt: Date.now(),
      });
    },
  },
  persist: true,
});

export const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
