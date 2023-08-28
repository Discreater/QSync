<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';

import { useI18n } from 'vue-i18n';
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { useQSyncStore } from '~/store';
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
  }
});
const views = ref(store.musicFolders.flatMap(folder => folder.tracks));
onUnmounted(() => {
  if (container.value)
    scrollbar.value?.destroy();
});

function shufflePlay() {
  store.play(shuffle([...views.value]));
}

function playByIdx(idx: number) {
  store.play(views.value, idx);
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
            <div class="flex">
              <QHoverButton :icon="IconPlay" class="text-passion" @click="playByIdx(rowIdx)" />
            </div>
          </template>
          <template v-else-if="column.key === 'title'">
            <QHoverButton :text="row.title" class="hover:text-passion text-left" @click="handleTitleClick(row)" />
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
