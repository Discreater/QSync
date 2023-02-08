<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import MenuItem from './MenuItem.vue';

import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';

const { t } = useI18n();
</script>

<template>
  <div class="hidden sm:flex md:flex sm:w-12 md:w-80 px-1 pt-9 flex-col bg-menu_w_bg dark:bg-menu_d_bg">
    <div class="absolute w-[3px] h-4 my-2.5 rounded-md bg-main" />
    <MenuItem :selected="true" :name="t('menu.home')">
      <IconHome />
    </MenuItem>
    <MenuItem :selected="false" :name="t('menu.musiclib')">
      <IconMusicNote />
    </MenuItem>

    <MenuItem :selected="false" :name="t('menu.settings')" class="mt-auto">
      <IconSettings />
    </MenuItem>
  </div>
</template>
