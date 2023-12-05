<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';

import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useBreakpoints } from '@vueuse/core';
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
import { breakpoints } from '~/theme';

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

const breakPoints = useBreakpoints(breakpoints);

const route = useRoute();
const router = useRouter();

const activated = ref<ItemKey | undefined>(route.name ?? undefined);
watch(
  () => route.name,
  (name) => {
    activated.value = name ?? undefined;
  },
);

const searchText = ref('');

const inTauri = getPlatform() === 'tauri';

const sizeSm = breakPoints.smaller('sm');
const sizeMd = breakPoints.between('sm', 'md');
const sizeLg = breakPoints.greater('md');

// valid when sizeSm
const showMenu = ref(false);
// valid when sizeMd
const expandMenu = ref(false);
// valid when sizeLg
const shrinkMenu = ref(false);

function onMenuScale() {
  if (sizeLg.value)
    shrinkMenu.value = !shrinkMenu.value;
  else if (sizeMd.value)
    expandMenu.value = !expandMenu.value;
}

function onMenuScaleFromSearchButton() {
  if (sizeLg.value)
    shrinkMenu.value = true;
  else if (sizeMd.value)
    expandMenu.value = true;

  setTimeout(() => {
    const input = document.getElementById('nav-search') as HTMLInputElement;
    input?.focus();
  }, 100);
}
const onlyIcon = computed(() => { return sizeLg.value && shrinkMenu.value || sizeMd.value && !expandMenu.value; });

const delayedOnlyIcon = ref(onlyIcon.value);
watch(onlyIcon, (value, oldValue) => {
  if (oldValue && !value) {
    setTimeout(() => {
      delayedOnlyIcon.value = false;
    }, 100);
  } else {
    delayedOnlyIcon.value = value;
  }
});

function onItemClick(item: Item) {
  router.push({ name: item.key });
  showMenu.value = false;
  expandMenu.value = false;
}

function search() {
  router.push({ name: 'search-result', query: { q: searchText.value } });
  showMenu.value = false;
  expandMenu.value = false;
}
</script>

<template>
  <div v-if="sizeSm" class="w-full px-3 flex items-center">
    <QHoverButton class="h-10 w-10" @click="showMenu = !showMenu">
      <IconMenu class="text-lg" />
    </QHoverButton>
    <QInput
      id="nav-search" v-model:value="searchText" class="mx-2.5 grow" type="text" :placeholder="t('nav.search')"
      @keyup.enter="search"
    >
      <template #extra>
        <QHoverButton class="py-1" :disabled="!searchText" @click="search">
          <IconSearch class="text-2xs" />
        </QHoverButton>
      </template>
    </QInput>
    <Teleport to="#qsync">
      <div
        v-if="showMenu" class="fixed inset-x-0 top-10 bottom-player p-1 bg-menu_bg"
        :class="inTauri ? 'mt-[32px]' : ''"
      >
        <QMenu :activated="activated" :top="menu.top" :bottom="menu.bottom" @item-click="onItemClick" />
      </div>
    </Teleport>
  </div>
  <div
    v-else class="shrink-0 bg-menu_bg flex flex-col space-y-2 px-1 transition-shape"
    :class="`
    ${sizeLg ? (shrinkMenu ? 'w-12' : 'w-80')
    : sizeMd ? (expandMenu ? 'w-80' : 'w-12')
    : 'w-0'}
    ${sizeMd && expandMenu ? 'bg-[#fafafa] dark:bg-[#2d2d2d] ring-1 ring-black/10 rounded-r-WINDOW' : ''}
        `"
  >
    <QHoverButton class="shrink-0 h-9 w-10" @click="onMenuScale">
      <IconMenu class="text-base" />
    </QHoverButton>
    <div class="shrink-0 h-10 flex items-center">
      <QInput
        v-if="!onlyIcon"
        id="nav-search" v-model:value="searchText" class="mx-2.5 w-full" type="text"
        :placeholder="t('nav.search')" @keyup.enter="search"
      >
        <template #extra>
          <QHoverButton class="py-1" :disabled="!searchText" @click="search">
            <IconSearch class="text-2xs" />
          </QHoverButton>
        </template>
      </QInput>
      <QHoverButton v-else class="h-10 w-10" @click="onMenuScaleFromSearchButton">
        <IconSearch class="text-sm" />
      </QHoverButton>
    </div>
    <QMenu
      class="mt-4" :activated="activated" :top="menu.top" :bottom="menu.bottom"
      :only-icon="delayedOnlyIcon" @item-click="onItemClick"
    />
  </div>
</template>
