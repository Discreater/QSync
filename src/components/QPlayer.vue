<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import QHoverButton from './QHoverButton.vue';
import QSlider from './QSlider.vue';
import QPopover from './QPopover.vue';
import HoverLayer from './HoverLayer.vue';
import H2 from './typo/H2.vue';
import { sameTrack, useConfigStore, useQSyncStore } from '~/store';
import { usePlayerStore } from '~/store/player';
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

const audio = ref<HTMLAudioElement | null>(null);
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
  audio.value!.src = uri;
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

const cannotPlay = ref(false);

async function updatePlayer(pState: typeof playerStore) {
  if (audio.value === null) {
    logger.info('audio is null');
    return;
  }

  const playQueue = qsyncStore.playQueue;
  if (playQueue.length === 0)
    return;

  const loadingProgress = pState.progress();
  // ignore the same track
  if (!currentTrack.value || !sameTrack(playQueue[pState.position], currentTrack.value)) {
    loadTrack(playQueue[pState.position]);
    audio.value.oncanplay = () => {
      updateState(true);
    };
  } else {
    updateState(false);
  }
  async function updateState(delayUpdate: boolean) {
    if (pState.playing && audio.value!.paused) {
      if (!cannotPlay.value) {
        try {
          await audio.value!.play();
        } catch (e) {
          if (e instanceof DOMException) {
            logger.info('mute play');
            toggleMute(true);
            try {
              await audio.value!.play();
            } catch (e) {
              logger.info('cannot play');
              cannotPlay.value = true;
              ifCannotPlay();
              console.error(e);
            }
          } else {
            console.error(e);
          }
        } finally {
          if (delayUpdate) {
            logger.trace('delay update');
            pState.updateProgress(loadingProgress, false);
          }
        }
      }
    } else if (!pState.playing && !audio.value!.paused) {
      audio.value!.pause();
    }

    // logger.trace('updating');
    const progress = pState.progress();
    if (Math.abs(progress - audio.value!.currentTime * 1000) > 1000) {
      const progresInSec = progress / 1000;
      // logger.debug(`update time, from ${audio.currentTime} to ${progresInSec}`);
      localProgress.value = progresInSec;
      audio.value!.currentTime = localProgress.value;
    }
  }
}

watch(playerStore, async (pState) => {
  await updatePlayer(pState);
}, { deep: true });

function onUserInteract() {
  logger.debug('user interact');
  cannotPlay.value = false;
  updatePlayer(playerStore);
  window.removeEventListener('mousedown', onUserInteract);
}
function ifCannotPlay() {
  window.addEventListener('mousedown', onUserInteract);
}

onMounted(async () => {
  audio.value!.onended = () => {
    audio.value!.pause();
    qsyncStore.nextTrack(false);
  };
  audio.value!.ontimeupdate = () => {
    localProgress.value = audio.value!.currentTime;
  };
  audio.value!.onloadedmetadata = () => {
    if (audio.value!.duration === Number.POSITIVE_INFINITY)
      return;

    duration.value = audio.value!.duration;
  };
  audio.value!.volume = configStore.volume / 100;

  await updatePlayer(playerStore);
});
onUnmounted(() => {
  audio.value?.pause();
});

localProgress.value = playerStore.progress();

function handlePrevious() {
  qsyncStore.previousTrack();
}
function handleNext() {
  qsyncStore.nextTrack(true);
}
function togglePlay() {
  playerStore.togglePlay();
}

function onSliderUpdate(v: number) {
  localProgress.value = v;
  playerStore.updateProgress(v * 1000, true);
}

watch(configStore, (config) => {
  if (audio.value === null)
    return;
  audio.value.volume = config.volume / 100;
});
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
  audio.value!.muted = muteState;
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
      <div class="flex-1 h-full flex overflow-hidden">
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
        <QHoverButton :icon="IconArrowShuffle" @click="qsyncStore.shufflePlayQueue()" />
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
    <audio ref="audio" class="hidden" />
  </div>
</template>

<style scoped></style>
