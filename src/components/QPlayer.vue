<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import QHoverButton from './QHoverButton.vue';
import QSlider from './QSlider.vue';
import QPopover from './QPopover.vue';
import { ViewTrack, readTrack } from '~/sources/folder';
import type { Track } from '~/store';
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

const { t } = useI18n();

const audio = new Audio();
const store = useQSyncStore();

const currentTrack = ref<Track>();
const preloadedTrack = ref<{ track: Track; blob: Blob }>();

const localProgress = ref(0);
const duration = ref(0);

async function loadTrack(track: Track, preload: Track | undefined) {
  currentTrack.value = track;
  let blob;
  logger.debug(`next track: ${track.path}`);
  logger.debug(`preloaded track: ${preloadedTrack.value?.track.path}`);
  if (preloadedTrack.value && sameTrack(track, preloadedTrack.value.track)) {
    logger.trace('same track');
    blob = preloadedTrack.value.blob;
  } else {
    blob = await readTrack(track.path);
  }
  const url = URL.createObjectURL(blob);
  audio.src = url;
  if (preload) {
    logger.trace(`preloading next track ${preload.path}`);
    readTrack(preload.path).then((blob) => {
      preloadedTrack.value = { track: preload, blob };
    });
  }
}

watch((store.playbackQueue), async (playback) => {
  // ignore the same track
  if (!currentTrack.value || !sameTrack(playback.queue[playback.current], currentTrack.value)) {
    logger.debug('play new track');
    loadTrack(playback.queue[playback.current], playback.queue[playback.current + 1]);
  }

  if (Math.abs(playback.progress - audio.currentTime) > 1)
    audio.currentTime = playback.progress;

  if (playback.playing && audio.paused)
    audio.play();
  else if (!playback.playing && !audio.paused)
    audio.pause();
}, { deep: true });

audio.onended = () => {
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

const viewTrack = computed(() => currentTrack.value ? new ViewTrack(currentTrack.value) : null);

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
  function pad(num: number, length: number): string {
    const str = `${num}`;
    return str.padStart(length, '0');
  }
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
const volume = ref(audio.volume);
function onVolumeUpdate(v: number) {
  v = Math.max(0, v);
  v = Math.min(100, v);
  volume.value = v / 100;
  audio.volume = volume.value;
}
</script>

<template>
  <div class="h-[118px] flex flex-col border-solid border-t border-black/30 gap-1">
    <QSlider
      class="px-3"
      :left="{
        type: 'value',
        formatter: formatTime,
      }"
      :right="{
        type: 'value',
        formatter: (v) => duration ? `${formatTime(duration - v)}` : '',
      }"
      :value="store.playbackQueue.progress" :min="0" :max="duration" :formatter="formatTime" @update:value="onSliderUpdate"
    />
    <div class="flex justify-between items-center px-2">
      <div class="flex-1">
        {{ viewTrack?.name() }}
      </div>
      <div class="flex-1 flex justify-center items-center gap-3">
        <QHoverButton :icon="IconArrowShuffle" :disabled="true" />
        <QHoverButton :icon="IconPrevious" @click="handlePrevious" />
        <button class="rounded-full w-14 h-14 text-2xl flex justify-center items-center bg-gradient-to-br from-orange-500 to-purple-500" @click="togglePlay">
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
            <QSlider
              :left="{
                type: 'icon',
                icon: IconVolume,
              }" :right="{
                type: 'value',
                formatter: (v) => `${v.toFixed(0)}`,
              }" :min="0" :max="100" :value="volume * 100" @input="onVolumeUpdate"
            />
          </template>
        </QPopover>
        <QHoverButton :icon="IconMore" :disabled="true" />
        <!-- todo controller -->
      </div>
    </div>
  </div>
</template>

<style scoped>
input[type="range"] {
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  width: 100%;
  cursor: pointer;
  outline: none;
  height: 0.25rem;
  background: #ffffff80;
  border-radius: 0.5rem;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb {
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  height: 1.5rem;
  width: 1.5rem;
  background-color: rgb(249 115 22 / 1);
  border-radius: 50%;
  border: 0.4rem solid #454545;
  /*  slider progress trick  */
  transition: .2s ease-in-out;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb {
  height: 1.5rem;
  width: 1.5rem;
  background-color: #f97316;
  border-radius: 50%;
  border: 0.4rem solid #454545;

  /* box-shadow: -407px 0 0 400px #f50; emove this line */
  transition: .2s ease-in-out;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb:hover {
  border: 0.3rem solid #454545;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb:hover {
  border: 0.3rem solid #454545;
}
</style>
