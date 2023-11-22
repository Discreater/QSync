<script setup lang="ts">
type Size = 'custom' | 'fixed';

type Cursor = 'default' | 'pointer';

const props = withDefaults(defineProps<{
  disabled?: boolean
  size?: Size
  cursor?: Cursor
}>(), {
  disabled: false,
  size: 'fixed',
  cursor: 'default',
});

const buttonClass = props.size === 'custom' ? 'my-0.5' : 'h-10 w-10';
</script>

<template>
  <button
    :disabled="disabled" class="hover:enabled:bg-black/10 dark:hover:enabled:bg-white/10 disabled:opacity-25
           flex items-center justify-between
           rounded px-2" :class="`${buttonClass} cursor-${cursor}`"
  >
    <slot />
  </button>
</template>
