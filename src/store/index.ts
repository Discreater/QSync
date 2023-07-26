import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { updateFolder } from '~/sources/folder';
import type { LocalFolder, RawTrack } from '~/sources/folder';

import type { JellyfinClientOptions } from '~/sources/jellyfin';

import { generateDeviceId, getDeviceName } from '~/utils/apphost';
import { logger } from '~/utils/logger';

export type Track = RawTrack;

export function sameTrack(a: Track | undefined, b: Track | undefined): boolean {
  if (!a || !b)
    return false;
  return a.path === b.path;
}

interface PlaybackQueue {
  queue: Track[]
  playing: boolean
  current: number
  /** in milliseconds */
  progress: number
  repeat: boolean
}
interface JellyfinSource {
  deviceId: string
  deviceName: string
  clients: {
    jellyfin: JellyfinClientOptions[]
  }
}

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as LocalFolder[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    playbackQueue: {
      queue: [] as Track[],
      playing: false,
      current: 0,
      progress: 0,
      repeat: false,
    } as PlaybackQueue,
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
    play(tracks: Track[]) {
      logger.debug('play tracks');
      this.$patch({
        playbackQueue: {
          queue: tracks,
          playing: true,
          current: 0,
          progress: 0,
          repeat: true,
        },
      });
    },
    nextTrack() {
      logger.trace('next track');
      this.$patch((state) => {
        state.playbackQueue.current = state.playbackQueue.current === state.playbackQueue.queue.length - 1 ? 0 : state.playbackQueue.current + 1;
        state.playbackQueue.progress = 0;
      });
    },
    previousTrack() {
      logger.trace('previous track');
      this.$patch((state) => {
        state.playbackQueue.current = state.playbackQueue.current === 0 ? state.playbackQueue.queue.length - 1 : state.playbackQueue.current - 1;
        state.playbackQueue.progress = 0;
      });
    },
    togglePlay() {
      logger.trace('toggle play');
      this.$patch((state) => {
        state.playbackQueue.playing = !state.playbackQueue.playing;
      });
    },
    pauseTrack() {
      logger.trace('pause track');
      this.$patch({
        playbackQueue: {
          playing: false,
        },
      });
    },
    resumeTrack() {
      logger.trace('resume track');
      this.$patch({
        playbackQueue: {
          playing: true,
        },
      });
    },
  },
  persist: true,
});

export const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
