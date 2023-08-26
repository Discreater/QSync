<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import SmoothScrollbar from 'smooth-scrollbar';
import { useRoute } from 'vue-router';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import { getPlatform, inTauri } from './platforms';
import { useQSyncStore } from './store';
import QPlayer from './components/QPlayer.vue';
import { WsClient } from './api/client';
import { usePlayerStore } from './store/player';
import TitleBar from '~/components/TitleBar.vue';
import { defaultTheme } from '~/utils/theme';

// originally created by @DjSt3rios
// see: https://github.com/idiotWu/smooth-scrollbar/discussions/367
class ShiftScrollPlugin extends SmoothScrollbar.ScrollbarPlugin {
  static pluginName = 'ShiftScroll';

  transformDelta(delta: any, fromEvent: any) {
    return /wheel/.test(fromEvent.type) && fromEvent.shiftKey ? { x: delta.y, y: delta.x } : delta;
  }
}

SmoothScrollbar.use(ShiftScrollPlugin);

const playerStore = usePlayerStore();
const qsyncStore = useQSyncStore();

onMounted(async () => {
  inTauri(async () => {
    const _result = await invoke('greet', { name: 'World' });
  });
  const root = document.documentElement;
  const theme = defaultTheme;
  root.style.setProperty('--main', theme.main);
});
const { locale: i18nLocale } = useI18n();

const listen1 = WsClient.listenOnUpdatePlayer((update) => {
  playerStore.updateFromRemote(update);
});
const listen2 = WsClient.listenOnUpdatePlayQueue((update) => {
  qsyncStore.updatePlayQueueFromRemote(update.trackIds);
});

onUnmounted(() => {
  WsClient.cancelOnUpdatePlayer(listen1);
  WsClient.cancelOnUpdatePlayQueue(listen2);
});

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
    :class="`w-full h-full max-h-screen flex flex-col
     text-black dark:text-white bg-main_w_bg dark:bg-main_d_bg ${getPlatform() !== 'web' ? 'border-white/10 border' : ''} `"
  >
    <RouterView />
    <TitleBar v-if="getPlatform() !== 'web'" :dense="denseTitle" />
    <QPlayer class="shrink-0" />
  </div>
</template>
