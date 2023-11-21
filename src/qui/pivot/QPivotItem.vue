<script setup lang="ts">
defineProps<{ value: string; name: string }>();
</script>

<template>
  <div>
    <slot />
  </div>
</template>
