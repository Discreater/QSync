import type { Component } from 'vue';

export interface Item {
  key: ItemKey
  icon?: Component
  name: string
}

export type ItemKey = string | symbol;

export type QStaticThemeItem = string;

export interface QThemeItem {
  dark: string
  light: string
}

export interface QTheme {
  main: QStaticThemeItem
  main_bg: QThemeItem
  content_bg: QThemeItem
  menu_bg: QThemeItem
  hovering: QThemeItem
  highlight: QThemeItem
}
export const defaultTheme: QTheme = {
  main: '#4cc2ff',
  main_bg: {
    dark: '#202020',
    light: '#f3f3f3',
  },
  content_bg: {
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
