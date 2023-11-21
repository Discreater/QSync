<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import QPivot from '~qui/pivot/QPivot.vue';
import QPivotItem from '~qui/pivot/QPivotItem.vue';
import { ApiClient } from '~/api/client';
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
  <Basic :header="`${t('search-result.title')} : ${query}`" :no-scroll="true">
    <div v-if="loading">
      {{ t('loading') }}
    </div>
    <div v-else class="h-full flex-1 flex">
      <QPivot value="local" class="bg-main_bg">
        <QPivotItem value="local" :name="t('search-result.local-track')">
          <div v-for="track in tracks" :key="track.id">
            {{ track.title }} | {{ track.artist }} | {{ track.album }}
          </div>
        </QPivotItem>
        <QPivotItem value="netease" :name="t('search-result.netease-result')">
          <div v-if="ncmRes?.result">
            <ol>
              <li v-for="track in ncmRes.result.songs" :key="track.id">
                {{ track.name }} | {{ track.ar.map(ar => ar.name).join(', ') }} | {{ track.al.name }} | {{ track.pop }}
              </li>
            </ol>
          </div>
        </QPivotItem>
      </QPivot>
    </div>
  </Basic>
</template>
