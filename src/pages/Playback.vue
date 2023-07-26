<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import QList from '~/components/QList.vue';
import QScrollbar from '~/components/QScrollbar.vue';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { ViewTrack } from '~/sources/folder';
import { useQSyncStore } from '~/store';

const { t } = useI18n();
const store = useQSyncStore();
const views = store.playbackQueue.queue.map(track => new ViewTrack(track));
</script>

<template>
  <Basic :custom-padding="true">
    <H1 class="px-16">
      {{ t('menu.playback') }}
    </H1>
    <QScrollbar class="flex-1 grow relative px-16">
      <QList :items="views" :key-map="(track) => track.path">
        <template #item="{ item, idx }">
          <div :class="`h-12 mb-2 rounded-md ring-1 ring-black/10 ${idx === store.playbackQueue.current ? 'text-orange-500' : ''}  ${idx % 2 === 0 ? 'bg-[#4b4b4b60]' : 'hover:bg-[#4b4b4b60]'}`">
            {{ item.name() }}
          </div>
        </template>
      </qlist>
    </QScrollbar>
  </Basic>
</template>
