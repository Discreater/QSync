<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import SmoothScrollbar from 'smooth-scrollbar';
import { inTauri } from './platforms';
import { useQSyncStore } from './store';
import QPlayer from './components/QPlayer.vue';
import TitleBar from '~/components/TitleBar.vue';
import Navigator from '~/components/Navigator.vue';
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

const root = ref<HTMLElement>();

onMounted(async () => {
  inTauri(async () => {
    const _result = await invoke('greet', { name: 'World' });
  });
  root.value = document.documentElement;
  const theme = defaultTheme;
  root.value.style.setProperty('--main', theme.main);
});
const { locale: i18nLocale } = useI18n();

const store = useQSyncStore();
store.$subscribe((_mutation, state) => {
  i18nLocale.value = state.locale;
});
</script>

<template>
  <div
    class="w-full h-full max-h-screen flex flex-col
     text-black dark:text-white bg-main_w_bg dark:bg-main_d_bg border-white/10 border rounded-lg"
  >
    <div class="flex-1 flex overflow-hidden">
      <Navigator class="shrink-0" />
      <RouterView />
    </div>
    <TitleBar />
    <QPlayer class="shrink-0" />
  </div>
</template>
