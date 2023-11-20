import { inject, provide, ref } from 'vue';

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

export function useLayerLevel(layer?: number) {
  const level = inject(qLayerLevelKey, layer ?? 0);
  provide(qLayerLevelKey, level + 1);
  return level;
}

export function bgLayer(level: number) {
  switch (level) {
    case 0: return 'bg-layer_0';
    case 1: return 'bg-layer_1';
    case 2: return 'bg-layer_2';
    default :return 'bg-[#ff0000]';
  }
}
