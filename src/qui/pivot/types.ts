import type { Component, ComputedRef } from 'vue';

export interface Item {
  key: ItemKey
  icon?: Component
  name: string
}

export type ItemKey = string | symbol;

export const qPivotRegisterKey = Symbol('qPivotRegister');
export type PivotRegister = (tab: Item) => {
  active: ComputedRef<boolean>
  unregister: () => void
};
