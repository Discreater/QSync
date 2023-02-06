<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted } from 'vue';
import TitleBar from './components/TitleBar.vue';
import { toggleDark } from './utils';

onMounted(async () => {
  const result = await invoke('greet', { name: 'World' });

  // eslint-disable-next-line no-console
  console.log(result);
});
</script>

<template>
  <div class="w-full h-full text-black dark:text-white">
    <TitleBar />
    <main class="w-full h-full bg-white_bg dark:bg-black_bg">
      <h1 class="text-center text-3xl font-bold underline">
        Hello World!
      </h1>
      <button @click="toggleDark()">
        toggle dark
      </button>
    </main>
  </div>
</template>
