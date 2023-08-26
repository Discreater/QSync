<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref, toRefs } from 'vue';
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

const handlerTop = computed(() => {
  if (!activated.value)
    return 0;
  let idx = top.value.findIndex(item => item.key === activated.value);
  if (idx !== -1)
    return `${(idx * 11) / 4}rem`;

  if (bottom?.value) {
    idx = bottom.value.findIndex(item => item.key === activated.value);
    if (idx !== -1)
      return `calc(100% - ${((bottom.value.length - idx) * 11) / 4}rem)`;
  }
  return 0;
});

function handleItemClick(item: Item) {
  localActivated.value = item.key;
  emit('itemClick', item);
};
</script>

<template>
  <div class="h-full relative select-none flex flex-col justify-between">
    <div
      class="transition-position duration-400 absolute w-[3px] h-5 my-2.5 rounded-md bg-passion" :style="{
        top: handlerTop,
      }"
    />
    <div>
      <MenuItem
        v-for="item in top" :key="item.key" :selected="item.key === activated" :name="t(item.name)" :only-icon="onlyIcon"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-xl" />
      </MenuItem>
    </div>
    <div>
      <MenuItem
        v-for="item in bottom" :key="item.key" :selected="item.key === activated" :name="t(item.name)" :only-icon="onlyIcon"
        @click="handleItemClick(item)"
      >
        <Component :is="item.icon" class="text-xl" />
      </MenuItem>
    </div>
  </div>
</template>
