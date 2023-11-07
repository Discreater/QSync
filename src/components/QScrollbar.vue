<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';
import { onMounted, onUnmounted, ref } from 'vue';

const scrollbar = ref<Scrollbar | null>(null);

const container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  if (container.value) {
    scrollbar.value = Scrollbar.init(container.value, {
      alwaysShowTracks: true,
    });
  }
});
onUnmounted(() => {
  if (container.value)
    scrollbar.value?.destroy();
});

defineExpose({
  scrollbar,
});
</script>

<template>
  <div ref="container" class="container overflow-auto">
    <slot />
  </div>
</template>
