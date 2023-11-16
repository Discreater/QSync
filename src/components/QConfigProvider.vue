<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import type { QTheme, QThemeItem } from './types';
import { defaultTheme } from './types';
import { isDark } from '~/utils';

const props = withDefaults(
  defineProps<{ theme?: QTheme }>(),
  {
    theme: () => defaultTheme,
  },
);

const container = ref<HTMLElement | null>(null);

const localTheme = ref(props.theme);

onMounted(() => {
  write(localTheme.value, isDark.value);
});

watch(localTheme, () => {
  write(localTheme.value, isDark.value);
}, {
  deep: true,
  flush: 'pre',
});

watch(isDark, (isDark) => {
  write(localTheme.value, isDark);
});

function write(data: QTheme, isDark: boolean) {
  const rootStyle = container.value?.style;
  if (rootStyle) {
    for (const [key, value] of Object.entries(data)) {
      if (typeof value === 'string') {
        rootStyle.setProperty(`--${key}`, value);
      } else {
        const item = value as QThemeItem;
        rootStyle.setProperty(`--${key}`, isDark ? item.dark : item.light);
      }
    }
  }
}
</script>

<template>
  <div ref="container">
    <slot />
  </div>
</template>
