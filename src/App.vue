<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import TitleBar from './components/TitleBar.vue';
import Navigator from './components/Navigator.vue';
import MusicPlayer from './components/MusicPlayer.vue';
import { toggleDark } from './utils';

const { locale } = useI18n();

onMounted(async () => {
  const result = await invoke('greet', { name: 'World' });

  // eslint-disable-next-line no-console
  console.log(result);
});
</script>

<template>
  <div class="w-full h-full flex flex-col text-black dark:text-white bg-main_w_bg dark:bg-main_d_bg">
    <TitleBar />
    <div class="grow flex">
      <Navigator class="hidden sm:block md:block md:w-80" />
      <main class="grow">
        <h1 class="text-center text-3xl font-bold underline">
          Hello World!
        </h1>
        <button @click="toggleDark()">
          toggle dark
        </button>
        <p class="block sm:hidden md:hidden">
          small
        </p>
        <p class="hidden sm:block md:hidden">
          middle
        </p>
        <p class="hidden sm:hidden md:block">
          large
        </p>
        <select v-model="locale">
          <option value="en">
            English
          </option>
          <option value="zh-CN">
            简体中文
          </option>
        </select>
      </main>
    </div>
    <MusicPlayer />
  </div>
</template>
