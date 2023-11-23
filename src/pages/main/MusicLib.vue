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
import QHoverText from '~/components/QHoverText.vue';
import IconPlay from '~icons/fluent/play-24-regular';
import IconLocation from '~icons/fluent/my-location-24-regular';
import IconTop from '~icons/fluent/arrow-upload-24-regular';
import type { Column } from '~qui/table/types';
import type { Track } from '~/generated/protos/musync';
import { usePlayerStore } from '~/store/player';
import QTable from '~qui/table/QTable.vue';

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
    style: {
      gridTemplateColumn: '56px',
    },
  },
  {
    key: 'title',
    title: 'Title',
    style: {
      gridTemplateColumn: 'minmax(0, 1.75fr)',
    },
  },
  {
    key: 'artist',
    title: 'Artist',
    style: {
    },
  },
  {
    key: 'album',
    title: 'Album',
    style: {
      hidePx: 576,
    },
  },
  {
    key: 'year',
    title: 'Year',
    style: {
      gridTemplateColumn: 'minmax(0, 56px)',
      textAlign: 'right',
      hidePx: 740,
    },
  },
  {
    key: 'genre',
    title: 'Genre',
    style: {
      hidePx: 704,
    },
  },
  {
    key: 'duration',
    title: 'Duration',
    style: {
      gridTemplateColumn: '56px',
      textAlign: 'right',
    },
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
    <QTable :columns="columns" :data="views" :row-class-name="rowClassName" :row-key="(row) => row.id">
      <template #actions="{ rowIdx }">
        <div class="flex invisible group-hover:visible">
          <QHoverButton class="text-passion h-8 w-8" @click="playByIdx(rowIdx)">
            <IconPlay class="text-base" />
          </QHoverButton>
        </div>
      </template>
      <template #title="{ row }">
        <QHoverText class="hover:text-passion truncate" @click="handleTitleClick(row)">
          {{ row.title }}
        </QHoverText>
      </template>
      <template #artist="{ row }">
        <p class="truncate">
          {{ row.artist }}
        </p>
      </template>
      <template #album="{ row }">
        <p class="truncate">
          {{ row.album }}
        </p>
      </template>
      <template #year="{ row }">
        {{ row.year }}
      </template>
      <template #genre="{ row }">
        <p class="truncate">
          {{ row.genre }}
        </p>
      </template>
      <template #duration="{ row }">
        {{ row.duration != null ? formatTime(Math.floor(row.duration / 1000)) : '' }}
      </template>
    </QTable>
  </Basic>
</template>

<style scoped>
:deep(.playing) {
  color: rgb(249 115 22);
}
</style>
