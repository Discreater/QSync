<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';

import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import type { Item, ItemKey } from './types';

import QMenu from './QMenu.vue';
import QInput from './QInput.vue';
import QHoverButton from './QHoverButton.vue';
import IconHome from '~icons/fluent/home-24-regular';
import IconMusicNote from '~icons/fluent/music-note-2-24-regular';
import IconSettings from '~icons/fluent/settings-24-regular';
import IconAccount from '~icons/fluent/person-circle-24-regular';
import IconPlayQueue from '~icons/fluent/navigation-play-20-regular';
import IconAppList from '~icons/fluent/apps-list-24-regular';
import IconSearch from '~icons/fluent/search-12-regular';
import IconMenu from '~icons/fluent/line-horizontal-3-20-regular';
import { getPlatform } from '~/platforms';

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
    {
      key: 'play-queue',
      icon: IconPlayQueue,
      name: 'menu.play-queue',
    },
  ],
  bottom: [
    {
      key: 'account',
      icon: IconAccount,
      name: 'menu.account',
    },
    {
      key: 'source',
      icon: IconAppList,
      name: 'menu.source',
    },
    {
      key: 'settings',
      icon: IconSettings,
      name: 'menu.settings',
    },
  ],
};

const breakPoints = useBreakpoints(breakpointsTailwind);

const route = useRoute();
const router = useRouter();

const activated = ref<ItemKey | undefined>(route.name ?? undefined);
watch(
  () => route.name,
  (name) => {
    activated.value = name ?? undefined;
  },
);
const showMenuInPhone = ref(false);

function onItemClick(item: Item) {
  router.push({ name: item.key });
  showMenuInPhone.value = false;
}

const searchText = ref('');
function search() {
  router.push({ name: 'search-result', query: { q: searchText.value } });
}

const onlyIcon = breakPoints.sm;
const inPhone = breakPoints.smaller('sm');
const inTauri = getPlatform() === 'tauri';
</script>

<template>
  <div v-if="inPhone" class="w-full px-3 flex items-center">
    <QHoverButton @click="showMenuInPhone = !showMenuInPhone">
      <IconMenu class="text-lg" />
    </QHoverButton>
    <QInput
      id="nav-search" v-model:value="searchText" class="mx-2.5 grow" type="text" :placeholder="t('nav.search')"
      @keyup.enter="search"
    >
      <template #extra>
        <QHoverButton size="custom" :disabled="!searchText" @click="search">
          <IconSearch clip="text-xs" />
        </QHoverButton>
      </template>
    </QInput>
    <Teleport to="#qsync">
      <div
        v-if="showMenuInPhone" class="fixed inset-x-0 top-10 bottom-player p-1 bg-menu_bg"
        :class="inTauri ? 'mt-[32px]' : ''"
      >
        <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" @item-click="onItemClick" />
      </div>
    </Teleport>
  </div>
  <div
    v-else class="bg-menu_bg flex flex-col sm:w-12 md:w-80 px-0 sm:px-1 pt-1"
  >
    <QInput
      id="nav-search" v-model:value="searchText" class="mx-2.5 mb-3 hidden md:flex" type="text"
      :placeholder="t('nav.search')" @keyup.enter="search"
    >
      <template #extra>
        <QHoverButton size="custom" :disabled="!searchText" @click="search">
          <IconSearch text="text-xs" />
        </QHoverButton>
      </template>
    </QInput>
    <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" :only-icon="onlyIcon" @item-click="onItemClick" />
  </div>
</template>
