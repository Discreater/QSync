<script setup lang="ts">
defineProps<{
  selected: boolean
  name: string
}>();
</script>

<template>
  <div
    class="items-center h-10 leading-10 text-2xl rounded hover:text-[#d2d2d2] cursor-default" :class="selected ? '' : 'text-[#8e8e8e]'"
  >
    {{ name }}
  </div>
</template>
