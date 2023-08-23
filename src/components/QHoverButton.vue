<script setup lang="ts">
import type { Component } from 'vue';

type Size = 'small' | 'normal';

const props = withDefaults(defineProps<{ icon?: Component; text?: string; disabled?: boolean; size?: Size }>(), {
  disabled: false,
  size: 'normal',
});

const buttonClass = props.size === 'small' ? 'my-0.5' : 'h-10 w-10';
</script>

<template>
  <button
    :disabled="disabled" :class="`hover:enabled:bg-white/10 disabled:opacity-25
           flex items-center justify-between
           rounded-md px-2 ${icon ? (`${buttonClass} cursor-default`) : ''}`"
  >
    <Component :is="icon" v-if="icon" :class="size === 'normal' ? 'text-lg' : 'text-xs'" />
    <span v-if="text">{{ text }}</span>
  </button>
</template>
