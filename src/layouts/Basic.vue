<script setup lang="ts">
import { getPlatform } from '~/platforms';

defineProps<{ customPadding?: boolean }>();
</script>

<template>
  <main
    :class="`relative h-full grow ${getPlatform() !== 'web' ? 'pt-12' : 'pt-4'} pb-1 ${customPadding ? 'px-1' : 'px-16'} overflow-auto flex flex-col`"
  >
    <slot />
  </main>
</template>
