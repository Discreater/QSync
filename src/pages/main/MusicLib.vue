<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';

import { useI18n } from 'vue-i18n';
import { onMounted, onUnmounted, ref } from 'vue';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { usePlayerStore, useQSyncStore } from '~/store';
import { ViewTrack } from '~/sources/folder';
import QButton from '~/components/QButton.vue';
import { pad, shuffle } from '~/utils';
import QHoverButton from '~/components/QHoverButton.vue';
import IconPlay from '~icons/fluent/play-24-regular';
import IconLocation from '~icons/fluent/my-location-24-regular';
import IconTop from '~icons/fluent/arrow-upload-24-regular';
import QTable from '~/components/QTable.vue';
import type { Column } from '~/components/QTable.vue';

const { t } = useI18n();
const store = useQSyncStore();
const playerStore = usePlayerStore();
const container = ref<HTMLDivElement | null>(null);
const scrollbar = ref<Scrollbar | null>(null);

onMounted(() => {
  if (container.value) {
    scrollbar.value = Scrollbar.init(container.value, {
      alwaysShowTracks: true,
    });
    setTimeout(() => {
      locateToPlaying();
    }, 50);
  }
});
const views = ref(store.musicFolders.flatMap(folder => folder.tracks.map(track => new ViewTrack(track))));
onUnmounted(() => {
  if (container.value)
    scrollbar.value?.destroy();
});

function shufflePlay() {
  store.play(shuffle([...views.value.map(v => v.raw)]));
}

function playByIdx(idx: number) {
  store.play(views.value.map(v => v.raw), idx);
}

function locateToPlaying() {
  if (container.value) {
    const playing = container.value.querySelector<HTMLElement>('.playing');
    if (playing) {
      scrollbar.value?.scrollIntoView(playing, {
        onlyScrollIfNeeded: true,
      });
    }
  }
}

function locateToTop() {
  if (container.value) {
    const playing = container.value.querySelector<HTMLElement>('.playing');
    if (playing)
      scrollbar.value?.scrollTo(0, 0, 600);
  }
}

function rowClassName(row: ViewTrack) {
  if (store.playbackQueue[playerStore.current]?.path === row.raw.path)
    return 'playing';
  return '';
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

function formatTime(time: number) {
  time = Math.floor(time / 1000);
  const minutes = pad(Math.floor(time / 60), 2);
  const seconds = pad(time % 60, 2);
  return `${minutes}:${seconds}`;
}
</script>

<template>
  <Basic :custom-padding="true">
    <H1 class="px-16">
      {{ t('music-lib.music') }}
    </H1>
    <div class="px-16 mb-4 flex gap-2">
      <QButton :icon="IconArrowShuffle" :text="t('btn.shuffle')" @click="shufflePlay()" />
      <QButton :icon="IconLocation" class="text-passion" :title="t('btn.location')" @click="locateToPlaying" />
      <QButton :icon="IconTop" class="text-passion" @click="locateToTop" />
    </div>
    <div ref="container" class="flex-1 overflow-auto grow relative container px-16">
      <QTable :columns="columns" :data="views" :row-class-name="rowClassName">
        <template #bodyCell="{ column, row, rowIdx }">
          <template v-if="column.key === 'actions'">
            <div>
              <QHoverButton :icon="IconPlay" @click="playByIdx(rowIdx)" />
            </div>
          </template>
          <template v-else-if="column.key === 'title'">
            <div>
              {{ row.name() }}
            </div>
          </template>
          <template v-else-if="column.key === 'artist'">
            <div>
              {{ row.artist() }}
            </div>
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
            {{ formatTime(row.duration()) }}
          </template>
        </template>
      </QTable>
    </div>
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
