<script setup lang="ts" generic="Row">
export interface Column {
  key: string
  title?: string
};
defineProps<{ columns: Column[]; data: Row[]; showHead?: boolean; rowClassName?: (row: Row) => string }>();
</script>

<template>
  <table class="text-sm border-separate border-spacing-y-2">
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
      <tr v-for="(row, rowIndex) in data" :key="rowIndex" :class="`${rowClassName?.(row)} ${rowIndex % 2 === 1 ? 'bg-highlight' : 'hover:bg-highlight'}`">
        <td
          v-for="col in columns" :key="col.key" :data-col-key="col.key" class="border-x-2 border-transparent"
        >
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
