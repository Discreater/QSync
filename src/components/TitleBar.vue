<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { onBeforeMount, ref } from 'vue';

import IconUnmaximized from '~icons/fluent/square-multiple-16-regular';
import IconMaximized from '~icons/fluent/maximize-16-regular';
import IconClose from '~icons/fluent/dismiss-16-regular';

const maximized = ref(false);

onBeforeMount(async () => {
  maximized.value = await appWindow.isMaximized();
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
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div id="titlebar-minimize" class="titlebar-button hover:bg-gray-500/10" @click="onMinimize()">
      <!-- <Icon icon="fluent:subtract-16-regular"/> -->
      <!-- modified fluent:subtract-16-regular -->
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16">
        <rect width="10" height="0.5" x="3" y="7.5" fill="currentColor" rx=".5" />
      </svg>
    </div>
    <div id="titlebar-maximize" class="titlebar-button hover:bg-gray-500/10" @click="onToggleMaxmize()">
      <IconUnmaximized v-if="maximized" />
      <IconMaximized v-else />
    </div>
    <div id="titlebar-close" class="titlebar-button close hover:text-white hover:bg-[#c42b1c]" @click="onClose()">
      <IconClose />
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 33px;
  background: transparent;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 46px;
  height: 33px;
}
</style>
