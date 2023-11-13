<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import QHoverButton from './QHoverButton.vue';
import QSlider from './QSlider.vue';
import QPopover from './QPopover.vue';
import HoverLayer from './HoverLayer.vue';
import H2 from './typo/H2.vue';
import QImage from './QImage.vue';
import { sameTrack, useMusyncStore } from '~/store';
import { usePlayerConfigStore, usePlayerStore } from '~/store/player';
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
import IconMusic from '~icons/fluent/music-note-2-24-regular';
import { formatTime } from '~/utils';
import { ApiClient } from '~/api/client';
import type { Track } from '~/generated/protos/musync';
import { TrackExt } from '~/model_ext/track';

const router = useRouter();
const route = useRoute();

const audio = ref<HTMLAudioElement | null>(null);
const qsyncStore = useMusyncStore();
const playerStore = usePlayerStore();
const configStore = usePlayerConfigStore();

const currentTrack = ref<Track>();
const unsupported = ref(false);

// in seconds (float point)
const localProgress = ref(0);
const duration = ref(0);

function loadTrack(track: Track) {
  logger.trace(`load new track: ${track.title}`);
  const uri = ApiClient.get().track_uri(track.id);
  currentTrack.value = track;
  audio.value!.src = uri;
  unsupported.value = false;
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
  if (playQueue.length <= pState.position) {
    logger.warn(`update player error: play queue length is ${playQueue.length}, position is ${pState.position}`);
    return;
  }

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
      if (!cannotPlay.value && !unsupported.value) {
        try {
          await audio.value!.play();
        } catch (e) {
          if (e instanceof DOMException) {
            // No supported media type
            if (e.message.match('no supported')) {
              unsupported.value = true;
              return;
            }

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

  setMediaSessionHandler();
  updateMediaSession(currentTrack.value);
});
onUnmounted(() => {
  audio.value?.pause();
});

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
  audio.value!.muted = false;
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

const { t } = useI18n();

function artistAlbum(view: Track | undefined) {
  if (view) {
    if (view.artist && view.album)
      return `${view.artist} • ${view.album}`;
    else if (view.artist)
      return view.artist;
    else if (view.album)
      return `${t('player.unknown-artist')} • ${view.album}`;
  }
}

function toggleMute(mute?: boolean) {
  const muteState = mute === undefined ? !configStore.muted : mute;

  logger.trace(`toggle mute: ${muteState}`);
  configStore.$patch((state) => {
    state.muted = muteState;
  });
  audio.value!.muted = muteState;
}

watch(currentTrack, (track) => {
  updateMediaSession(track);
});

function updateMediaSession(track: Track | undefined) {
  if (navigator.mediaSession != null) {
    if (track) {
      navigator.mediaSession.metadata = new MediaMetadata({
        title: track.title,
        artist: track.artist,
        album: track.album,
        artwork: [
          {
            src: ApiClient.get().cover_uri(track.id),
          },
        ],
      });
    } else {
      navigator.mediaSession.metadata = null;
    }
  }
}

function setMediaSessionHandler() {
  if (navigator.mediaSession != null) {
    navigator.mediaSession.setActionHandler('play', () => {
      playerStore.resumeTrack();
    });
    navigator.mediaSession.setActionHandler('pause', () => {
      playerStore.pauseTrack();
    });
    navigator.mediaSession.setActionHandler('seekforward', (details) => {
      if (details.seekOffset)
        logger.trace(`seek forward: ${details.seekOffset}`);
    });
    navigator.mediaSession.setActionHandler('seekbackward', (details) => {
      if (details.seekOffset)
        logger.trace(`seek backward: ${details.seekOffset}`);
    });
    navigator.mediaSession.setActionHandler('seekto', (details) => {
      if (details.seekTime)
        logger.trace(`seek to: ${details.seekTime}`);
    });
    navigator.mediaSession.setActionHandler('previoustrack', () => {
      qsyncStore.previousTrack();
    });
    navigator.mediaSession.setActionHandler('nexttrack', () => {
      qsyncStore.nextTrack(true);
    });
  }
}
</script>

<template>
  <div class="h-player flex flex-col border-solid border-t border-gray-500/30 dark:border-black/30 gap-1 p-0.5">
    <!-- Progress slider -->
    <QSlider class="px-3 mt-1" :value="localProgress" :min="0" :max="duration" @update:value="onSliderUpdate">
      <template #left="{ value }">
        <span class="text-xs w-14"> {{ formatTime(value, 'hh:mm:ss') }} </span>
      </template>
      <template #right="{ value }">
        <span class="text-xs text-right w-14">{{ duration && value != null ? `${formatTime(duration - value, 'hh:mm:ss')}`
          : ''
        }}</span>
      </template>
    </QSlider>
    <div class="grow flex justify-between items-center p-0.5">
      <div class="flex-1 h-full flex overflow-hidden">
        <!-- Track info (cover/title/artist...) -->
        <HoverLayer v-if="currentTrack" class="flex items-center select-none cursor-default min-w-[160px] h-20" @click="onInfoCardClick()">
          <QImage
            v-if="showCardImg" :src="currentTrack ? ApiClient.get().cover_uri(currentTrack.id) : ''"
            class="object-scale-down w-[70px] h-[70px] border-white/10 border"
          >
            <template #failed>
              <div class="flex items-center justify-center h-full ">
                <IconMusic class="text-3xl" />
              </div>
            </template>
          </QImage>
          <div class="flex flex-col min-w-0 transition-all duration-300" :class="showCardImg ? 'ml-5' : 'ml-2 mr-3'">
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
        <QHoverButton @click="qsyncStore.shufflePlayQueue()">
          <IconArrowShuffle class="text-lg" />
        </QHoverButton>
        <QHoverButton @click="handlePrevious">
          <IconPrevious class="text-lg" />
        </QHoverButton>
        <button
          class="rounded-full w-14 h-14 text-2xl flex justify-center items-center bg-gradient-to-br from-orange-500 to-purple-500"
          @click="togglePlay"
        >
          <IconPause v-if="playerStore.playing" />
          <IconPlay v-else />
        </button>
        <QHoverButton @click="handleNext">
          <IconNext class="text-lg" />
        </QHoverButton>
        <QHoverButton :disabled="true">
          <IconRepeat class="text-lg" />
        </QHoverButton>
      </div>
      <div class="flex-1 flex justify-end items-center gap-2">
        <QPopover>
          <QHoverButton>
            <IconVolumeMute v-if="configStore.muted" class="text-lg" />
            <IconVolume v-else class="text-lg" />
          </QHoverButton>
          <template #popover>
            <QSlider :min="0" :max="100" :value="configStore.volume" @input="onVolumeUpdate">
              <template #left>
                <QHoverButton @click="toggleMute()">
                  <IconVolumeMute v-if="configStore.muted" class="text-lg" />
                  <IconVolume v-else class="text-lg" />
                </QHoverButton>
              </template>
              <template #right="{ value }">
                <span class="text-xs text-right w-8">{{ value.toFixed(0) }}</span>
              </template>
            </QSlider>
          </template>
        </QPopover>
        <QHoverButton :disabled="true">
          <IconMore class="text-lg" />
        </QHoverButton>
      </div>
    </div>
    <audio ref="audio" class="hidden" />
  </div>
</template>

<style scoped></style>
