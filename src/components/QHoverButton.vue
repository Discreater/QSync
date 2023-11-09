<script setup lang="ts">
type Size = 'small' | 'normal';

type Cursor = 'default' | 'pointer';

const props = withDefaults(defineProps<{ disabled?: boolean; size?: Size; cursor?: Cursor }>(), {
  disabled: false,
  size: 'normal',
  cursor: 'default',
});

const buttonClass = props.size === 'small' ? 'my-0.5' : 'h-10 w-10';
</script>

<template>
  <button
    :disabled="disabled" class="hover:enabled:bg-white/10 disabled:opacity-25
           flex items-center justify-between
           rounded px-2" :class="`${buttonClass} cursor-${cursor}`"
  >
    <slot />
    <!-- <Component :is="icon" v-if="icon" :class="size === 'normal' ? 'text-lg' : 'text-xs'" /> -->
  </button>
</template>
