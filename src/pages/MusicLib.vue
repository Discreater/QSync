<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';

import { useI18n } from 'vue-i18n';
import { h, onMounted, onUnmounted, ref } from 'vue';
import type { DataTableColumns, GlobalThemeOverrides } from 'naive-ui';
import { NConfigProvider, NDataTable } from 'naive-ui';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { useQSyncStore } from '~/store';
import { ViewTrack } from '~/sources/folder';
import QButton from '~/components/QButton.vue';
import { pad, shuffle } from '~/utils';
import QHoverButton from '~/components/QHoverButton.vue';
import IconPlay from '~icons/fluent/play-24-filled';
import IconLocation from '~icons/fluent/my-location-24-regular';
import IconTop from '~icons/fluent/arrow-upload-24-regular';

const { t } = useI18n();
const store = useQSyncStore();
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
const views = store.musicFolders.flatMap(folder => folder.tracks.map(track => new ViewTrack(track)));
onUnmounted(() => {
  if (container.value)
    scrollbar.value?.destroy();
});

function shufflePlay() {
  store.play(shuffle([...views.map(v => v.raw)]));
}

function playByIdx(idx: number) {
  store.$patch((state) => {
    state.playbackQueue.current = idx;
    state.playbackQueue.queue = views.map(v => v.raw);
    state.playbackQueue.playing = true;
    state.playbackQueue.progress = 0;
  });
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
  if (store.playbackQueue.queue[store.playbackQueue.current]?.path === row.raw.path)
    return 'playing';
  return '';
}

function createColumns(): DataTableColumns<ViewTrack> {
  return [
    {
      key: 'actions',
      width: 60,
      render(row, idx) {
        return h(QHoverButton, {
          icon: IconPlay,
          onClick: () => playByIdx(idx),
        });
      },
    },
    {
      title: 'Title',
      key: 'title',
      resizable: true,
      render(row) {
        return row.name();
      },
      sorter(row1, row2) {
        return row1.name().localeCompare(row2.name());
      },
    },
    {
      title: 'Artist',
      key: 'artist',
      resizable: true,
      render(row) {
        return row.artist();
      },
      sorter(row1, row2) {
        return row1.artist().localeCompare(row2.artist());
      },
    },
    {
      title: 'Album',
      key: 'album',
      resizable: true,
      maxWidth: 120,
      render(row) {
        return row.album();
      },
      sorter(row1, row2) {
        return row1.album().localeCompare(row2.album());
      },
    },
    {
      title: 'Year',
      key: 'year',
      width: 60,
      render(row) {
        return row.year();
      },
      sorter(row1, row2) {
        if (row1.year() && row2.year())
          return row1.year()! - row2.year()!;
        if (row1.year())
          return 1;
        if (row2.year())
          return -1;
        return 0;
      },
      ellipsis: {
        tooltip: true,
      },
    },
    {
      title: 'Genre',
      key: 'genre',
      width: 70,
      render(row) {
        return row.genre();
      },
    },
    {
      key: 'duration',
      title() {
        return h(
          'div',
          {
            class: 'text-right w-full',
          },
          'Duration',
        );
      },
      width: 120,
      render(row) {
        const duration = Math.floor(row.duration() / 1000);
        const minutes = pad(Math.floor(duration / 60), 2);
        const seconds = pad(duration % 60, 2);
        const strValue = `${minutes}:${seconds}`;
        return h(
          'div',
          {
            class: 'text-right w-full',
          },
          strValue,
        );
      },
    },
  ];
}
const themeOverrides: GlobalThemeOverrides = {
  DataTable: {
    borderColor: 'transparent',
    tdColor: 'rgb(39 39 39)',
    tdColorHover: '#4b4b4b60',
    tdColorStriped: '#4b4b4b60',
    tdTextColor: 'white',
    thColor: 'rgb(39 39 39)',
    thColorHover: '#transparent',
    thTextColor: 'rgb(249 115 22)',
    borderRadius: '0.375rem',
  },
};
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
      <NConfigProvider :theme-overrides="themeOverrides">
        <NDataTable :columns="createColumns()" :data="views" :row-class-name="rowClassName" class="text-sm" striped />
      </NConfigProvider>
    </div>
  </Basic>
</template>

<style scoped>
:deep(.playing td) {
  color: rgb(249 115 22);
}
</style>
