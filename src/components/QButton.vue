<script setup lang="ts">
import type { Component } from 'vue';
import IconDown from '~icons/fluent/chevron-down-24-regular';

const { text, icon, dropdown, disabled } = withDefaults(defineProps<{ text?: string; icon?: Component; dropdown?: boolean; disabled?: boolean }>(), {
  dropdown: false,
  disabled: false,
});
</script>

<template>
  <button
    :disabled="disabled"
    class="bg-[#4b4b4b80] hover:enabled:bg-[#4b4b4bb0] disabled:opacity-25
          cursor-default flex items-center justify-between
          rounded-md px-3 gap-2 h-10"
  >
    <Component :is="icon" v-if="icon" class="text-xl" />
    <span v-if="text">
      {{ text }}
    </span>
    <IconDown v-if="dropdown" />
  </button>
</template>
