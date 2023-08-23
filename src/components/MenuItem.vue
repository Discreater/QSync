<script setup lang="ts">
defineProps<{
  selected: boolean
  name?: string
  responsible?: boolean
}>();
</script>

<template>
  <div
    :class="`flex space-x-2 ${responsible ? 'sm:justify-center md:justify-start' : 'justify-start'} items-center
    ${responsible ? 'px-1.5 md:px-2.5' : 'px-2.5'}
    mb-1 h-10 rounded-md
    ${selected ? 'bg-gray-600/10' : ''} hover:bg-gray-500/10`"
  >
    <slot />
    <span v-if="name" :class="`cursor-default ${responsible ? 'hidden sm:hidden md:block' : 'block'}`">{{ name }}</span>
  </div>
</template>
