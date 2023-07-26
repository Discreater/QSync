<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';
import { onMounted, onUnmounted, ref } from 'vue';

const container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  if (container.value) {
    Scrollbar.init(container.value, {
      alwaysShowTracks: true,
    });
  }
});
onUnmounted(() => {
  if (container.value)
    Scrollbar.destroy(container.value!);
});
</script>

<template>
  <div ref="container" class="container overflow-auto">
    <slot />
  </div>
</template>
