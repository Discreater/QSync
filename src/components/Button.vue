<script setup lang="ts">
import type { Component } from 'vue';

const { text, icon, disabled } = withDefaults(defineProps<{ text?: string; icon?: Component; disabled?: boolean }>(), {
  disabled: false,
});
</script>

<template>
  <button
    :disabled="disabled"
    class="bg-[#4b4b4b80] hover:enabled:bg-[#4b4b4bb0] disabled:opacity-25
          cursor-default flex items-center justify-between
          rounded-md px-2 h-8"
  >
    <Component :is="icon" v-if="icon" class="mr-2" />
    <span>
      {{ text }}
    </span>
  </button>
</template>
