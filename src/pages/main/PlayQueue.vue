<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Column } from '~qui/table/types';
import Basic from '~/layouts/Basic.vue';
import { useMusyncStore } from '~/store';
import QTable from '~qui/table/QTable.vue';
import QHoverButton from '~/components/QHoverButton.vue';
import IconPlay from '~icons/fluent/play-24-regular';
import { formatTime } from '~/utils';
import { TrackExt } from '~/model_ext/track';
import { usePlayerStore } from '~/store/player';

const { t } = useI18n();
const store = useMusyncStore();
const playerStore = usePlayerStore();
const views = store.playQueue;

function playByIdx(idx: number) {
  playerStore.play(idx);
}

const columns: Column[] = [
  {
    key: 'actions',
    style: {
      gridTemplateColumn: '48px',
    },
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
    style: {
      gridTemplateColumn: '56px',
      textAlign: 'right',
    },
  },
];
</script>

<template>
  <Basic :header="t('menu.play-queue')">
    <QTable :columns="columns" :data="views">
      <template #actions="{ rowIdx }">
        <QHoverButton class="text-passion h-8 w-8" @click="playByIdx(rowIdx)">
          <IconPlay class="text-base" />
        </QHoverButton>
      </template>
      <template #title="{ row }">
        <p class="truncate">
          {{ row.title }}
        </p>
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
      <template #duration="{ row }">
        {{ row.duration != null ? formatTime(TrackExt.durationInSecs(row.duration)) : '' }}
      </template>
    </QTable>
  </Basic>
</template>
