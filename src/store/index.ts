import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { ApiClient } from '~/api/client';
import type { Track } from '~/generated/protos/musync';
import type { LocalFolder } from '~/sources/folder';

import type { JellyfinClientOptions } from '~/sources/jellyfin';
import { mod, shuffle } from '~/utils';

import { generateDeviceId, getDeviceName } from '~/utils/apphost';
import { logger } from '~/utils/logger';

export function sameTrack(a: Track | undefined, b: Track | undefined): boolean {
  if (!a || !b)
    return false;
  return a.id === b.id;
}

interface JellyfinSource {
  deviceId: string
  deviceName: string
  clients: {
    jellyfin: JellyfinClientOptions[]
  }
}

export interface PlayerStore {
  playing: boolean
  current: number
  // When not playing, it represents the progress directly. In milliseconds.
  startAt: number
  repeat: boolean
}

export const usePlayerStore = defineStore('playQueueStatus', {
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
    muted: false,
  }),
  persist: true,
});

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as LocalFolder[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    playQueue: [] as Track[],
    jellyfinSource: {
      deviceId: generateDeviceId(),
      deviceName: getDeviceName(),
    } as JellyfinSource,
  }),
  actions: {
    async addMusicFolder(folder: string) {
      if (this.musicFolders.find(v => v.path === folder))
        return;
      try {
        await ApiClient.get().grpcClient.AddLocalFolder({
          path: folder,
        });
      } catch (e) {
        console.error(e);
      }
      await this.updateFolders();
    },
    async updateFolders() {
      await ApiClient.get().grpcClient.QueryLocalFolders({}).forEach((folder) => {
        let f = this.musicFolders.find(v => v.path === folder.path);
        if (!f) {
          this.musicFolders.push({
            path: folder.path,
            updating: true,
            tracks: [],
          });
          f = this.musicFolders[this.musicFolders.length - 1];
        }
        this.updateFolder(f);
      });
    },
    async updateFolder(folder: string | LocalFolder) {
      let f: LocalFolder | undefined;
      if (typeof folder === 'string')
        f = this.musicFolders.find(v => v.path === folder);
      else
        f = folder;

      if (f === undefined)
        return;
      f.updating = true;
      f.tracks = [];
      ApiClient.grpc().QueryTracks({}).forEach((track) => {
        f!.tracks.push(track);
      }).then(() => {
        f!.updating = false;
      });
    },
    async removeFolder(folder: string) {
      const f = this.musicFolders.findIndex(v => v.path === folder);
      if (f === -1)
        return;
      await ApiClient.grpc().RemoveLocalFolder({
        path: folder,
      });
      this.musicFolders.splice(f, 1);
    },
    async play(tracks: Track[], current: number = 0) {
      logger.debug('play tracks');
      this.$patch({
        playQueue: tracks,
      });
      // let { playlist } = await ApiClient.grpc().CreatePlaylist({
      //   trackIds: tracks.map(v => v.id),
      //   name: 'temp',
      //   description: 'temp',
      //   temp: true,
      // });
      const playerStatus = usePlayerStore();
      playerStatus.play(current);
    },
    nextTrack() {
      logger.trace('next track');
      const playerStore = usePlayerStore();
      playerStore.$patch((state) => {
        const nextCurrent = mod(state.current + 1, this.playQueue.length);
        state.current = nextCurrent;
        state.startAt = Date.now();
      });
    },
    previousTrack() {
      logger.trace('previous track');
      const playerStore = usePlayerStore();
      playerStore.$patch((state) => {
        const nextCurrent = mod(state.current - 1, this.playQueue.length);
        state.current = nextCurrent;
        state.startAt = Date.now();
      });
    },
    shufflePLayQueue() {
      this.playQueue = shuffle([...this.playQueue]);
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
