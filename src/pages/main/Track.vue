<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { ApiClient } from '~/api/client';
import QInput from '~/components/QInput.vue';
import H1 from '~/components/typo/H1.vue';
import type { LocalSource, NeteaseSource, Track } from '~/generated/protos/musync';
import Basic from '~/layouts/Basic.vue';

const props = defineProps<{ id?: string | number }>();

function numberId(id?: string | number) {
  if (typeof id === 'string')
    return Number.parseInt(id);

  return id;
}

interface EditingTrack {
  title: string
  artist: string
  album: string
  duration?: number
  genre: string
  year: string
  createdAt?: Date
  updatedAt?: Date
  localSrc: LocalSource
  neteaseSrc: NeteaseSource
}

function into_editing(t: Track): EditingTrack {
  return {
    title: t.title,
    artist: t.artist ?? '',
    album: t.album ?? '',
    duration: t.duration,
    genre: t.genre ?? '',
    year: t.year?.toString() ?? '',
    createdAt: t.createdAt,
    updatedAt: t.updatedAt,
    localSrc: t.localSrc ?? { path: '' },
    neteaseSrc: t.neteaseSrc ?? { id: '' },
  };
}

const track = ref<Track>();
const editingTrack = ref<EditingTrack>();
const route = useRoute();
const loading = ref(true);
watch(() => route.query, async () => {
  loading.value = true;
  const resp = await ApiClient.grpc().GetTrack({ id: numberId(props.id) });
  track.value = resp;
  editingTrack.value = into_editing(resp);
  loading.value = false;
}, {
  immediate: true,
});
const { t } = useI18n();
</script>

<template>
  <Basic>
    <H1>
      {{ t('track.track-info') }}
    </H1>
    <div v-if="!loading && editingTrack" class="grid grid-cols-[100px_minmax(0,1fr)] items-center space-y-2">
      <label for="track-info-title">{{ t('track.title') }}</label>
      <QInput id="track-info-title" v-model:value="editingTrack.title" type="text" />
      <label for="track-info-artist">{{ t('track.artist') }}</label>
      <QInput id="track=info-artist" v-model:value="editingTrack.artist" type="text" />
      <label for="track-info-album">{{ t('track.album') }}</label>
      <QInput id="track-info-album" v-model:value="editingTrack.album" type="text" />
      <label for="track-info-genre">{{ t('track.genre') }}</label>
      <QInput id="track-info-genre" v-model:value="editingTrack.genre" type="text" />
      <label for="track-info-year">{{ t('track.year') }}</label>
      <QInput id="track-info-year" v-model:value="editingTrack.year" type="text" />
      <lable for="track-local-src">
        {{ t("track.local-src") }}
      </lable>
      <p id="track-local-src">
        {{ editingTrack.localSrc.path }}
      </p>
      <lable for="track-netease-src">
        {{ t("track.netease-src") }}
      </lable>
      <p id="track-netease-src">
        {{ editingTrack.neteaseSrc.id }}
      </p>
    </div>
    <div v-else>
      loading
    </div>
  </Basic>
</template>
