<script setup lang="ts">
defineProps<{
  selected: boolean
  name?: string
  onlyIcon?: boolean
}>();
</script>

<template>
  <div
    class="flex space-x-2 items-center h-10 mr-1 rounded hover:bg-gray-500/10"
    :class="`${onlyIcon ? 'sm:justify-center md:justify-start' : 'justify-start'}
    ${onlyIcon ? 'px-1.5 md:px-2.5' : 'px-2.5'}
    ${selected ? 'bg-gray-600/10' : ''}`"
  >
    <slot />
    <span v-if="name" :class="`cursor-default ${onlyIcon ? 'hidden sm:hidden md:block' : 'block'}`">{{ name }}</span>
  </div>
</template>
