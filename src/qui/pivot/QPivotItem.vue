<script setup lang="ts">
import { inject, onUnmounted, reactive } from 'vue';
import type { PivotRegister } from './types';
import { qPivotRegisterKey } from './types';

const props = defineProps<{ value: string; name: string }>();

const register = inject<PivotRegister>(qPivotRegisterKey);
const tab = reactive({
  key: props.value,
  name: props.name,
});

if (!register)
  throw new Error('QTabItem must be used inside QTab');

const { active, unregister } = register(tab);

onUnmounted(unregister);
</script>

<template>
  <div v-if="active">
    <slot />
  </div>
</template>
