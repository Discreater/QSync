<script setup lang="ts">
import { ref } from 'vue';

const show = ref(false);
</script>

<template>
  <div class="relative">
    <div @click="show = !show">
      <slot />
    </div>
    <div v-show="show" class="absolute bottom-10 right-0 ring ring-black/10 bg-main_d_bg shadow-lg p-4 rounded-lg transition-all">
      <slot name="popover" />
    </div>
  </div>
</template>
