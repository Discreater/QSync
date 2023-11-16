import { provide, ref } from 'vue';

import type Scrollbar from 'smooth-scrollbar';

export const qLayerLevelKey = Symbol('qInjectLayerLevel');
export const qThemeKey = Symbol('qTheme');
export const qInnerScrollBarKey = Symbol('qScrollBar');

export function useInnerScrollbar() {
  const scrollbar = ref<Scrollbar | null>(null);
  provide(qInnerScrollBarKey, (s: Scrollbar) => {
    scrollbar.value = s;
  });
  return scrollbar;
}
