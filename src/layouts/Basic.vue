<script setup lang="ts">
import H1 from '~/components/typo/H1.vue';
import { QScrollbar } from '~qui';
import { bgLayer, useLayerLevel } from '~/components/injects';

const props = defineProps<{
  header: string
  showModel?: boolean
  layer?: number
  noScroll?: boolean
}>();

const level = useLayerLevel(props.layer);
</script>

<template>
  <main
    class="relative h-full w-full grow pb-1 flex flex-col px-1 rounded-none sm:rounded-tl-WINDOW ring-1 ring-black/10 pt-5"
    :class="bgLayer(level)"
  >
    <div class="flex justify-between px-14 mb-3">
      <H1>{{ header }}</H1>
      <slot name="header-extra" />
    </div>
    <div class="px-14">
      <slot name="actions" />
    </div>
    <div v-if="noScroll" class="flex flex-col flex-1 relative px-14 py-1 overflow-auto">
      <slot />
    </div>
    <QScrollbar v-else class="flex-1 relative px-14 py-1" content-class="space-y-1">
      <slot />
    </QScrollbar>
    <div v-if="showModel" class="absolute bottom-0 left-0 right-0 top-0 bg-black/20 dark:bg-[#4b4b4b80] flex justify-center items-center pt-5">
      <slot name="model" />
    </div>
  </main>
</template>
