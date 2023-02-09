<script setup lang="ts">
const { selected, name } = defineProps<{
  selected: boolean
  name: string
}>();
</script>

<template>
  <div
    :class="`flex space-x-2 sm:justify-center md:justify-start items-center
    px-1.5 mb-1 h-9 rounded-md
    ${selected ? 'bg-gray-600/20' : ''} hover:bg-gray-500/20`"
  >
    <slot />
    <span class="cursor-default hidden sm:hidden md:block">{{ name }}</span>
  </div>
</template>
