<script setup lang="ts">
import { computed, ref } from 'vue';
import { onBeforeRouteUpdate } from 'vue-router';
import { ApiClient } from '~/api/client';
import QImage from '~/components/QImage.vue';
import type { TrackId } from '~/model_ext/track';
import IconMusic from '~icons/fluent/music-note-2-24-regular';

const props = defineProps<{ id?: string | number }>();
const trackId = ref<TrackId>();

function numberId(id?: string | number) {
  if (typeof id === 'string')
    return Number.parseInt(id);

  return id;
}

onBeforeRouteUpdate((to, from, next) => {
  if (to.query.id !== from.query.id)
    trackId.value = numberId(to.query.id as string);
  next();
});

if (props.id)
  trackId.value = numberId(props.id);

const picture_url = computed(() => {
  return ApiClient.get().cover_uri(trackId.value);
});
</script>

<template>
  <main
    class="track-pic-bg h-full w-full bg-cover" :style="{
      backgroundImage: `url(${picture_url})`,
    }"
  >
    <div class="flex bg-white/40 dark:bg-gray-800/60 backdrop-blur-2xl w-full h-full">
      <QImage :src="picture_url" class="ml-5 mb-5 w-64 h-64 rounded ring-1 ring-gray-500/10 dark:ring-black/10 shadow-xl self-end" :class="picture_url ? '' : 'invisible'">
        <template #failed>
          <div class="flex items-center justify-center h-full">
            <IconMusic class="text-6xl" />
          </div>
        </template>
      </QImage>
      <div class="grow h-full flex items-center justify-center">
        lyric
      </div>
    </div>
  </main>
</template>

<style scoped></style>
