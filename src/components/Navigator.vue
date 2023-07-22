<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';

import { ref, watch } from 'vue';
import type { Item, ItemKey } from './types';

import QMenu from './QMenu.vue';
import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';
import IconAccount from '~icons/fluent/person-circle-24-regular';

const { t } = useI18n();

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
  <div class="bg-menu_w_bg dark:bg-menu_d_bg  sm:w-12 md:w-[23rem] px-1 pt-14">
    <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" @item-click="onItemClick" />
  </div>
</template>
