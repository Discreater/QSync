<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import LongButton from '~/components/LongButton.vue';
import H2 from '~/components/typo/H2.vue';
import Basic from '~/layouts/Basic.vue';
import IconConnected from '~icons/fluent/plug-connected-24-regular';
import IconDisconnected from '~icons/fluent/plug-disconnected-24-regular';
import { useAccountStore } from '~/store/user';
import QButton from '~/components/QButton.vue';
import QInput from '~/components/QInput.vue';

const { t } = useI18n();
const accountStore = useAccountStore();
const username = ref<string>('');

const online = computed(() => accountStore.online);
</script>

<template>
  <Basic :header="t('menu.account')">
    <H2>{{ t('account.qsync') }}</H2>
    <LongButton :icon="online ? IconConnected : IconDisconnected" :text="online ? accountStore.username : t('account.not-login')">
      <template #extra>
        <QInput v-if="!online" id="username" v-model:value="username" type="text" :placeholder="t('account.username')" />
        <QButton
          :text="online ? t('account.logout') : t('account.login')"
          @click="online ? accountStore.logout() : accountStore.login(username)"
        />
      </template>
    </LongButton>
  </Basic>
</template>
