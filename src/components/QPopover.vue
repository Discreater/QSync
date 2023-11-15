<script setup lang="ts">
import { ref } from 'vue';
import { getPlatform } from '~/platforms';

const show = ref(false);
const inTauri = getPlatform() === 'tauri';
</script>

<template>
  <div class="relative">
    <div @click="show = !show">
      <slot />
    </div>
    <!-- mask -->
    <div
      v-if="show" class="fixed inset-0 bg-transparent w-screen h-screen" :class="inTauri ? 'top-title' : ''"
      @click="show = false"
    />
    <div
      v-show="show"
      class="absolute bottom-12 right-0 ring-1 ring-gray-500/10 dark:ring-black/10 bg-main_bg shadow-lg p-2 rounded transition-all"
    >
      <slot name="popover" />
    </div>
  </div>
</template>
