<script setup lang="ts">
import { computed, ref } from 'vue';
import QButton from './QButton.vue';
import type { Item, ItemKey } from './types';
import QMenu from './QMenu.vue';

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
</script>

<template>
  <div class="relative w-36 h-10">
    <QButton v-if="!open" :text="selected?.name" class="w-full" :dropdown="true" @click="open = !open" />
    <div
      v-else class="absolute p-2 w-full rounded-sm dark:bg-main_d_bg ring-2 ring-black/10" :style="{
        top: menuOffset,
      }"
    >
      <QMenu :top="options" :activated="selected?.key" @item-click="onValueChange" />
    </div>
  </div>
</template>
