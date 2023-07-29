<script setup lang="ts">
import { ref } from 'vue';
import { onBeforeRouteUpdate } from 'vue-router';
import type { ViewTrack } from '~/sources/folder';
import { useQSyncStore } from '~/store';

const props = defineProps<{ path?: string }>();
const viewTrack = ref<ViewTrack>();
const store = useQSyncStore();

onBeforeRouteUpdate((to, from, next) => {
  if (to.query.path !== from.query.path) {
    store.getShowTrack(to.query.path as string).then((track) => {
      viewTrack.value = track;
    });
  }
  next();
});

if (props.path) {
  store.getShowTrack(props.path as string).then((track) => {
    viewTrack.value = track;
  });
}
</script>

<template>
  <main
    class="track-pic-bg  h-full w-full mt-[32px] bg-cover" :style="{
      backgroundImage: `url(${viewTrack?.picture_url()})`,
    }"
  >
    <div class="flex items-end bg-gray-800/60 backdrop-blur-2xl w-full h-full">
      <img :src="viewTrack?.picture_url()" :class="`ml-5 mb-5 w-72 rounded-lg ${viewTrack?.picture_url() ? '' : 'invisible'} ring-1 ring-white/10`">
      <div class="grow">
        lyric
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>
