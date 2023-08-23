<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';

import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
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

const route = useRoute();
const router = useRouter();

const activated = ref<ItemKey | undefined>(route.name ?? undefined);
watch(
  () => route.name,
  (name) => {
    activated.value = name ?? undefined;
  },
);

function onItemClick(item: Item) {
  router.push({ name: item.key });
}

const searchText = ref('');
function search() {
  router.push({ name: 'search-result', query: { q: searchText.value } });
}
</script>

<template>
  <div
    :class="`bg-menu_w_bg dark:bg-menu_d_bg flex flex-col sm:w-12 md:w-[23rem] px-0 sm:px-1 ${getPlatform() !== 'web' ? 'pt-14' : 'pt-2'}`"
  >
    <QInput
      id="nav-search" v-model:value="searchText" class="mx-2.5 mb-3" type="text" :placeholder="t('nav.search')"
      @keyup.enter="search"
    >
      <template #extra>
        <QHoverButton :icon="IconSearch" size="small" :disabled="!searchText" @click="search" />
      </template>
    </QInput>
    <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" :responsible="true" @item-click="onItemClick" />
  </div>
</template>
