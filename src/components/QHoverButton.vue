<script setup lang="ts">
withDefaults(defineProps<{
  disabled?: boolean
}>(), {
  disabled: false,
});
</script>

<template>
  <button
    :disabled="disabled"
    class="hover:enabled:bg-black/10 dark:hover:enabled:bg-white/10 disabled:opacity-25
           flex items-center justify-center
           rounded px-2 cursor-default"
  >
    <slot />
  </button>
</template>
