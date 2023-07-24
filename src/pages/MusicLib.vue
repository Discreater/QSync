<script setup lang="ts">
import Scrollbar from 'smooth-scrollbar';

import { useI18n } from 'vue-i18n';
import { onMounted, onUnmounted, ref } from 'vue';
import H1 from '~/components/typo/H1.vue';
import Basic from '~/layouts/Basic.vue';
import { useQSyncStore } from '~/store';

const { t } = useI18n();
const store = useQSyncStore();
const container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  if (container.value) {
    Scrollbar.init(container.value, {
      alwaysShowTracks: true,
    });
  }
});
onUnmounted(() => {
  if (container.value)
    Scrollbar.destroy(container.value!);
});
</script>

<template>
  <Basic>
    <H1>
      {{ t('music-lib.music') }}
    </H1>
    <div ref="container" class="flex-1 overflow-auto grow relative container">
      <div v-for="track in store.musicFolders[0].tracks" :key="track.path">
        {{ track.name() }}
      </div>
    </div>
  </Basic>
</template>

<style scoped>
p {
  animation-duration: 3s;
  animation-name: slidein;
}

@keyframes slidein {
  from {
    margin-left: 100%;
    width: 300%;
  }

  to {
    margin-left: 0%;
    width: 100%;
  }
}
</style>
