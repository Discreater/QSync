<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import Basic from '~/layouts/Basic.vue';
import H2 from '~/components/typo/H2.vue';
import QButton from '~/components/QButton.vue';

import IconAdd from '~icons/fluent/add-circle-24-regular';
import IconSpinner from '~icons/fluent/spinner-ios-20-regular';
import IconFolder from '~icons/fluent/folder-24-regular';
import IconDelete from '~icons/fluent/delete-24-regular';
import { useMusyncStore } from '~/store';
import QInputRow from '~/components/QInputRow.vue';
import QSelect from '~/components/QSelect.vue';
import LongButton from '~/components/LongButton.vue';
import type { Item } from '~/components/types';
import { logger } from '~/utils/logger';
import { getPlatform } from '~/platforms';

const { t } = useI18n();
const store = useMusyncStore();

type SourceType = 'jellyfin' | 'local';

const sourceType = ref<SourceType>('local');
const sourceTypeOptions: Item[] = [
  { name: t('source.types.jellyfin'), key: 'jellyfin' },
  { name: t('source.types.local'), key: 'local' },
];
const server = ref<string>();
const username = ref<string>();
const password = ref<string>();
const directory = ref<string>();

const showAddModel = ref(false);
async function addAccount() {
  if (sourceType.value === 'jellyfin') {
    if (server.value !== undefined && username.value !== undefined && password.value !== undefined) {
      const _opt = {
        server: server.value,
        user: username.value,
        pwd: password.value,
      };
      // const _client = new JellyfinClient(opt, store.jellyfinSource.deviceName, store.jellyfinSource.deviceId);
      logger.warn('jellyfin not support yet');
    }
    // const client = new JellyfinClient({
    // }, store.deviceName, store.deviceId);
    // client.connect().then(() => {
    //   logger.info('jellyfin test success');
    // }).catch((reason) => {
    //   logger.error('jellyfin test error');
    //   logger.error(reason);
    // });
  } else if (sourceType.value === 'local') {
    let path;
    if (getPlatform() === 'web') {
      // path = 'D:\\media\\music';
      path = '/mnt/wd/音乐/';
    } else if (directory.value) {
      path = directory.value;
    } else {
      return;
    }

    store.addMusicFolder(path);
    showAddModel.value = false;
  }
}
</script>

<template>
  <Basic :header="t('menu.source')" :show-model="showAddModel">
    <template #header-extra>
      <QButton :icon="IconAdd" :text="t('source.add')" @click="showAddModel = true" />
    </template>
    <H2>{{ t('source.types.local') }}</H2>
    <div>
      <LongButton
        v-for="dir in store.musicFolders" :key="dir.path" :text="dir.path" :icon="IconFolder"
        @click="store.updateFolder(dir.path)"
      >
        <template #extra>
          <IconSpinner :class="dir.updating ? 'animate-spin' : ''" />
          <QButton :icon="IconDelete" @click="store.removeFolder(dir.path)" />
        </template>
      </LongButton>
    </div>
    <template #model>
      <div class="flex flex-col bg-[#1c1c1c] px-4 pb-4 pt-2 rounded gap-2">
        <H2>{{ t('source.add') }}</H2>
        <LongButton :text="t('source.type')">
          <template #extra>
            <QSelect v-model:value="sourceType" :options="sourceTypeOptions" />
          </template>
        </LongButton>
        <div class="w-full border-white/20 border-b" />
        <div class="flex flex-col gap-2">
          <QInputRow
            v-if="sourceType === 'local'" id="directory" v-model:value="directory" :label="t('source.directory')"
            type="directory"
          />
          <template v-if="sourceType === 'jellyfin'">
            <QInputRow id="server" v-model="server" :label="t('source.server')" type="url" placeholder="localhost:8096" />
            <QInputRow id="username" v-model="username" :label="t('source.user')" type="text" />
            <QInputRow id="password" v-model="password" :label="t('source.pwd')" type="password" />
          </template>
        </div>
        <div class="flex justify-end gap-2">
          <QButton :text="t('confirm')" @click="addAccount()" />
          <QButton :text="t('cancel')" @click="showAddModel = false" />
        </div>
      </div>
    </template>
  </Basic>
</template>

<style>
@keyframes rotating {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}
</style>
