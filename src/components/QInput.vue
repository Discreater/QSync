<script setup lang="ts">
const { id, type, placeholder, value } = defineProps<{ id: string; type: 'url' | 'text' | 'password'; placeholder?: string; value?: string }>();
const emit = defineEmits<{
  (e: 'update:value', value: string): void
}>();
function onValueChange(e: Event) {
  emit('update:value', (e.target as HTMLInputElement).value);
}
</script>

<template>
  <input
    :id="id" :type="type" :placeholder="placeholder" :value="value"
    class="bg-[#323232] focus:bg-[#1f1f1f] focus:border-main focus:outline-none focus:ring-1 focus:ring-gray-500/30 rounded border-b-2"
    @input="onValueChange($event)"
  >
</template>
