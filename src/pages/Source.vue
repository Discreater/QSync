<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import Basic from '~/layouts/Basic.vue';
import H1 from '~/components/typo/H1.vue';
import H2 from '~/components/typo/H2.vue';
import QButton from '~/components/QButton.vue';

import IconAdd from '~icons/fluent/add-circle-24-regular';
import { useQSyncStore } from '~/store';
import QInput from '~/components/QInput.vue';
import { JellyfinClient } from '~/sources/jellyfin';
import QSelect from '~/components/QSelect.vue';
import LongButton from '~/components/LongButton.vue';
import type { Item } from '~/components/types';

const { t } = useI18n();
const store = useQSyncStore();

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
function addAccount() {
  if (sourceType.value === 'jellyfin') {
    if (server.value !== undefined && username.value !== undefined && password.value !== undefined) {
      const opt = {
        server: server.value,
        user: username.value,
        pwd: password.value,
      };
      const client = new JellyfinClient(opt, store.deviceName, store.deviceId);
    }
  }

  // const client = new JellyfinClient({
  // }, store.deviceName, store.deviceId);
  // client.connect().then(() => {
  //   logger.info('jellyfin test success');
  // }).catch((reason) => {
  //   logger.error('jellyfin test error');
  //   logger.error(reason);
  // });
}
</script>

<template>
  <Basic>
    <div class="flex justify-between">
      <H1>{{ t("menu.source") }}</H1>
      <QButton :icon="IconAdd" :text="t('source.add')" @click="showAddModel = true" />
    </div>
    <div>
      accounts
    </div>
    <div v-show="showAddModel" class="absolute inset-0 bg-[#4b4b4b80] flex justify-center items-center">
      <div class="flex flex-col bg-[#1c1c1c] px-4 pb-4 pt-2 rounded-md gap-2">
        <H2>{{ t('source.add') }}</H2>
        <LongButton :text="t('source.type')">
          <template #extra>
            <QSelect v-model:value="sourceType" :options="sourceTypeOptions" />
          </template>
        </LongButton>
        <div class="grid grid-cols-[1fr_3fr] gap-2">
          <QInput v-if="sourceType === 'local'" id="directory" v-model="directory" :label="t('source.directory')" type="directory" />
          <QInput v-if="sourceType === 'jellyfin'" id="server" v-model="server" :label="t('source.server')" type="url" placeholder="localhost:8096" />
          <QInput v-if="sourceType === 'jellyfin'" id="username" v-model="username" :label="t('source.user')" type="text" />
          <QInput v-if="sourceType === 'jellyfin'" id="password" v-model="password" :label="t('source.pwd')" type="password" />
        </div>
        <div class="flex justify-end gap-2">
          <QButton :text="t('confirm')" @click="addAccount()" />
          <QButton :text="t('cancel')" @click="showAddModel = false" />
        </div>
      </div>
    </div>
  </Basic>
</template>
