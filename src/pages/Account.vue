<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import Basic from '~/layouts/Basic.vue';
import H1 from '~/components/typo/H1.vue';
import H2 from '~/components/typo/H2.vue';
import Button from '~/components/Button.vue';

import IconAdd from '~icons/fluent/add-circle-24-regular';
import { useQSyncStore } from '~/store';
import QInput from '~/components/QInput.vue';
import { JellyfinClient } from '~/sources/jellyfin';

const { t } = useI18n();
const store = useQSyncStore();

const sourceType = ref<'jellyfin'>('jellyfin');
const server = ref<string | undefined>();
const username = ref<string | undefined>();
const password = ref<string | undefined>();

const showAddModel = ref(false);
function addAccount() {
  if (sourceType.value === 'jellyfin') {
    if (server.value !== undefined && username.value !== undefined && password.value !== undefined) {
      const opt = new JellyfinClient({
        server: server.value,
        user: username.value,
        pwd: password.value,
      }, store.deviceName, store.deviceId);
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
      <H1>{{ t("menu.account") }}</H1>
      <Button :icon="IconAdd" :text="t('account.add')" @click="showAddModel = true" />
    </div>
    <div>
      accounts
    </div>
    <div v-show="showAddModel" class="absolute inset-0 bg-[#4b4b4b80] flex justify-center items-center">
      <div class="flex flex-col bg-[#1c1c1c] px-4 pb-4 pt-2 rounded-md gap-2">
        <H2>{{ t('account.add') }}</H2>
        <div class="grid grid-cols-[1fr_3fr] gap-2">
          <label for="server">{{ t('account.server') }}: </label>
          <QInput id="server" v-model="server" type="url" placeholder="localhost:8096" />
          <label for="username">{{ t('account.user') }}: </label>
          <QInput id="username" v-model="username" type="text" />
          <label for="password">{{ t('account.pwd') }}: </label>
          <QInput id="password" v-model="password" type="password" />
        </div>
        <div class="flex justify-end gap-2">
          <Button :text="t('confirm')" @click="addAccount()" />
          <Button :text="t('cancel')" @click="showAddModel = false" />
        </div>
      </div>
    </div>
  </Basic>
</template>
