<script setup lang="ts">
import { computed, ref } from 'vue';
import QHoverButton from './QHoverButton.vue';
import IconDismiss from '~icons/fluent/dismiss-12-regular';

const props = defineProps<{ id: string; type: 'url' | 'text' | 'password'; placeholder?: string; value?: string }>();
const emit = defineEmits<{
  (e: 'update:value', value: string): void
}>();

const focus = ref(false);

const localValue = ref<string>();

const showValue = computed(() => props.value != null ? props.value : localValue.value);

function onValueChange(e: Event) {
  const v = (e.target as HTMLInputElement).value;
  localValue.value = v;
  emit('update:value', v);
}

function clear() {
  localValue.value = '';
  emit('update:value', '');
}
</script>

<template>
  <div
    class="group ring-1 ring-gray-500/30 rounded border-b-2 h-8 px-2 flex"
    :class="focus
      ? 'bg-[#fff] dark:bg-[#1f1f1f] border-passion'
      : 'bg-[#fafcfd] dark:bg-[#323232] border-gray-500 dark:border-white/50'"
  >
    <input
      :id="id" :type="type" :placeholder="placeholder" :value="showValue"
      class="bg-transparent focus:outline-none w-full" autocomplete="off" @focusin="focus = true"
      @focusout="focus = false"
      @input="onValueChange($event)"
    >
    <QHoverButton v-if="showValue" size="small" @click="clear()">
      <IconDismiss class="text-xs" />
    </QHoverButton>
    <slot name="extra" />
  </div>
</template>
