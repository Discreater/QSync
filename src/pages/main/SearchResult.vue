<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { useStorage } from '@vueuse/core';
import QPivot from '~qui/pivot/QPivot.vue';
import QPivotItem from '~qui/pivot/QPivotItem.vue';
import { ApiClient } from '~/api/client';
import Basic from '~/layouts/Basic.vue';
import type { Track } from '~/generated/protos/musync';
import type { NcmSearchResult } from '~/model_ext/ncm';
import QTable from '~qui/table/QTable.vue';
import type { Column } from '~qui/table/types';

import { formatTime } from '~/utils';

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

const pivotValue = useStorage('search-result-pivot', 'local');

const localCols: Column[] = [
  { key: 'title', title: t('track.title') },
  { key: 'artist', title: t('track.artist') },
  { key: 'album', title: t('track.album') },
];
const ncmCols: Column[] = [
  { key: 'title', title: t('track.title') },
  { key: 'artist', title: t('track.artist') },
  { key: 'album', title: t('track.album') },
  { key: 'duration', title: t('track.duration'), style: {
    gridTemplateColumn: '48px',
  } },
  { key: 'pop', title: t('track.pop'), style: {
    textAlign: 'right',
    gridTemplateColumn: '24px',
  } },
];
</script>

<template>
  <Basic :header="`${t('search-result.title')} : ${query}`" :no-scroll="true">
    <div v-if="loading">
      {{ t('loading') }}
    </div>
    <div v-else class="h-full flex-1 flex">
      <QPivot v-model:value="pivotValue" class="bg-main_bg">
        <QPivotItem value="local" :name="t('search-result.local-track')">
          <QTable :columns="localCols" :data="tracks" :show-head="true" :row-key="(row) => row.id">
            <template #title="{ row }">
              {{ row.title }}
            </template>
            <template #artist="{ row }">
              {{ row.artist }}
            </template>
            <template #album="{ row }">
              {{ row.album }}
            </template>
          </QTable>
        </QPivotItem>
        <QPivotItem value="netease" :name="t('search-result.netease-result')">
          <QTable v-if="ncmRes?.result" :columns="ncmCols" :show-head="true" :data="ncmRes.result.songs" :row-key="(row) => row.id">
            <template #title="{ row }">
              {{ row.name }}
            </template>
            <template #artist="{ row }">
              {{ row.ar.map(ar => ar.name).join(', ') }}
            </template>
            <template #album="{ row }">
              {{ row.al.name }}
            </template>
            <template #duration="{ row }">
              {{ formatTime(row.dt / 1000) }}
            </template>
            <template #pop="{ row }">
              {{ row.pop }}
            </template>
          </QTable>
        </QPivotItem>
      </QPivot>
    </div>
  </Basic>
</template>
