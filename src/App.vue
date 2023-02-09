<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import TitleBar from '~/components/TitleBar.vue';
import Navigator from '~/components/Navigator.vue';
import MusicPlayer from '~/components/MusicPlayer.vue';
import { defaultTheme } from '~/utils/theme';
const root = ref<HTMLElement>();

onMounted(async () => {
  const result = await invoke('greet', { name: 'World' });
  root.value = document.documentElement;
  const theme = defaultTheme;
  root.value.style.setProperty('--main', theme.main);
});
</script>

<template>
  <div class="w-full h-full flex flex-col text-black dark:text-white bg-main_w_bg dark:bg-main_d_bg">
    <div class="grow flex">
      <Navigator class="shrink-0" />
      <main class="grow pt-9">
        <RouterView />
      </main>
    </div>
    <TitleBar />
    <MusicPlayer class="shrink-0" />
  </div>
</template>
