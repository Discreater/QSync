<script setup lang="ts" generic="T">
const { items, keyMap } = defineProps<{ items: T[]; keyMap: (t: T) => string }>();
</script>

<template>
  <ul v-if="items.length > 0" class="">
    <li v-for="item in items" :key="keyMap(item)">
      <slot name="item" :item="item" />
    </li>
  </ul>
  <slot v-else name="empty" />
</template>
