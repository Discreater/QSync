<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import QScrollbar from '~/components/QScrollbar.vue';
import type { Column } from '~/components/QTable.vue';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { ViewTrack } from '~/sources/folder';
import { useQSyncStore } from '~/store';
import QTable from '~/components/QTable.vue';
import QHoverButton from '~/components/QHoverButton.vue';
import IconPlay from '~icons/fluent/play-24-regular';

const { t } = useI18n();
const store = useQSyncStore();
const views = store.playbackQueue.queue.map(track => new ViewTrack(track));

function playByIdx(idx: number) {
  store.$patch((state) => {
    state.playbackQueue.current = idx;
    state.playbackQueue.playing = true;
    state.playbackQueue.progress = 0;
  });
}

const columns: Column[] = [
  {
    key: 'actions',
  },
  {
    title: 'Title',
    key: 'title',
  },
  {
    title: 'Artist',
    key: 'artist',
  },
  {
    title: 'Album',
    key: 'album',
  },
  {
    title: 'Year',
    key: 'year',
  },
  {
    title: 'Genre',
    key: 'genre',
  },
  {
    key: 'duration',
    title: 'Duration',
  },
];
</script>

<template>
  <Basic :custom-padding="true">
    <H1 class="px-16">
      {{ t('menu.playback') }}
    </H1>
    <QScrollbar class="flex-1 grow relative px-16">
      <QTable :columns="columns" :data="views">
        <template #bodyCell="{ column, row, rowIdx }">
          <template v-if="column.key === 'actions'">
            <QHoverButton :icon="IconPlay" class="text-passion" @click="playByIdx(rowIdx)" />
          </template>
          <template v-else-if="column.key === 'title'">
            {{ row.name() }}
          </template>
          <template v-else-if="column.key === 'artist'">
            {{ row.artist() }}
          </template>
          <template v-else-if="column.key === 'album'">
            {{ row.album() }}
          </template>
          <template v-else-if="column.key === 'year'">
            {{ row.year() }}
          </template>
          <template v-else-if="column.key === 'genre'">
            {{ row.genre() }}
          </template>
          <template v-else-if="column.key === 'duration'">
            {{ row.duration() }}
          </template>
        </template>
      </QTable>
    </QScrollbar>
  </Basic>
</template>
