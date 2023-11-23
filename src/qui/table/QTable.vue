<script setup lang="ts" generic="Row">
import type { StyleValue } from 'vue';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { throttle } from 'lodash';
import type { Column } from './types';

const props = defineProps<{
  columns: Column[]
  data: Row[]
  showHead?: boolean
  rowClassName?: (row: Row) => string
  rowKey?: (row: Row) => string | number | symbol
}>();

function columnStyle(col: Column): StyleValue {
  const style: StyleValue = {};
  if (col.style) {
    if (col.style.textAlign)
      style.textAlign = col.style.textAlign;
  }
  return style;
}

const showColumns = ref(props.columns);

const tableResizeObserver = new ResizeObserver(throttle((entries: ResizeObserverEntry[]) => {
  for (const entry of entries) {
    if (entry.borderBoxSize) {
      const contentBoxSize: typeof entry.borderBoxSize[0] = Array.isArray(entry.borderBoxSize)
        ? entry.borderBoxSize[0]
        : entry.borderBoxSize;
      showColumns.value = props.columns.filter((col) => {
        if (col.style?.hidePx)
          return contentBoxSize.inlineSize > col.style.hidePx;
        else
          return true;
      });
    }
  }
}, 200));

const tableRef = ref<HTMLDivElement>();
onMounted(() => {
  tableResizeObserver.observe(tableRef.value!);
});

onUnmounted(() => {
  tableResizeObserver.disconnect();
});

const gridTemplateColumns = computed(() => {
  return showColumns.value.map(col => col.style?.gridTemplateColumn ?? 'minmax(0, 1fr)').join(' ');
});

function rowKeyMap(row: Row, idx: number) {
  return props.rowKey?.(row) ?? (row as any).a ?? idx;
}
</script>

<template>
  <div ref="tableRef" class="block text-xs w-full space-y-2 p-[0.5px]" role="table">
    <div v-if="showHead" class="flex-table items-center rounded px-2 h-12 text-passion font-bold" role="rowgroup">
      <div
        v-for="(column) in showColumns" :key="column.key" class="w-full block pr-2 last:pr-0" :style="columnStyle(column)"
        role="columnheader"
      >
        {{ column.title }}
      </div>
    </div>
    <div
      v-for="(row, rowIdx) in data" :key="rowKeyMap(row, rowIdx)"
      class="group flex-table items-center rounded px-2 h-12 even:bg-layer_1 even:ring-1 even:ring-black/10 odd:hover:bg-layer_1"
      :class="rowClassName?.(row) ?? ''"
      role="rowgroup"
    >
      <div
        v-for="(column) in showColumns" :key="column.key" :style="columnStyle(column)" class="w-full my-auto pr-2 last:pr-0"
        role="cell"
      >
        <slot :name="column.key" :row="row" :row-idx="rowIdx" />
      </div>
    </div>
  </div>
</template>

<style lang="postcss" scoped>
div {
  box-sizing: border-box;
}

.flex-table {
  display: grid;
  grid-template-columns: v-bind(gridTemplateColumns);
  grid-template-rows: 100% auto;
}
</style>
