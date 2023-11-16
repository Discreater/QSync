<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { onBeforeMount, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useI18n } from 'vue-i18n';
import IconUnmaximized from '~icons/fluent/square-multiple-16-regular';
import IconMaximized from '~icons/fluent/maximize-16-regular';
import IconClose from '~icons/fluent/dismiss-16-regular';
import IconGoBack from '~icons/fluent/arrow-left-24-regular';
import IconMinimize from '~icons/qsync/minimize';

import QSyncIcon from '~/assets/icon.svg';
import { getPlatform, inTauri } from '~/platforms';

defineProps<{ dense?: boolean }>();

const route = useRoute();
const router = useRouter();

const { t } = useI18n();

const maximized = ref(false);

inTauri(() => {
  onBeforeMount(async () => {
    maximized.value = await appWindow.isMaximized();
  });
});

function onMinimize() {
  appWindow.minimize();
}
async function onToggleMaxmize() {
  await appWindow.toggleMaximize();
  maximized.value = await appWindow.isMaximized();
}

function onClose() {
  appWindow.close();
}

const canGoBack = ref(false);
watch(() => route.fullPath, () => {
  canGoBack.value = window.history.length !== 0 && window.history.state.back !== null;
});

function onGoBack() {
  if (canGoBack.value)
    router.back();
}
</script>

<template>
  <div data-tauri-drag-region class="h-title bg-transparent select-none cursor-default flex justify-start" :class="dense ? 'h-[32px]' : 'h-title'">
    <div class="flex justify-center items-center space-x-2" :class="dense ? '' : 'h-14'">
      <div class="flex justify-center items-center" :class="`${dense ? 'h-title w-12' : 'h-10 w-10 mx-1 rounded'} ${canGoBack ? 'hover:bg-gray-500/10' : 'opacity-25'}`" @click="onGoBack()">
        <IconGoBack />
      </div>
      <img data-tauri-drag-region :src="QSyncIcon" class="w-6" alt="QSync logo">
      <span data-tauri-drag-region class="text-sm leading-none">{{ t('app-title') }}</span>
    </div>
    <div v-if="getPlatform() === 'tauri'" class="flex ml-auto">
      <div id="titlebar-minimize" class="titlebar-button hover:bg-gray-500/10" @click="onMinimize()">
        <IconMinimize />
      </div>
      <div id="titlebar-maximize" class="titlebar-button hover:bg-gray-500/10" @click="onToggleMaxmize()">
        <IconUnmaximized v-if="maximized" />
        <IconMaximized v-else />
      </div>
      <div id="titlebar-close" class="titlebar-button close hover:text-white hover:bg-[#c42b1c]" @click="onClose()">
        <IconClose />
      </div>
    </div>
  </div>
</template>

<style scoped>
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 46px;
  height: 33px;
}
</style>
