<script setup lang="ts">
import { ref } from 'vue';
import QButton from './QButton.vue';
import LongButton from './LongButton.vue';
import { logger } from '~/utils/logger';
import { open } from '~/platforms/dialog';

const props = defineProps<{ id: string; type: 'url' | 'directory' | 'text' | 'password'; placeholder?: string; value?: string; label?: string }>();
const emit = defineEmits<{
  (e: 'update:value', value: string): void
}>();

const inputType = props.type === 'directory' ? 'file' : props.type;
function onValueChange(e: Event) {
  emit('update:value', (e.target as HTMLInputElement).value);
}
const showDir = ref<string | undefined>(undefined);
async function onClcickSelect() {
  const dirs = await open({
    title: 'Select directory',
    directory: true,
    recursive: true,
  });
  if (dirs === undefined)
    return;
  logger.trace(dirs);
  let dir;
  if (Array.isArray(dirs))
    dir = dirs[0];
  else if (dirs === null)
    dir = '';
  else
    dir = dirs;
  showDir.value = dir;

  emit('update:value', dir);
}
</script>

<template>
  <LongButton :text="label">
    <template #extra>
      <div v-if="type === 'directory'" class="flex items-center gap-3">
        <p>{{ showDir }}</p>
        <QButton @click="onClcickSelect()">
          ...
        </QButton>
      </div>
      <input
        v-else :id="id" :type="inputType" :placeholder="placeholder" :value="value"
        class="bg-[#323232] focus:bg-[#1f1f1f] focus:border-main focus:outline-none ring-1 ring-gray-500/30 rounded border-b h-8 px-2"
        @input="onValueChange($event)"
      >
    </template>
  </LongButton>
</template>
