<script setup lang="ts">
import { Icon } from '@iconify/vue';
const props = defineProps<{
  icon: string
  name: string
}>();
</script>

<template>
  <div class="flex">
    <Icon :icon="icon" />
    <span>{{ name }}</span>
  </div>
</template>
