<script setup lang="ts">
import type { Component } from 'vue';

const { icon, disabled } = withDefaults(defineProps<{ icon?: Component; disabled?: boolean }>(), {
  disabled: false,
});
</script>

<template>
  <button
    :disabled="disabled"
    class="hover:enabled:bg-[#4b4b4bb0] disabled:opacity-25
          cursor-default flex items-center justify-between
          rounded-md px-2 h-10 w-10"
  >
    <Component :is="icon" v-if="icon" class="text-lg" />
  </button>
</template>
