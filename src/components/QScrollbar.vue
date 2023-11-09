<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';
import { onMounted, onUnmounted, ref } from 'vue';

defineProps<{ contentClass?: string }>();

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
    <div :class="contentClass">
      <slot />
    </div>
  </div>
</template>

<style>
.container .scrollbar-track {
  background: transparent;
  transition: all 0.2s ease;
}

.container .scrollbar-track:hover {
  background: #ffffff20;
  border-radius: 4px;
}

.container .scrollbar-track:active {
  background: #ffffff20;
  border-radius: 4px;
}

.container .scrollbar-track-y:hover .scrollbar-thumb {
  width: 8px;
}

.container .scrollbar-track-y:active .scrollbar-thumb {
  width: 8px;
}

.container .scrollbar-track-x:hover .scrollbar-thumb {
  height: 8px;
}

.container .scrollbar-track-x:active .scrollbar-thumb {
  height: 8px;
}

.container .scrollbar-track .scrollbar-thumb {
  transition: all 0.2s ease;
  margin: auto;
  background: rgb(158, 160, 163);
}

.container .scrollbar-track-y .scrollbar-thumb {
  transition: all 0.2s ease;
  left: auto;
  right: 0;
  width: 2px;
}

.container .scrollbar-track-x .scrollbar-thumb {
  transition: all 0.2s ease;
  top: auto;
  bottom: 0;
  height: 2px;
}
</style>
