import type { Component } from 'vue';

export interface Item {
  key: ItemKey
  icon?: Component
  name: string
}

export type ItemKey = string | symbol;
