<script setup lang="ts">
import { bgLayer, useLayerLevel } from './injects';

const props = withDefaults(defineProps<{ disabled?: boolean; layer?: number }>(), {
  disabled: false,
});

const level = useLayerLevel(props.layer);
</script>

<template>
  <button
    :disabled="disabled"
    class="
        hover:enabled:bg-[#f5f4f4b0] dark:hover:enabled:bg-[#4b4b4bb0]
          disabled:opacity-25
          cursor-default
          flex items-center justify-between
          px-3 gap-2 h-10
          rounded ring-1 ring-black/10 dark:ring-white/20"
    :class="bgLayer(level)"
  >
    <slot />
  </button>
</template>
