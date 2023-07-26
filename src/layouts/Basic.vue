<script setup lang="ts">
const { customPadding } = defineProps<{ customPadding?: boolean }>();
</script>

<template>
  <main :class="`relative h-full grow pt-12 ${customPadding ? '' : 'px-16'} overflow-auto flex flex-col`">
    <slot />
  </main>
</template>
