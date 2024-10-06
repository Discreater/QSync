import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { ApiClient, WsClient } from '~/api/client';
import type { Track } from '~/generated/protos/musync';
import { useLoading } from '~/logic';
import type { TrackId } from '~/model_ext/track';
import type { LocalFolder } from '~/sources/folder';
import type { JellyfinClientOptions } from '~/sources/jellyfin';

import { mod, shuffle } from '~/utils';
import { logger } from '~/utils/logger';
import { usePlayerStore } from './player';
import { useAccountStore } from './user';

export function sameTrack(a: Track | undefined, b: Track | undefined): boolean {
  if (!a || !b)
    return false;
  return a.id === b.id;
}

export interface JellyfinSource {
  deviceId: string
  deviceName: string
  clients: {
    jellyfin: JellyfinClientOptions[]
  }
}

export const useMusyncStore = defineStore('musync', {
  state: () => ({
    musicFolders: [] as LocalFolder[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    playQueue: [] as Track[],
    // jellyfinSource: {
    //   deviceId: generateDeviceId(),
    //   deviceName: getDeviceName(),
    // } as JellyfinSource,
  }),
  actions: {
    async init(host: string) {
      const loading = useLoading();
      loading.value = true;

      WsClient.listenOnUpdatePlayQueue((update) => {
        if (!loading.value)
          this.updatePlayQueueFromRemote(update.trackIds);
      });
      const playerStore = usePlayerStore();
      WsClient.listenOnUpdatePlayer((update) => {
        playerStore.updateFromRemote(update);
      });

      await ApiClient.set(`${host}:8396`);

      const account = useAccountStore();
      if (!account.online) {
        loading.value = false;
        return;
      }

      await this.updateFoldersFromRemote();
      await this.updatePlayQueueFromRemote();
      loading.value = false;
    },
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
      logger.trace('update folders from remote');
      const folderUpdates: Promise<void>[] = [];
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
        folderUpdates.push(this.updateFolder(f));
      });
      logger.trace(`Got ${folderUpdates.length} folders`);
      await Promise.all(folderUpdates);
      logger.trace('update folders from remote done');
    },
    async updateFolder(folder: string | LocalFolder) {
      logger.trace('update folder');
      let f: LocalFolder | undefined;
      if (typeof folder === 'string')
        f = this.musicFolders.find(({ path }) => path === folder);
      else
        f = folder;

      if (f === undefined) {
        logger.warn(`folder ${folder} not found`);
        return;
      }
      f.updating = true;
      f.tracks = [];
      await ApiClient.grpc().QueryTracks({}).forEach((track) => {
        if (track)
          f!.tracks.push(track);
      }).then(() => {
        f!.updating = false;
      });
    },
    async removeFolder(folder: string) {
      logger.trace('remove folder');
      const f = this.musicFolders.findIndex(v => v.path === folder);
      if (f === -1)
        return;
      await ApiClient.grpc().RemoveLocalFolder({
        path: folder,
      });
      this.musicFolders.splice(f, 1);
    },
    async updatePlayQueueFromRemote(trackIds?: TrackId[]) {
      logger.trace('update play queue from remote');
      if (!trackIds) {
        try {
          const play_queue = await ApiClient.grpc().GetPlayQueue({});
          trackIds = play_queue.trackIds;
        } catch (e) {
          console.error(e);
          return;
        }
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
