<script setup lang="ts" generic="T">
defineProps<{
  items: T[]
  keyMap: (t: T) => string
}>();
</script>

<template>
  <ul v-if="items.length > 0" class="">
    <li v-for="item, idx in items" :key="keyMap(item)">
      <slot name="item" :item="item" :idx="idx" />
    </li>
  </ul>
  <slot v-else name="empty" />
</template>
