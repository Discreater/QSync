<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';

import { useI18n } from 'vue-i18n';
import { onMounted, onUnmounted, ref } from 'vue';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { useQSyncStore } from '~/store';
import QList from '~/components/QList.vue';
import { ViewTrack } from '~/sources/folder';
import QButton from '~/components/QButton.vue';
import { shuffle } from '~/utils';

const { t } = useI18n();
const store = useQSyncStore();
const container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  if (container.value) {
    Scrollbar.init(container.value, {
      alwaysShowTracks: true,
    });
  }
});
const views = store.musicFolders.flatMap(folder => folder.tracks.map(track => new ViewTrack(track)));
onUnmounted(() => {
  if (container.value)
    Scrollbar.destroy(container.value!);
});

function shufflePlay() {
  store.play(shuffle([...views]));
}
</script>

<template>
  <Basic :custom-padding="true">
    <H1 class="px-16">
      {{ t('music-lib.music') }}
    </H1>
    <div class="px-16 mb-4">
      <QButton :icon="IconArrowShuffle" :text="t('btn.shuffle')" @click="shufflePlay()" />
    </div>
    <div ref="container" class="flex-1 overflow-auto grow relative container px-16">
      <QList :items="views" :key-map="(track) => track.path">
        <template #item="{ item, idx }">
          <div :class="`h-12 mb-2 rounded-md ring-1 ring-black/10  ${idx % 2 === 0 ? 'bg-[#4b4b4b60]' : 'hover:bg-[#4b4b4b60]'}`">
            {{ item.name() }}
          </div>
        </template>
      </QList>
    </div>
  </Basic>
</template>
