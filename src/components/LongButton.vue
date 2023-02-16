<script setup lang="ts">
import type { Component } from 'vue';
import { ref } from 'vue';
import { useToggle } from '@vueuse/core';
import Button from './Button.vue';
import IconDown from '~icons/fluent/chevron-down-24-regular?width=1em';
import IconUp from '~icons/fluent/chevron-up-24-regular?width=1em';

const { icon, text, droppable } = defineProps<{ icon: Component; text: string; droppable: boolean }>();

const dropped = ref(false);
const toggleDropped = useToggle(dropped);
</script>

<template>
  <div
    class="group flex justify-between items-center
         bg-[#323232] rounded-md select-none cursor-default
          h-12 px-3" @click="toggleDropped()"
  >
    <div class="flex space-x-4 items-center">
      <Component :is="icon" />
      <span>{{ text }}</span>
    </div>
    <div />
    <div class="flex items-center">
      <slot name="extra" />
      <button
        v-if="droppable" class="hover:enabled:bg-[#3e3e3e] disabled:opacity-25
        group-hover:enabled:bg-[#3e3e3e]
          cursor-default
          rounded-md px-2 h-8 ml-3"
      >
        <IconUp v-if="dropped" />
        <IconDown v-else />
      </button>
    </div>
  </div>
  <slot v-if="dropped" name="drop" />
</template>
