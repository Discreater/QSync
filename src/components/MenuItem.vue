<script setup lang="ts">
const { selected, name } = defineProps<{
  selected: boolean
  name: string
}>();
</script>

<template>
  <div
    :class="`flex space-x-2 sm:justify-center md:justify-start items-center
    px-1.5 md:px-2.5
     mb-1 h-10 rounded-md
    ${selected ? 'bg-gray-600/10' : ''} hover:bg-gray-500/10`"
  >
    <slot />
    <span class="cursor-default hidden sm:hidden md:block">{{ name }}</span>
  </div>
</template>
