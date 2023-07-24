import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { type LocalFolder, updateFolder } from '~/sources/folder';
import type { JellyfinClientOptions } from '~/sources/jellyfin';
import { generateDeviceId, getDeviceName } from '~/utils/apphost';

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as LocalFolder[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    deviceId: generateDeviceId(),
    deviceName: getDeviceName(),
    clients: {} as {
      jellyfin: JellyfinClientOptions[]
    },
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
  },
  persist: true,
});

export const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
