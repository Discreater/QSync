<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  src: string
}>();

const loadFailed = ref(false);

watch(() => props.src, () => {
  loadFailed.value = false;
});

function handleImageError() {
  loadFailed.value = true;
}
</script>

<template>
  <img v-if="!loadFailed" :src="src" @error="handleImageError">
  <div v-else class="bg-menu_w_bg dark:bg-menu_d_bg">
    <slot name="failed" />
  </div>
</template>
