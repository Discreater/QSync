<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { ApiClient } from '~/api/client';
import H1 from '~/components/typo/H1.vue';
import H2 from '~/components/typo/H2.vue';
import Basic from '~/layouts/Basic.vue';
import type { Track } from '~/generated/protos/musync';

const props = defineProps<{
  query: string
}>();
const loading = ref(true);

const tracks = ref<Track[]>([]);

const route = useRoute();
watch(() => route.query, async () => {
  loading.value = true;
  const resp = await ApiClient.grpc().SearchAll({ query: route.query.q as string });
  tracks.value = resp.tracks;
  loading.value = false;
}, {
  immediate: true,
});

const { t } = useI18n();
</script>

<template>
  <Basic>
    <H1>{{ t('search-result.title') }} : "{{ props.query }}"</H1>
    <div v-if="loading">
      Loading
    </div>
    <div v-else>
      <H2>{{ t("search-result.track") }}</H2>
      <div v-for="track in tracks" :key="track.id">
        {{ track.title }} | {{ track.artist }} | {{ track.album }}
      </div>
    </div>
  </Basic>
</template>
