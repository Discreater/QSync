import { defineStore } from 'pinia';

export const useQSyncStore = defineStore('qsync', {
  state: () => ({
    musicFolders: [] as string[],
  }),
  actions: {
    addMusicFolder(folder: string) {
      if (this.musicFolders.includes(folder))
        return;
      this.musicFolders.push(folder);
    },
  },
});
