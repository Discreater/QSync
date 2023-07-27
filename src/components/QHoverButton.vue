<script setup lang="ts">
import type { Component } from 'vue';

const { icon, text, disabled } = withDefaults(defineProps<{ icon?: Component; text?: string; disabled?: boolean }>(), {
  disabled: false,
});
</script>

<template>
  <button
    :disabled="disabled" :class="`hover:enabled:bg-[#4b4b4bb0] disabled:opacity-25
           flex items-center justify-between
          rounded-md px-2 ${icon ? 'h-10 w-10 cursor-default' : ''}`"
  >
    <Component :is="icon" v-if="icon" class="text-lg" />
    <span v-if="text">{{ text }}</span>
  </button>
</template>
