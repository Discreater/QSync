<script setup lang="ts">
import { onMounted, provide, ref, watch } from 'vue';
import { qThemeKey } from './injects';
import type { QTheme } from '~/theme';
import { defaultTheme } from '~/theme';

const props = defineProps<{ theme?: QTheme }>();

const container = ref<HTMLElement | null>(null);

onMounted(() => {
  write();
});

watch(props, () => {
  write();
}, {
  deep: true,
  flush: 'pre',
});

function write() {
  const rootStyle = container.value?.style;
  const theme = props.theme ?? defaultTheme;
  if (rootStyle) {
    for (const [key, value] of Object.entries(theme))
      rootStyle.setProperty(`--${key}`, value);
  }
}
provide(qThemeKey, props.theme);
</script>

<template>
  <div ref="container">
    <slot />
  </div>
</template>
