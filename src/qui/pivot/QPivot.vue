<script setup lang="ts">
import { computed, provide, reactive, ref } from 'vue';
import QScrollbar from '../QScrollbar.vue';
import QHorizontalMenu from './QHorizontalMenu.vue';
import { qPivotRegisterKey } from './types';
import type { Item, ItemKey, PivotRegister } from './types';

const props = defineProps<{
  title?: string
  value: ItemKey
}>();

const activated = ref<ItemKey>(props.value);

const tabs: Item[] = reactive([]);

provide<PivotRegister>(qPivotRegisterKey, (tab: Item) => {
  tabs.push(tab);

  if (!activated.value)
    activate(tab);

  return {
    active: computed(() => activated.value === tab.key),

    unregister() {
      const index = tabs.indexOf(tab);
      tabs.splice(index, 1);

      if (activated.value === tab.key)
        activate(tabs[0]);
    },
  };
});

function activate(opt: Item) {
  if (opt)
    activated.value = opt.key;
}
</script>

<template>
  <div class="flex flex-1 flex-col h-full rounded-WINDOW  py-6 space-y-2">
    <h2 v-if="title" class="font-semibold">
      {{ title }}
    </h2>
    <QHorizontalMenu :top="tabs" :activated="activated" class="px-6" @item-click="activate" />
    <QScrollbar class="px-6">
      <slot />
      <!-- <component :is="child" v-for="(child, idx) of children" v-show="tabs[idx].key === activated" :key="idx" /> -->
    </QScrollbar>
  </div>
</template>
