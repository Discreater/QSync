<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import { getPlatform, inTauri } from './platforms';
import { useMusyncStore } from './store';
import QPlayer from './components/QPlayer.vue';
import { useTheme } from './logic/theme';
import TitleBar from '~/components/TitleBar.vue';

import { useLoading } from '~/logic';

const { t } = useI18n();

const qsyncStore = useMusyncStore();

const loading = useLoading();
// create default theme
const _theme = useTheme();

onMounted(async () => {
  inTauri(async () => {
    const _result = await invoke('greet', { name: 'World' });
  });
});
const { locale: i18nLocale } = useI18n();

qsyncStore.$subscribe((_mutation, state) => {
  i18nLocale.value = state.locale;
});

const breakPoints = useBreakpoints(breakpointsTailwind);
const inPhone = breakPoints.smaller('sm');

const route = useRoute();
const denseTitle = computed(() => route.name === 'lyric' || inPhone.value);
</script>

<template>
  <div
    id="qsync" class="
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
      <RouterView />
      <TitleBar v-if="getPlatform() !== 'web'" :dense="denseTitle" />
      <QPlayer class="shrink-0" />
    </template>
  </div>
</template>
