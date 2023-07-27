<script setup lang="ts" generic="Row">
export interface Column {
  key: string
  title?: string
};

const { columns, data } = defineProps<{ columns: Column[]; data: Row[]; rowClassName?: (row: Row) => string }>();
</script>

<template>
  <table class="text-sm border-separate border-spacing-y-2">
    <thead>
      <tr>
        <th v-for="col in columns" :key="col.key">
          <slot name="headerCell" :column="col">
            {{ col.title }}
          </slot>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(row, rowIndex) in data" :key="rowIndex" :class="`${rowClassName?.(row)} group`">
        <td v-for="col in columns" :key="col.key" :data-col-key="col.key" :class="`${rowIndex % 2 === 1 ? 'bg-highlight' : 'group-hover:bg-highlight'} border-x-2 border-transparent`">
          <slot name="bodyCell" :column="col" :row="row" :row-idx="rowIndex" />
        </td>
      </tr>
      <slot v-if="data.length === 0" name="empty" />
    </tbody>
  </table>
</template>

<style lang="postcss" scoped>
td:first-child {
  @apply rounded-l-lg;
}
td:last-child {
  @apply rounded-r-lg;
}
</style>
