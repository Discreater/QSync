<script setup lang="ts">
import { ref } from 'vue';

interface Option {
  label: string
  value: string
}

const { value, options } = defineProps<{
  value?: string
  options: Option[]
}>();
const emit = defineEmits<{
  (e: 'update:value', value: string): void
}>();
const open = ref(false);
const selected = ref<Option | undefined>(options[0]);

function onValueChange(opt: Option) {
  selected.value = opt;
  open.value = false;
  emit('update:value', opt.value);
}
</script>

<template>
  <div>
    <div class="bg-transparent px-2.5 py-2.5 border-b-2 border-gray-200 dark:border-gray-700" @click="open = !open">
      {{ selected?.label }}
    </div>
    <div v-if="open">
      <div v-for="opt in options" :key="opt.value" :value="opt.value" @click="onValueChange(opt)">
        {{ opt.label }}
      </div>
    </div>
  </div>
</template>
