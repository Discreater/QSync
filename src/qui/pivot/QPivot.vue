<script setup lang="ts">
import { computed, ref, useSlots } from 'vue';
import QScrollbar from '../QScrollbar.vue';
import QHorizontalMenu from './QHorizontalMenu.vue';
import type { Item, ItemKey } from './types';
import QPivotItem from './QPivotItem.vue';

const props = defineProps<{
  title?: string
  value: ItemKey
}>();

const activated = ref<ItemKey>(props.value);

const children = useSlots().default?.();
const tabs = children?.filter(child => child.type === QPivotItem).map((child) => {
  return {
    key: child.props!.value as ItemKey,
    name: child.props!.name as string,
  };
}) ?? [];

const activatedKey = computed(() => tabs.findIndex(i => i.key === activated.value));

function onValueChange(opt: Item) {
  activated.value = opt.key;
}
</script>

<template>
  <div class="flex flex-1 flex-col h-full rounded-WINDOW px-6 py-6 space-y-2">
    <h2 v-if="title" class="font-semibold">
      {{ title }}
    </h2>
    <QHorizontalMenu :top="tabs" :activated="activated" @item-click="onValueChange" />
    <QScrollbar>
      <component :is="children[activatedKey]" v-if="children" />
      <!-- <component :is="child" v-for="(child, idx) of children" v-show="tabs[idx].key === activated" :key="idx" /> -->
    </QScrollbar>
  </div>
</template>
