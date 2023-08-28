import { defineStore } from 'pinia';
import { ApiClient } from '~/api/client';
import type { Token } from '~/generated/protos/musync';

export interface AccountStore {
  username: string
  token?: Token
}

export const useAccountStore = defineStore('account', {
  state: () => ({
    username: 'Anonymous',
    token: undefined,
  } as AccountStore),
  getters: {
    online: (state) => {
      return state.token !== undefined;
    },
  },
  actions: {
    login(name: string) {
      ApiClient.grpc().Login({ name, type: 'local' }).then((token) => {
        this.$patch({
          token,
          username: name,
        });
        ApiClient.reset();
      }).catch((err) => {
        console.error(err);
      });
    },
    logout() {
      this.$patch({
        token: undefined,
      });
    },
  },
  persist: true,
});
