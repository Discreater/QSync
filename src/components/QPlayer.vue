<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { useRoute, useRouter } from 'vue-router';
import QHoverButton from './QHoverButton.vue';
import QSlider from './QSlider.vue';
import QPopover from './QPopover.vue';
import HoverLayer from './HoverLayer.vue';
import H2 from './typo/H2.vue';
import type { ViewTrack } from '~/sources/folder';
import { getTrackInfo } from '~/sources/folder';
import type { PlaybackQueue, Track } from '~/store';
import { sameTrack, useQSyncStore } from '~/store';
import { logger } from '~/utils/logger';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import IconPrevious from '~icons/fluent/previous-24-filled';
import IconNext from '~icons/fluent/next-24-filled';
import IconPlay from '~icons/fluent/play-24-filled';
import IconPause from '~icons/fluent/pause-20-filled';
import IconRepeat from '~icons/fluent/arrow-repeat-all-24-regular';
import IconVolume from '~icons/fluent/speaker-2-24-regular';
import IconMore from '~icons/fluent/more-horizontal-24-regular';
import { pad } from '~/utils';

const router = useRouter();
const route = useRoute();

const audio = new Audio();
const store = useQSyncStore();

const currentTrack = ref<Track>();

const localProgress = ref(0);
const duration = ref(0);

function loadTrack(track: Track) {
  logger.trace(`load new track: ${track.path}`);
  currentTrack.value = track;
  audio.src = convertFileSrc(track.path);
  getTrackInfo(track.path, true).then((info) => {
    track.pictures = info.pictures;
  });
  if (route.name === 'lyric') {
    router.replace({
      name: 'lyric',
      query: {
        path: track.path,
      },
    });
  }
}

async function updatePlayer(playback: PlaybackQueue) {
  // ignore the same track
  if (!currentTrack.value || !sameTrack(playback.queue[playback.current], currentTrack.value))
    loadTrack(playback.queue[playback.current]);

  if (Math.abs(playback.progress - audio.currentTime) > 1)
    audio.currentTime = playback.progress;

  if (playback.playing && audio.paused)
    await audio.play();
  else if (!playback.playing && !audio.paused)
    audio.pause();
}

watch((store.playbackQueue), async (playback) => {
  await updatePlayer(playback);
}, { deep: true });

onMounted(async () => {
  await updatePlayer(store.playbackQueue);
});
onUnmounted(() => {
  audio.pause();
});

audio.onended = () => {
  audio.pause();
  store.nextTrack();
};
audio.ontimeupdate = () => {
  localProgress.value = audio.currentTime;
  store.$patch((state) => {
    state.playbackQueue.progress = audio.currentTime;
  });
};
audio.onloadedmetadata = () => {
  duration.value = audio.duration;
};

const viewTrack = ref<ViewTrack>();
watch(currentTrack, (track) => {
  if (track) {
    store.getShowTrack(track.path).then((t) => {
      viewTrack.value = t;
    });
  }
});

function handlePrevious() {
  store.previousTrack();
}
function handleNext() {
  store.nextTrack();
}
function togglePlay() {
  store.togglePlay();
}

function formatTime(time: number) {
  const hour = Math.floor(time / 3600);
  const min = pad(Math.floor(time / 60), 2);
  const sec = pad(Math.floor(time % 60), 2);
  return `${hour}:${min}:${sec}`;
}

function onSliderUpdate(v: number) {
  store.$patch((state) => {
    state.playbackQueue.progress = v;
  });
}
watch(store.config, (config) => {
  audio.volume = config.volume / 100;
});
audio.volume = store.config.volume / 100;
function onVolumeUpdate(v: number) {
  v = Math.max(0, v);
  v = Math.min(100, v);
  store.$patch((state) => {
    state.config.volume = v;
  });
}
function onInfoCardClick() {
  if (route.name === 'lyric') {
    if (window.history.length > 1)
      router.back();
    else
      router.push({ name: 'main' });
  } else {
    router.push({ name: 'lyric', query: { path: currentTrack.value?.path } });
  }
}
const showCardImg = computed(() => route.name !== 'lyric');
</script>

<template>
  <div class="h-[118px] flex flex-col border-solid border-t border-black/30 gap-1 p-1">
    <QSlider class="px-3" :value="store.playbackQueue.progress" :min="0" :max="duration" @update:value="onSliderUpdate">
      <template #left="{ value }">
        <span class="text-xs w-14"> {{ formatTime(value) }} </span>
      </template>
      <template #right="{ value }">
        <span class="text-xs text-right w-14">{{ duration && value ? `${formatTime(duration - value)}` : '' }}</span>
      </template>
    </QSlider>
    <div class="grow flex justify-between items-center p-1">
      <HoverLayer class="flex-1 h-full flex gap-5 select-none cursor-default min-w-0" @click="onInfoCardClick()">
        <img
          v-if="showCardImg" :src="viewTrack?.picture_url()"
          :class="`object-scale-down w-20 border-white/10 border ${viewTrack?.picture_url() ? '' : 'invisible'}`"
        >
        <div class="flex flex-col min-w-0">
          <H2 class="truncate" :title="viewTrack?.name">
            {{ viewTrack?.name() }}
          </H2>
          <p class="truncate" :title="viewTrack?.artist()">
            {{ viewTrack?.artist() }}
          </p>
        </div>
      </HoverLayer>
      <div class="flex-1 flex justify-center items-center gap-3">
        <QHoverButton :icon="IconArrowShuffle" @click="store.shufflePLayback()" />
        <QHoverButton :icon="IconPrevious" @click="handlePrevious" />
        <button
          class="rounded-full w-14 h-14 text-2xl flex justify-center items-center bg-gradient-to-br from-orange-500 to-purple-500"
          @click="togglePlay"
        >
          <IconPause v-if="store.playbackQueue.playing" />
          <IconPlay v-else />
        </button>
        <QHoverButton :icon="IconNext" @click="handleNext" />
        <QHoverButton :icon="IconRepeat" :disabled="true" />
      </div>
      <div class="flex-1 flex justify-end items-center gap-2">
        <QPopover>
          <QHoverButton :icon="IconVolume" />
          <template #popover>
            <QSlider :min="0" :max="100" :value="store.config.volume" @input="onVolumeUpdate">
              <template #left>
                <QHoverButton :icon="IconVolume" :disabled="true" />
              </template>
              <template #right="{ value }">
                <span class="text-xs text-right w-8">{{ value.toFixed(0) }}</span>
              </template>
            </QSlider>
          </template>
        </QPopover>
        <QHoverButton :icon="IconMore" :disabled="true" />
        <!-- todo controller -->
      </div>
    </div>
  </div>
</template>

<style scoped></style>
