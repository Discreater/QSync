<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { ApiClient } from '~/api/client';
import H2 from '~/components/typo/H2.vue';
import Basic from '~/layouts/Basic.vue';
import type { Track } from '~/generated/protos/musync';
import type { NcmSearchResult } from '~/model_ext/ncm';

defineProps<{
  query: string
}>();
const loading = ref(true);

const tracks = ref<Track[]>([]);
const ncmRes = ref<NcmSearchResult>();

const route = useRoute();
watch(() => route.query, async () => {
  loading.value = true;
  const resp = await ApiClient.grpc().SearchAll({ query: route.query.q as string });
  tracks.value = resp.dbTracks;
  ncmRes.value = JSON.parse(resp.ncmRes);
  loading.value = false;
}, {
  immediate: true,
});

const { t } = useI18n();
</script>

<template>
  <Basic :header="`${t('search-result.title')} : ${query}`">
    <div v-if="loading">
      Loading
    </div>
    <div v-else>
      <H2>{{ t("search-result.local-track") }}</H2>
      <div v-for="track in tracks" :key="track.id">
        {{ track.title }} | {{ track.artist }} | {{ track.album }}
      </div>
      <H2>{{ t("search-result.netease-result") }}</H2>
      <div v-if="ncmRes?.result">
        <ol>
          <li v-for="track in ncmRes.result.songs" :key="track.id">
            {{ track.name }} | {{ track.ar.map(ar => ar.name).join(', ') }} | {{ track.al.name }} | {{ track.pop }}
          </li>
        </ol>
      </div>
    </div>
  </Basic>
</template>
