<script setup lang="ts">
const { customPadding } = defineProps<{ customPadding?: boolean }>();
</script>

<template>
  <main :class="`relative h-full grow pt-12 pb-1 ${customPadding ? 'px-1' : 'px-16'} overflow-auto flex flex-col`">
    <slot />
  </main>
</template>
