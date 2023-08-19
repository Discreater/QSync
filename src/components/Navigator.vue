<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';

import { ref, watch } from 'vue';
import type { Item, ItemKey } from './types';

import QMenu from './QMenu.vue';
import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';
import IconAccount from '~icons/fluent/person-circle-24-regular';
import IconPLayback from '~icons/fluent/navigation-play-20-regular';
import { getPlatform } from '~/platforms';

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
    {
      key: 'playback',
      icon: IconPLayback,
      name: 'menu.playback',
    },
  ],
  bottom: [
    {
      key: 'source',
      icon: IconAccount,
      name: 'menu.source',
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
const activated = ref<ItemKey>();
watch(
  () => route.name,
  (name) => {
    activated.value = name ?? undefined;
  },
);

function onItemClick(item: Item) {
  router.push({ name: item.key });
}
</script>

<template>
  <div :class="`bg-menu_w_bg dark:bg-menu_d_bg  sm:w-12 md:w-[23rem] px-0 sm:px-1 ${getPlatform() !== 'web' ? 'pt-14' : 'pt-2'}`">
    <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" :responsible="true" @item-click="onItemClick" />
  </div>
</template>
