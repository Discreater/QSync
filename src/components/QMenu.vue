<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref, toRefs, watchEffect } from 'vue';
import MenuItem from './MenuItem.vue';
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

const { t } = useI18n();

const localActivated = ref(props.defaultActivated);
const activated = computed(() => _activated?.value ?? localActivated.value);

const noActivated = computed(() => (props.top.findIndex(i => i.key === activated.value) === -1
  && (props.bottom?.findIndex(i => i.key === activated.value) === -1))
  ?? false);

const topContainer = ref<HTMLDivElement>();
const bottomContainer = ref<HTMLDivElement>();

function topOf(container: typeof topContainer, idx: number) {
  const top = (container.value?.children[idx] as HTMLElement)?.offsetTop ?? 0;
  return top;
}

const handlerTop = ref('0');

watchEffect(() => {
  if (!activated.value) {
    handlerTop.value = '0';
    return;
  }
  let idx = top.value.findIndex(item => item.key === activated.value);
  if (idx !== -1) {
    handlerTop.value = `${topOf(topContainer, idx)}px`;
    return;
  }
  if (bottom?.value) {
    idx = bottom.value.findIndex(item => item.key === activated.value);
    if (idx !== -1)
      handlerTop.value = `${topOf(bottomContainer, idx)}px`;
  }
});

function handleItemClick(item: Item) {
  localActivated.value = item.key;
  emit('itemClick', item);
};
</script>

<template>
  <div class="h-full relative select-none flex flex-col justify-between">
    <div
      v-show="!noActivated"
      class="transition-position-y duration-400 absolute w-[3px] h-5 my-2.5 rounded bg-passion" :style="{
        top: handlerTop,
      }"
    />
    <div ref="topContainer">
      <MenuItem
        v-for="item in top" :key="item.key" :selected="item.key === activated" :name="t(item.name)" :only-icon="onlyIcon"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-base" />
      </MenuItem>
    </div>
    <div ref="bottomContainer">
      <MenuItem
        v-for="item in bottom" :key="item.key" :selected="item.key === activated" :name="t(item.name)" :only-icon="onlyIcon"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-base" />
      </MenuItem>
    </div>
  </div>
</template>
