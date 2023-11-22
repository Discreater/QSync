<script setup lang="ts" generic="Row">
export interface Column {
  key: string
  title?: string
};
const props = defineProps<{
  columns: Column[]
  data: Row[]
  showHead?: boolean
  rowClassName?: (row: Row) => string
}>();

function rowClass(row: Row) {
  return `${props.rowClassName?.(row)} odd:bg-layer_1 even:hover:bg-layer_1`;
}
</script>

<template>
  <table class="text-xs border-separate border-spacing-y-2">
    <thead v-if="showHead">
      <tr>
        <th v-for="col in columns" :key="col.key">
          <slot name="headerCell" :column="col">
            {{ col.title }}
          </slot>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(row, rowIndex) in data" :key="rowIndex" class="h-12" :class="rowClass(row)">
        <td v-for="col in columns" :key="col.key" :data-col-key="col.key" class="border-x-2 border-x-transparent">
          <slot name="bodyCell" :column="col" :row="row" :row-idx="rowIndex" />
        </td>
      </tr>
      <slot v-if="data.length === 0" name="empty" />
    </tbody>
  </table>
</template>

<style lang="postcss" scoped>
td:first-child {
  @apply rounded-l;
}

td:last-child {
  @apply rounded-r;
}
</style>
