import { useDark } from '@vueuse/core';
import { ref, watch } from 'vue';
import { isDark } from '~/utils';

type StaticThemeItem = string;

interface ThemeItem {
  dark: string
  light: string
}

export interface Theme {
  main: StaticThemeItem
  main_bg: ThemeItem
  menu_bg: ThemeItem
  hovering: ThemeItem
  highlight: ThemeItem
}

export const defaultTheme: Theme = {
  main: '#4cc2ff',
  main_bg: {
    dark: '#272727',
    light: '#f9f9f9',
  },
  menu_bg: {
    dark: '#202020',
    light: '#f3f3f3',
  },
  hovering: {
    dark: '#4b4b4bb0',
    light: '#f5f4f4b0',
  },
  highlight: {
    dark: '#4b4b4b60',
    light: '#f9f9f960',
  },
};

export function useTheme(theme?: Theme) {
  const data = ref(theme ?? defaultTheme);

  watch(data, () => {
    const isDark = useDark();
    write(data.value, isDark.value);
  }, {
    deep: true,
    flush: 'pre',
    immediate: true,
  });

  watch(isDark, (isDark) => {
    write(data.value, isDark);
  });

  function write(data: Theme, isDark: boolean) {
    const rootStyle = document.documentElement.style;
    for (const [key, value] of Object.entries(data)) {
      if (typeof value === 'string') {
        rootStyle.setProperty(`--${key}`, value);
      } else {
        const item = value as ThemeItem;
        rootStyle.setProperty(`--${key}`, isDark ? item.dark : item.light);
      }
    }
  }
  return data;
}
