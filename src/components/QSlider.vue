<script setup lang="ts">
import { useDark } from '@vueuse/core';
import { computed, ref, toRefs } from 'vue';

interface Props {
  min?: number
  max?: number
  value?: number
}
const props = defineProps<Props>();
const emit = defineEmits<{
  'update:value': [value: number]
  'input': [value: number]
}>();
const { min, max } = toRefs(props);

const sliderValue = ref(0);
const dragging = ref(false);

const showValue = computed(() => props.value === undefined ? sliderValue.value : (dragging.value ? sliderValue.value : props.value));
const showValuePercent = computed(() => (max?.value && showValue?.value) ? (showValue.value / max.value) : 0);

function onMouseDown() {
  dragging.value = true;
}

function onMouseUp() {
  dragging.value = false;
  emit('update:value', sliderValue.value);
}

function onInput(e: InputEvent) {
  const v = Number.parseFloat((e.target as HTMLInputElement).value);
  sliderValue.value = v;
  emit('input', v);
}
const isDark = useDark();

const unreachedColor = computed(() => isDark.value ? '#ffffff80' : '#8a8a8a');
</script>

<template>
  <div class="flex items-center h-9 justify-between gap-4 min-w-[20rem]">
    <slot name="left" :value="showValue" />
    <input
      class="grow"
      type="range" :min="min" :max="max ?? 100" :value="showValue" :style="{
        background: `linear-gradient(to right, #f97316 ${showValuePercent * 100}%, ${unreachedColor} ${showValuePercent * 100}%)`,
      }"
      @mousedown="onMouseDown" @input="onInput($event as InputEvent)" @mouseup="onMouseUp"
    >
    <slot name="right" :value="showValue" />
  </div>
</template>

<style lang="postcss" scoped>
input[type="range"] {
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  width: 100%;
  outline: none;
  height: 0.25rem;
  background: #ffffff80;
  border-radius: 0.5rem;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb {
  @apply border-zinc-200 dark:border-[#454545] bg-passion;
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  height: 1.4rem;
  width: 1.4rem;

  border-radius: 50%;
  border-width: 0.4rem;
  border-style: solid;
  /*  slider progress trick  */
  transition: .2s ease-in-out;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb {
  @apply border-zinc-200 dark:border-[#454545] bg-passion;
  appearance: none;
  height: 1.4rem;
  width: 1.4rem;

  border-radius: 50%;
  border-radius: 50%;
  border-width: 0.4rem;
  border-style: solid;

  /* box-shadow: -407px 0 0 400px #f50; emove this line */
  transition: all .2s ease-in-out;
  box-sizing: border-box;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb:hover {
  border-width: 0.3rem;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb:active {
  border-width: 0.45rem;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb:hover {
  border-width: 0.3rem;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb:active {
  border-width: 0.45rem;
}
</style>
