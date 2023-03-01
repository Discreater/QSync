import { createPinia, defineStore } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import type { JellyfinClientOptions } from '~/sources/jellyfin';
import { generateDeviceId, getDeviceName } from '~/utils/apphost';

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as string[],
    locale: 'zh-CN' as 'zh-CN' | 'en',
    deviceId: generateDeviceId(),
    deviceName: getDeviceName(),
    clients: {} as {
      jellyfin: JellyfinClientOptions[]
    },
  }),
  actions: {
    addMusicFolder(folder: string) {
      if (this.musicFolders.includes(folder))
        return;
      this.musicFolders.push(folder);
    },
  },
  persist: true,
});

export const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
