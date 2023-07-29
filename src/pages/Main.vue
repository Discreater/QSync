<script setup lang="ts">
import Navigator from '~/components/Navigator.vue';
</script>

<template>
  <div class="flex-1 flex overflow-hidden">
    <Navigator class="shrink-0" />
    <RouterView />
  </div>
</template>
