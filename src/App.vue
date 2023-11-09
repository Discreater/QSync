<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import SmoothScrollbar from 'smooth-scrollbar';
import { useRoute } from 'vue-router';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import { getPlatform, inTauri } from './platforms';
import { useMusyncStore } from './store';
import QPlayer from './components/QPlayer.vue';
import TitleBar from '~/components/TitleBar.vue';
import { defaultTheme } from '~/utils/theme';

import { useLoading } from '~/logic';

// originally created by @DjSt3rios
// see: https://github.com/idiotWu/smooth-scrollbar/discussions/367
class ShiftScrollPlugin extends SmoothScrollbar.ScrollbarPlugin {
  static pluginName = 'ShiftScroll';

  transformDelta(delta: any, fromEvent: any) {
    return /wheel/.test(fromEvent.type) && fromEvent.shiftKey ? { x: delta.y, y: delta.x } : delta;
  }
}

SmoothScrollbar.use(ShiftScrollPlugin);

const { t } = useI18n();

const qsyncStore = useMusyncStore();

const loading = useLoading();

onMounted(async () => {
  inTauri(async () => {
    const _result = await invoke('greet', { name: 'World' });
  });
  const root = document.documentElement;
  const theme = defaultTheme;
  root.style.setProperty('--main', theme.main);
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
    id="qsync"
    class="w-full h-full max-h-screen flex flex-col
      text-sm text-black dark:text-white
      bg-main_w_bg dark:bg-main_d_bg"
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
