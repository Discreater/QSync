<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Column } from '~/components/QTable.vue';
import Basic from '~/layouts/Basic.vue';
import { useMusyncStore } from '~/store';
import QTable from '~/components/QTable.vue';
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
</script>

<template>
  <Basic :header="t('menu.play-queue')">
    <QTable :columns="columns" :data="views">
      <template #bodyCell="{ column, row, rowIdx }">
        <template v-if="column.key === 'actions'">
          <QHoverButton size="custom" class="text-passion h-8 w-8" @click="playByIdx(rowIdx)">
            <IconPlay class="text-base" />
          </QHoverButton>
        </template>
        <template v-else-if="column.key === 'title'">
          {{ row.title }}
        </template>
        <template v-else-if="column.key === 'artist'">
          {{ row.artist }}
        </template>
        <template v-else-if="column.key === 'album'">
          {{ row.album }}
        </template>
        <template v-else-if="column.key === 'duration'">
          {{ row.duration != null ? formatTime(TrackExt.durationInSecs(row.duration)) : '' }}
        </template>
      </template>
    </QTable>
  </Basic>
</template>
