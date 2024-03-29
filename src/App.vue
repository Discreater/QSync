<script setup lang="ts">
import { invoke } from '@tauri-apps/api/primitives';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { useBreakpoints } from '@vueuse/core';
import { getPlatform, inTauri } from './platforms';
import { useMusyncStore } from './store';
import QPlayer from './components/QPlayer.vue';
import QConfigProvider from './components/QConfigProvider.vue';
import { isDark } from './utils';
import { breakpoints } from '~/theme';
import { defaultDarkTheme, defaultTheme } from '~/theme.js';
import TitleBar from '~/components/TitleBar.vue';

import { useLoading } from '~/logic';

const { t } = useI18n();

const qsyncStore = useMusyncStore();

const loading = useLoading();

onMounted(async () => {
  inTauri(async () => {
    const _result = await invoke('greet', { name: 'World' });
  });
});
const { locale: i18nLocale } = useI18n();

qsyncStore.$subscribe((_mutation, state) => {
  i18nLocale.value = state.locale;
});

const breakPoints = useBreakpoints(breakpoints);
const inPhone = breakPoints.smaller('sm');

const route = useRoute();
const denseTitle = computed(() => route.name === 'lyric' || inPhone.value);
const theme = computed(() => {
  return isDark.value ? defaultDarkTheme : defaultTheme;
});
</script>

<template>
  <QConfigProvider
    id="qsync" :theme="theme" class="
      w-full h-full max-h-screen flex flex-col
      text-sm text-black dark:text-white selection:bg-passion selection:text-white
      bg-main_bg"
  >
    <div v-if="loading" class="w-full h-full flex justify-center items-center">
      <p>
        {{ t('loading') }}
      </p>
    </div>
    <template v-else>
      <TitleBar v-if="getPlatform() !== 'web'" :dense="denseTitle" />
      <RouterView />
      <QPlayer class="shrink-0" />
    </template>
  </QConfigProvider>
</template>
