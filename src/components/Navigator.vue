<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import MenuItem from './MenuItem.vue';

import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';

const { t } = useI18n();

const menu = [
  {
    key: 'home',
    icon: IconHome,
    name: t('menu.home'),
  },
  {
    key: 'music-lib',
    icon: IconMusicNote,
    name: t('menu.musiclib'),
  },
  {
    key: 'settings',
    icon: IconSettings,
    bottom: true,
    name: t('menu.settings'),
  },
];

const route = useRoute();
</script>

<template>
  <div class="hidden sm:flex md:flex sm:w-12 md:w-80 px-1 pt-9 flex-col bg-menu_w_bg dark:bg-menu_d_bg">
    <div class="absolute w-[3px] h-4 my-2.5 rounded-md bg-main" />
    <MenuItem
      v-for="item in menu" :key="item.key" :selected="item.key === route.name" :name="item.name"
      :class="item.bottom ? 'mt-auto' : ''"
    >
      <component :is="item.icon" />
    </MenuItem>
  </div>
</template>
