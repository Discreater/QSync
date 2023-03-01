<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import type { Component } from 'vue';
import { computed } from 'vue';
import MenuItem from './MenuItem.vue';

import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';
import IconAccount from '~icons/fluent/person-circle-24-regular';

const { t } = useI18n();

interface Item {
  key: string
  icon: Component
  name: string
}

const menu = {
  top: [
    {
      key: 'home',
      icon: IconHome,
      name: 'menu.home',
    },
    {
      key: 'music-lib',
      icon: IconMusicNote,
      name: 'menu.music-lib',
    },
  ],
  bottom: [
    {
      key: 'account',
      icon: IconAccount,
      name: 'menu.account',
    },
    {
      key: 'settings',
      icon: IconSettings,
      name: 'menu.settings',
    },
  ],
};

const route = useRoute();
const router = useRouter();

function onItemClick(item: Item) {
  router.push({ name: item.key });
}

const handlerTop = computed(() => {
  let idx = menu.top.findIndex(item => item.key === route.name);
  if (idx !== -1)
    return `${(idx * 11 + 11) / 4}rem`;

  idx = menu.bottom.findIndex(item => item.key === route.name);
  if (idx !== -1)
    return `calc(100% - ${((menu.bottom.length - idx) * 11) / 4}rem)`;

  return 0;
});
</script>

<template>
  <div class="select-none relative hidden sm:flex md:flex flex-col justify-between sm:w-12 md:w-80 px-1 pt-11 bg-menu_w_bg dark:bg-menu_d_bg">
    <div
      class="transition-position duration-400 absolute w-[3px] h-5 my-2.5 rounded-md bg-main" :style="{
        top: handlerTop,
      }"
    />
    <div>
      <MenuItem
        v-for="item in menu.top" :key="item.key" :selected="item.key === route.name" :name="t(item.name)"
        @click="onItemClick(item)"
      >
        <Component :is="item.icon" />
      </MenuItem>
    </div>
    <div>
      <MenuItem
        v-for="item in menu.bottom" :key="item.key" :selected="item.key === route.name" :name="t(item.name)"
        @click="onItemClick(item)"
      >
        <Component :is="item.icon" />
      </MenuItem>
    </div>
  </div>
</template>
