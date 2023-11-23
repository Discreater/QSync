<script setup lang="ts">
defineProps<{
  selected: boolean
  name?: string
  onlyIcon?: boolean
}>();
</script>

<template>
  <div
    class="flex space-x-2 items-center h-10 rounded hover:bg-gray-500/10 px-2.5 justify-start"
    :class="selected ? 'bg-gray-600/10' : ''"
  >
    <slot />
    <span v-if="name && !onlyIcon">{{ name }}</span>
  </div>
</template>
