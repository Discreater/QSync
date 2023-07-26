<script setup lang="ts">
import { computed, ref, toRefs } from 'vue';

interface Props {
  min?: number
  max?: number
  value?: number
  formatter?: (v: number) => string
}
const props = defineProps<Props>();
const emit = defineEmits<{
  'update:value': [value: number]
}>();

const { min, max, value } = toRefs(props);

const sliderValue = ref(0);
const dragging = ref(false);

const showValue = computed(() => dragging.value ? sliderValue.value : value?.value);
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
}
</script>

<template>
  <div class="flex items-center h-9 justify-between gap-4">
    <span class="text-xs w-14 pl-1"> {{ props.formatter?.(showValue ?? 0) }} </span>
    <input
      class="grow"
      type="range" :min="min" :max="max ?? 100" :value="showValue" :style="{
        background: `linear-gradient(to right, #f97316 ${showValuePercent * 100}%, #ffffff80 ${showValuePercent * 100}%)`,
      }" :title="showValue?.toString()"
      @mousedown="onMouseDown" @input="onInput($event as InputEvent)" @mouseup="onMouseUp"
    >
    <span class="text-xs text-right w-14 pr-1">{{ max && showValue ? props.formatter?.(max - showValue) : '' }}</span>
  </div>
</template>

<style scoped>
input[type="range"] {
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  width: 100%;
  cursor: pointer;
  outline: none;
  height: 0.25rem;
  background: #ffffff80;
  border-radius: 0.5rem;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb {
  /* removing default appearance */
  -webkit-appearance: none;
  appearance: none;
  /* creating a custom design */
  height: 1.5rem;
  width: 1.5rem;
  background-color: rgb(249 115 22 / 1);
  border-radius: 50%;
  border: 0.4rem solid #454545;
  /*  slider progress trick  */
  transition: .2s ease-in-out;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb {
  height: 1.5rem;
  width: 1.5rem;
  background-color: #f97316;
  border-radius: 50%;
  border: 0.4rem solid #454545;

  /* box-shadow: -407px 0 0 400px #f50; emove this line */
  transition: .2s ease-in-out;
}

/* Thumb: webkit */
input[type="range"]::-webkit-slider-thumb:hover {
  border: 0.3rem solid #454545;
}

/* Thumb: Firefox */
input[type="range"]::-moz-range-thumb:hover {
  border: 0.3rem solid #454545;
}
</style>