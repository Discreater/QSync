<script setup lang="ts">
import { computed, ref } from 'vue';
import { onClickOutside } from '@vueuse/core';
import QButton from './QButton.vue';
import type { Item, ItemKey } from './types';
import QMenu from './QMenu.vue';
import IconDown from '~icons/fluent/chevron-down-24-regular';

const props = defineProps<{
  value?: string
  options: Item[]
}>();
const emit = defineEmits<{
  'update:value': [value: ItemKey]
}>();
const open = ref(false);
const selected = ref<Item | undefined>(props.options.find(v => v.key === props.value));

function onValueChange(opt: Item) {
  selected.value = opt;
  open.value = false;
  emit('update:value', opt.key);
}

const menuOffset = computed(() => {
  const idx = props.options.findIndex(opt => opt.key === selected.value?.key);
  return `-${(idx * 11) / 4 + 0.5}rem`;
});

const menuRef = ref(null);

onClickOutside(menuRef, () => {
  open.value = false;
});
</script>

<template>
  <div class="relative w-36 h-10">
    <QButton v-if="!open" class="w-full" @click="open = !open">
      {{ selected?.name }}
      <IconDown />
    </QButton>
    <div
      v-else ref="menuRef" class="absolute p-1 w-full rounded bg-main_bg ring-1 ring-black/10" :style="{
        top: menuOffset,
      }"
    >
      <QMenu :top="options" :activated="selected?.key" @item-click="onValueChange" />
    </div>
  </div>
</template>
