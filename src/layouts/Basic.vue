<script setup lang="ts">
import { type VNodeRef, ref } from 'vue';
import { getPlatform } from '~/platforms';
import H1 from '~/components/typo/H1.vue';
import QScrollbar from '~/components/QScrollbar.vue';

defineProps<{ header: string; showModel?: boolean }>();

const scrollbar = ref<VNodeRef | null>(null);
const pt = getPlatform() !== 'web' ? 'pt-12' : 'pt-4';
defineExpose({
  scrollbar,
});
</script>

<template>
  <main
    class="relative h-full w-full grow pb-1 overflow-auto flex flex-col px-1"
    :class="pt"
  >
    <div class="flex justify-between px-12">
      <H1>{{ header }}</H1>
      <slot name="header-extra" />
    </div>
    <div class="px-12">
      <slot name="actions" />
    </div>
    <QScrollbar ref="scrollbar" class="flex-1 grow relative px-12">
      <slot />
    </QScrollbar>
    <div v-if="showModel" class="absolute bottom-0 left-0 right-0 top-title bg-[#4b4b4b80] flex justify-center items-center" :class="pt">
      <slot name="model" />
    </div>
  </main>
</template>
