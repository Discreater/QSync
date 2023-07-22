<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import MenuItem from './MenuItem.vue';
import type { Item, ItemKey } from './types';

const { top, bottom, defaultActivated, activated: _activated } = defineProps<{
  top: Item[]
  bottom?: Item[]
  defaultActivated?: ItemKey | null
  activated?: ItemKey | null
}>();

const emit = defineEmits<{
  'itemClick': [item: Item]
}>();

const { t } = useI18n();

const localActivated = ref(defaultActivated);
const activated = computed(() => _activated ?? localActivated.value);

const handlerTop = computed(() => {
  if (!activated.value)
    return 0;
  let idx = top.findIndex(item => item.key === activated.value);
  if (idx !== -1)
    return `${(idx * 11) / 4}rem`;

  if (bottom) {
    idx = bottom.findIndex(item => item.key === activated.value);
    if (idx !== -1)
      return `calc(100% - ${((bottom.length - idx) * 11) / 4}rem)`;
  }
  return 0;
});

function handleItemClick(item: Item) {
  localActivated.value = item.key;
  emit('itemClick', item);
};
</script>

<template>
  <div class="h-full relative select-none hidden sm:flex md:flex flex-col justify-between">
    <div
      class="transition-position duration-400 absolute w-[3px] h-5 my-2.5 rounded-md bg-main" :style="{
        top: handlerTop,
      }"
    />
    <div>
      <MenuItem
        v-for="item in top" :key="item.key" :selected="item.key === activated" :name="t(item.name)"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-xl" />
      </MenuItem>
    </div><div>
      <MenuItem
        v-for="item in bottom" :key="item.key" :selected="item.key === activated" :name="t(item.name)"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-xl" />
      </MenuItem>
    </div>
  </div>
</template>
