<script setup lang="ts">
import type { Component } from 'vue';
import { ref } from 'vue';
import { useToggle } from '@vueuse/core';
import IconDown from '~icons/fluent/chevron-down-24-regular';
import IconUp from '~icons/fluent/chevron-up-24-regular';

withDefaults(defineProps<{ icon?: Component; text?: string; droppable?: boolean }>(), {
  droppable: false,
});

const dropped = ref(false);
const toggleDropped = useToggle(dropped);
</script>

<template>
  <div>
    <div
      class="group flex justify-between items-center min-w-[20rem]
         bg-[#323232] rounded-md select-none cursor-default hover:bg-[#3e3e3e]
          h-[4.25rem] px-[17px] ring-1 ring-black/10" @click="toggleDropped()"
    >
      <div class="flex space-x-4 items-center">
        <Component :is="icon" v-if="icon" class="text-2xl" />
        <span>{{ text }}</span>
      </div>
      <div />
      <div class="flex items-center">
        <slot name="extra" />
        <button
          v-if="droppable" class=" disabled:opacity-25
          cursor-default
          rounded-md  h-8 ml-3"
        >
          <IconUp v-if="dropped" />
          <IconDown v-else />
        </button>
      </div>
    </div>
    <slot v-if="dropped" name="drop" :dropped="dropped" />
  </div>
</template>
