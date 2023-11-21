<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import Basic from '~/layouts/Basic.vue';
import { useMusyncStore } from '~/store';
import QButton from '~/components/QButton.vue';
import { formatTime, shuffle } from '~/utils';
import QHoverButton from '~/components/QHoverButton.vue';
import IconPlay from '~icons/fluent/play-24-regular';
import IconLocation from '~icons/fluent/my-location-24-regular';
import IconTop from '~icons/fluent/arrow-upload-24-regular';
import type { Column } from '~/components/QTable.vue';
import type { Track } from '~/generated/protos/musync';
import { usePlayerStore } from '~/store/player';
import QTable from '~/components/QTable.vue';

import { useInnerScrollbar } from '~/components/injects';
import { logger } from '~/utils/logger';

const { t } = useI18n();
const store = useMusyncStore();
const playerStore = usePlayerStore();

const scrollbar = useInnerScrollbar();

const views = ref(store.musicFolders.flatMap(folder => folder.tracks));

function shufflePlay() {
  logger.warn('shuffle play');
  store.play(shuffle([...views.value]));
}

function playByIdx(idx: number) {
  store.play(views.value, idx);
}

function locateToPlaying() {
  if (scrollbar.value) {
    const playing = document.querySelector<HTMLElement>('.playing');
    if (playing) {
      scrollbar.value.scrollIntoView(playing, {
        onlyScrollIfNeeded: true,
      });
    }
  }
}

function locateToTop() {
  scrollbar.value?.scrollTo(0, 0, 600);
}

function rowClassName(row: Track) {
  if (store.playQueue[playerStore.position]?.id === row.id)
    return 'playing';
  return '';
}

const columns: Column[] = [
  {
    key: 'actions',
  },
  {
    key: 'title',
    title: 'Title',
  },
  {
    key: 'artist',
    title: 'Artist',
  },
  {
    key: 'album',
    title: 'Album',
  },
  {
    key: 'duration',
    title: 'Duration',
  },
];

const router = useRouter();

function handleTitleClick(track: Track) {
  router.push({ name: 'track', query: { id: track.id } });
}
</script>

<template>
  <Basic :header="t('music-lib.music')">
    <template #actions>
      <div class="mb-4 flex gap-2">
        <QButton @click="shufflePlay()">
          <IconArrowShuffle class="text-xl" />
          {{ t('btn.shuffle') }}
        </QButton>
        <QButton class="text-passion" @click="locateToPlaying">
          <IconLocation class="text-xl" />
        </QButton>
        <QButton class="text-passion" @click="locateToTop">
          <IconTop class="text-xl" />
        </QButton>
      </div>
    </template>
    <QTable :columns="columns" :data="views" :row-class-name="rowClassName">
      <template #bodyCell="{ column, row, rowIdx }">
        <template v-if="column.key === 'actions'">
          <div class="flex">
            <QHoverButton size="custom" class="text-passion h-8 w-8" @click="playByIdx(rowIdx)">
              <IconPlay class="text-base" />
            </QHoverButton>
          </div>
        </template>
        <template v-else-if="column.key === 'title'">
          <QHoverButton size="custom" class="hover:text-passion text-left" @click="handleTitleClick(row)">
            {{ row.title }}
          </QHoverButton>
        </template>
        <template v-else-if="column.key === 'artist'">
          <div>
            {{ row.artist }}
          </div>
        </template>
        <template v-else-if="column.key === 'album'">
          {{ row.album }}
        </template>
        <template v-else-if="column.key === 'year'">
          {{ row.year }}
        </template>
        <template v-else-if="column.key === 'genre'">
          {{ row.genre }}
        </template>
        <template v-else-if="column.key === 'duration'">
          {{ row.duration != null ? formatTime(Math.floor(row.duration / 1000)) : '' }}
        </template>
      </template>
    </QTable>
  </Basic>
</template>

<style scoped>
:deep(.playing td) {
  color: rgb(249 115 22);
}

:deep(td[data-col-key="actions"] button) {
  visibility: hidden;
}

:deep(tr:hover td[data-col-key="actions"] button) {
  visibility: visible;
}
</style>
