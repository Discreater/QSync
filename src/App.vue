<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted } from 'vue';

onMounted(async () => {
  const result = await invoke('greet', { name: 'World' });

  // eslint-disable-next-line no-console
  console.log(result);
});
</script>

<template>
  <h1 class="text-center text-3xl font-bold underline">
    Hello World!
  </h1>
</template>
