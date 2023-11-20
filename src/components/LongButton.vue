<script setup lang="ts">
import type { Component } from 'vue';
import { ref } from 'vue';
import { bgLayer, useLayerLevel } from './injects';
import IconDown from '~icons/fluent/chevron-down-24-regular';
import IconUp from '~icons/fluent/chevron-up-24-regular';

const props = withDefaults(defineProps<{ icon?: Component; text?: string; droppable?: boolean; layer?: number }>(), {
  droppable: false,
});

const dropped = ref(false);

function toggleDropped() {
  if (props.droppable)
    dropped.value = !dropped.value;
}

const level = useLayerLevel(props.layer);
</script>

<template>
  <div>
    <div
      class="group flex justify-between items-center min-w-[20rem]
         rounded select-none cursor-default hover:bg-hovering
          h-[4.25rem] px-5 ring-1 ring-gray-200 dark:ring-black/10" :class="bgLayer(level)"
      @click="toggleDropped()"
    >
      <div class="flex items-center">
        <Component :is="icon" v-if="icon" class="h-7 w-7 mr-4" />
        <span>{{ text }}</span>
      </div>
      <div class="flex items-center">
        <slot name="extra" />
        <button
          v-if="droppable" class="disabled:opacity-25
          cursor-default
          rounded  h-8 ml-3"
        >
          <IconUp v-if="dropped" />
          <IconDown v-else />
        </button>
      </div>
    </div>
    <div v-if="dropped" class="bg-[#fdfefe] dark:bg-[#323232] mt-0.5 px-[60px] py-2">
      <slot name="drop" :dropped="dropped" />
    </div>
  </div>
</template>
