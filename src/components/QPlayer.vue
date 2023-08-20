<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import QHoverButton from './QHoverButton.vue';
import QSlider from './QSlider.vue';
import QPopover from './QPopover.vue';
import HoverLayer from './HoverLayer.vue';
import H2 from './typo/H2.vue';
import { sameTrack, useConfigStore, usePlayerStore, useQSyncStore } from '~/store';
import { logger } from '~/utils/logger';
import IconArrowShuffle from '~icons/fluent/arrow-shuffle-24-regular';
import IconPrevious from '~icons/fluent/previous-24-filled';
import IconNext from '~icons/fluent/next-24-filled';
import IconPlay from '~icons/fluent/play-24-filled';
import IconPause from '~icons/fluent/pause-20-filled';
import IconRepeat from '~icons/fluent/arrow-repeat-all-24-regular';
import IconVolume from '~icons/fluent/speaker-2-24-regular';
import IconVolumeMute from '~icons/fluent/speaker-mute-24-regular';
import IconMore from '~icons/fluent/more-horizontal-24-regular';
import { formatTime } from '~/utils';
import { ApiClient } from '~/api/client';
import type { Track } from '~/generated/protos/musync';
import { TrackExt } from '~/model_ext/track';

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
  logger.trace(`load new track: ${track.title}`);
  const uri = ApiClient.get().track_uri(track.id);
  currentTrack.value = track;
  audio.src = uri;
  if (track.duration)
    duration.value = TrackExt.durationInSecs(track.duration);

  if (route.name === 'lyric') {
    router.replace({
      name: 'lyric',
      query: {
        id: track.id,
      },
    });
  }
}

async function updatePlayer(pState: typeof playerStore) {
  const playQueue = qsyncStore.playQueue;
  if (playQueue.length === 0)
    return;

  // ignore the same track
  if (!currentTrack.value || !sameTrack(playQueue[pState.current], currentTrack.value))
    loadTrack(playQueue[pState.current]);

  if (Math.abs(pState.progress - audio.currentTime * 1000) > 1000) {
    logger.debug(`update time, from ${audio.currentTime * 1000} to ${pState.progress / 1000}`);
    localProgress.value = pState.progress / 1000;
    audio.currentTime = localProgress.value;
  }

  if (pState.playing && audio.paused) {
    try {
      await audio.play();
    } catch (e) {
      if (e instanceof DOMException) {
        logger.info('mute play');
        toggleMute(true);
        await audio.play();
      } else {
        throw e;
      }
    }
  } else if (!pState.playing && !audio.paused) {
    audio.pause();
  }
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
  if (audio.duration === Number.POSITIVE_INFINITY)
    return;

  duration.value = audio.duration;
};

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
    muted: false,
  });
}
function onInfoCardClick() {
  if (route.name === 'lyric') {
    if (window.history.length > 1)
      router.back();
    else
      router.push({ name: 'main' });
  } else {
    router.push({ name: 'lyric', query: { id: currentTrack.value?.id } });
  }
}
const showCardImg = computed(() => route.name !== 'lyric');

function artistAlbum(view: Track | undefined) {
  if (view)
    return `${view.artist} • ${view.album}`;
}

function toggleMute(mute?: boolean) {
  const muteState = mute === undefined ? !configStore.muted : mute;

  logger.trace(`toggle mute: ${muteState}`);
  configStore.$patch((state) => {
    state.muted = muteState;
  });
  audio.muted = muteState;
}
</script>

<template>
  <div class="h-[118px] flex flex-col border-solid border-t border-black/30 gap-1 p-1">
    <QSlider class="px-3" :value="localProgress" :min="0" :max="duration" @update:value="onSliderUpdate">
      <template #left="{ value }">
        <span class="text-xs w-14"> {{ formatTime(value, 'hh:mm:ss') }} </span>
      </template>
      <template #right="{ value }">
        <span class="text-xs text-right w-14">{{ duration && value != null ? `${formatTime(duration - value, 'hh:mm:ss')}`
          : ''
        }}</span>
      </template>
    </QSlider>
    <div class="grow flex justify-between items-center p-1">
      <div class="flex-1 h-full flex">
        <HoverLayer class="flex select-none cursor-default min-w-0" @click="onInfoCardClick()">
          <img
            v-if="showCardImg" :src="currentTrack ? ApiClient.get().cover_uri(currentTrack.id) : ''"
            class="object-scale-down w-20 border-white/10 bord er"
          >
          <div :class="`flex flex-col min-w-0 transition-all duration-300 ${showCardImg ? 'ml-5' : 'ml-2 mr-3'}`">
            <H2 class="truncate" :title="currentTrack?.title">
              {{ currentTrack?.title }}
            </H2>
            <p class="truncate font-thin" :title="artistAlbum(currentTrack)">
              {{ artistAlbum(currentTrack) }}
            </p>
          </div>
        </HoverLayer>
      </div>
      <div class="flex-1 flex justify-center items-center gap-3">
        <QHoverButton :icon="IconArrowShuffle" @click="qsyncStore.shufflePLayQueue()" />
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
          <QHoverButton v-if="configStore.muted" :icon="IconVolumeMute" />
          <QHoverButton v-else :icon="IconVolume" />
          <template #popover>
            <QSlider :min="0" :max="100" :value="configStore.muted ? 0 : configStore.volume" @input="onVolumeUpdate">
              <template #left>
                <QHoverButton v-if="configStore.muted" :icon="IconVolumeMute" @click="toggleMute()" />
                <QHoverButton v-else :icon="IconVolume" @click="toggleMute()" />
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
