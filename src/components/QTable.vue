<script setup lang="ts" generic="T">
const { items, column, keyMap } = defineProps<{ items: T[]; column: string[]; keyMap: (t: T) => string }>();
</script>

<template>
  <ul v-if="items.length > 0" class="">
    <li v-for="item, row in items" :key="keyMap(item)">
      <div v-for="col in column" :key="col">
        <slot :name="col" :item="item" :row="row" :col="col" />
      </div>
    </li>
  </ul>
  <slot v-else name="empty" />
</template>
