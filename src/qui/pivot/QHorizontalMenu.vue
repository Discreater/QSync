<script setup lang="ts">
import { computed, ref, toRefs, watchEffect } from 'vue';
import TabHeadItem from './TabHeadItem.vue';
import type { Item, ItemKey } from './types';

const props = defineProps<{
  top: Item[]
  bottom?: Item[]
  defaultActivated?: ItemKey | null
  activated?: ItemKey | null
  onlyIcon?: boolean
}>();
const emit = defineEmits<{
  'itemClick': [item: Item]
}>();

const { top, bottom, activated: _activated } = toRefs(props);

const localActivated = ref(props.defaultActivated);
const activated = computed(() => _activated?.value ?? localActivated.value);

const noActivated = computed(() => (props.top.findIndex(i => i.key === activated.value) === -1
                            && (props.bottom?.findIndex(i => i.key === activated.value) === -1))
                            ?? false);

const topContainer = ref<HTMLDivElement>();
const bottomContainer = ref<HTMLDivElement>();

function leftOf(container: typeof topContainer, idx: number) {
  const left = (container.value?.children[idx] as HTMLElement)?.offsetLeft ?? 0;

  return `${left}px`;
}

function widthOf(container: typeof topContainer, idx: number) {
  const width = container.value?.children[idx].clientWidth ?? 0;
  return `${width}px`;
}

const handlerWidth = ref('0');

const handlerLeft = ref('0');

watchEffect(() => {
  if (!activated.value) {
    handlerLeft.value = '0';
    handlerWidth.value = '0';
    return;
  }
  let idx = top.value.findIndex(item => item.key === activated.value);
  if (idx !== -1) {
    handlerWidth.value = widthOf(topContainer, idx);
    handlerLeft.value = leftOf(topContainer, idx);
    return;
  }
  if (bottom?.value) {
    idx = bottom.value.findIndex(item => item.key === activated.value);
    if (idx !== -1) {
      handlerWidth.value = widthOf(bottomContainer, idx);
      handlerLeft.value = leftOf(bottomContainer, idx);
      return;
    }
  }
  return 0;
});

function handleItemClick(item: Item) {
  localActivated.value = item.key;
  emit('itemClick', item);
};
</script>

<template>
  <div class="relative select-none flex justify-between">
    <div
      v-show="!noActivated"
      class="transition-all duration-400 absolute h-[3px] rounded bg-passion" :style="{
        width: handlerWidth,
        bottom: 0,
        left: handlerLeft,
      }"
    />
    <div ref="topContainer" class="flex space-x-4 items-center">
      <TabHeadItem
        v-for="item in top" :key="item.key" :selected="item.key === activated" :name="item.name"
        @click="handleItemClick(item)"
      />
    </div>
    <div ref="bottomContainer" class="flex space-x-2  items-center">
      <TabHeadItem
        v-for="item in bottom" :key="item.key" :selected="item.key === activated" :name="item.name"
        @click="handleItemClick(item)"
      />
    </div>
  </div>
</template>
