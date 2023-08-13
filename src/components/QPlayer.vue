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
import type { Track } from '~/store';
import { getShowTrack, sameTrack, useConfigStore, usePlayerStore, useQSyncStore } from '~/store';
import { logger } from '~/utils/logger';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import IconPrevious from '~icons/fluent/previous-24-filled';
import IconNext from '~icons/fluent/next-24-filled';
import IconPlay from '~icons/fluent/play-24-filled';
import IconPause from '~icons/fluent/pause-20-filled';
import IconRepeat from '~icons/fluent/arrow-repeat-all-24-regular';
import IconVolume from '~icons/fluent/speaker-2-24-regular';
import IconMore from '~icons/fluent/more-horizontal-24-regular';
import { formatTime } from '~/utils';

const router = useRouter();
const route = useRoute();

const audio = new Audio();
const qsyncStore = useQSyncStore();
const playerStore = usePlayerStore();
const configStore = useConfigStore();

const currentTrack = ref<Track>();

// in seconds (float point)
const localProgress = ref(0);
const duration = ref(0);

function loadTrack(track: Track) {
  logger.trace(`load new track: ${track.path}`);
  currentTrack.value = track;
  audio.src = convertFileSrc(track.path);
  if (route.name === 'lyric') {
    router.replace({
      name: 'lyric',
      query: {
        path: track.path,
      },
    });
  }
}

async function updatePlayer(pState: typeof playerStore) {
  const playback = qsyncStore.playbackQueue;
  if (playback.length === 0)
    return;

  // ignore the same track
  if (!currentTrack.value || !sameTrack(playback[pState.current], currentTrack.value))
    loadTrack(playback[pState.current]);

  if (Math.abs(pState.progress - audio.currentTime * 1000) > 1000) {
    logger.debug('update time');
    localProgress.value = pState.progress / 1000;
    audio.currentTime = localProgress.value;
  }

  if (pState.playing && audio.paused)
    await audio.play();
  else if (!pState.playing && !audio.paused)
    audio.pause();
}

watch(playerStore, async (pState) => {
  await updatePlayer(pState);
}, { deep: true });

onMounted(async () => {
  await updatePlayer(playerStore);
});
onUnmounted(() => {
  audio.pause();
});

audio.onended = () => {
  audio.pause();
  qsyncStore.nextTrack();
};
localProgress.value = playerStore.progress;
audio.ontimeupdate = () => {
  localProgress.value = audio.currentTime;
};

audio.onloadedmetadata = () => {
  duration.value = audio.duration;
};

const viewTrack = ref<ViewTrack>();
watch(currentTrack, (track) => {
  if (track) {
    getShowTrack(track.path).then((t) => {
      viewTrack.value = t;
    });
  }
});

function handlePrevious() {
  qsyncStore.previousTrack();
}
function handleNext() {
  qsyncStore.nextTrack();
}
function togglePlay() {
  playerStore.togglePlay();
}

function onSliderUpdate(v: number) {
  localProgress.value = v;
  playerStore.updateProgress(v * 1000);
}
watch(configStore, (config) => {
  audio.volume = config.volume / 100;
});
audio.volume = configStore.volume / 100;
function onVolumeUpdate(v: number) {
  v = Math.max(0, v);
  v = Math.min(100, v);
  configStore.$patch({
    volume: v,
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

function artistAlbum(view: ViewTrack | undefined) {
  if (view)
    return `${view.artist()} • ${view.album()}`;
}
</script>

<template>
  <div class="h-[118px] flex flex-col border-solid border-t border-black/30 gap-1 p-1">
    <QSlider class="px-3" :value="localProgress" :min="0" :max="duration" @update:value="onSliderUpdate">
      <template #left="{ value }">
        <span class="text-xs w-14"> {{ formatTime(value, 'hh:mm:ss') }} </span>
      </template>
      <template #right="{ value }">
        <span class="text-xs text-right w-14">{{ duration && value != null ? `${formatTime(duration - value, 'hh:mm:ss')}` : ''
        }}</span>
      </template>
    </QSlider>
    <div class="grow flex justify-between items-center p-1">
      <div class="flex-1 h-full flex">
        <HoverLayer class="flex select-none cursor-default min-w-0" @click="onInfoCardClick()">
          <img
            v-if="showCardImg" :src="viewTrack?.picture_url()"
            :class="`object-scale-down w-20 border-white/10 border ${viewTrack?.picture_url() ? '' : 'invisible'}`"
          >
          <div :class="`flex flex-col min-w-0 transition-all duration-300 ${showCardImg ? 'ml-5' : 'ml-2 mr-3'}`">
            <H2 class="truncate" :title="viewTrack?.name()">
              {{ viewTrack?.name() }}
            </H2>
            <p class="truncate font-thin" :title="artistAlbum(viewTrack)">
              {{ artistAlbum(viewTrack) }}
            </p>
          </div>
        </HoverLayer>
      </div>
      <div class="flex-1 flex justify-center items-center gap-3">
        <QHoverButton :icon="IconArrowShuffle" @click="qsyncStore.shufflePLayback()" />
        <QHoverButton :icon="IconPrevious" @click="handlePrevious" />
        <button
          class="rounded-full w-14 h-14 text-2xl flex justify-center items-center bg-gradient-to-br from-orange-500 to-purple-500"
          @click="togglePlay"
        >
          <IconPause v-if="playerStore.playing" />
          <IconPlay v-else />
        </button>
        <QHoverButton :icon="IconNext" @click="handleNext" />
        <QHoverButton :icon="IconRepeat" :disabled="true" />
      </div>
      <div class="flex-1 flex justify-end items-center gap-2">
        <QPopover>
          <QHoverButton :icon="IconVolume" />
          <template #popover>
            <QSlider :min="0" :max="100" :value="configStore.volume" @input="onVolumeUpdate">
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
      </div>
    </div>
  </div>
</template>

<style scoped></style>
