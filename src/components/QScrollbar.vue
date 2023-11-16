<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';
import { inject, onMounted, onUnmounted, ref } from 'vue';
import { qInnerScrollBarKey } from './injects';

defineProps<{ contentClass?: string }>();

// originally created by @DjSt3rios
// see: https://github.com/idiotWu/smooth-scrollbar/discussions/367
class ShiftScrollPlugin extends Scrollbar.ScrollbarPlugin {
  static pluginName = 'ShiftScroll';

  transformDelta(delta: any, fromEvent: any) {
    return /wheel/.test(fromEvent.type) && fromEvent.shiftKey ? { x: delta.y, y: delta.x } : delta;
  }
}
Scrollbar.use(ShiftScrollPlugin);

const scrollbar = ref<Scrollbar | null>(null);

const injectedScrollbar = inject<(s: Scrollbar) => void>(qInnerScrollBarKey);

const container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  if (container.value) {
    const sb = Scrollbar.init(container.value, {
      alwaysShowTracks: true,
      damping: 1,
    });
    scrollbar.value = sb;
    injectedScrollbar?.(sb);
  }
});

onUnmounted(() => {
  if (container.value)
    scrollbar.value?.destroy();
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
