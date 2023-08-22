import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { usePlayerStore } from './player';
import { ApiClient } from '~/api/client';
import type { Track } from '~/generated/protos/musync';
import type { TrackId } from '~/model_ext/track';
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
      await this.updateFoldersFromRemote();
    },
    async updateFoldersFromRemote() {
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
    async updatePlayQueueFromRemote(trackIds?: TrackId[]) {
      if (!trackIds) {
        const play_queue = await ApiClient.grpc().GetPlayQueue({});
        trackIds = play_queue.trackIds;
      }
      const tracks = trackIds
        .map(id =>
          this.musicFolders
            .flatMap(v => v.tracks)
            .find(v => v.id === id));
      this.$patch({
        playQueue: tracks,
      });
    },
    async play(tracks: Track[], current: number = 0) {
      logger.debug('play tracks');
      await ApiClient.grpc().CreatePlayQueue({
        trackIds: tracks.map(v => v.id),
      });
      this.$patch({
        playQueue: tracks,
      });
      const playerStatus = usePlayerStore();
      playerStatus.play(current);
    },
    nextTrack(manul: boolean) {
      logger.trace('next track');
      const playerStore = usePlayerStore();
      const nextPosition = mod(playerStore.position + 1, this.playQueue.length);
      playerStore.updatePosition(nextPosition, manul);
    },
    previousTrack() {
      logger.trace('previous track');
      const playerStore = usePlayerStore();
      const nextPosition = mod(playerStore.position - 1, this.playQueue.length);
      playerStore.updatePosition(nextPosition, true);
    },
    async shufflePlayQueue() {
      const playQueue = shuffle([...this.playQueue]);
      await ApiClient.grpc().CreatePlayQueue({
        trackIds: playQueue.map(v => v.id),
      });
      this.$patch({
        playQueue,
      });
      const playerStore = usePlayerStore();
      playerStore.updatePosition(0, true);
    },
  },
  persist: true,
});

export const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
