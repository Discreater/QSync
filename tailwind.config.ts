import type { Config } from 'tailwindcss';

/// It looks like tailwind can't import other typescript files.
/// So we put the default theme and theme type here,
/// and export them to the `./src/theme.ts` file.

/**
 * It was used to generate the tailwind colors and type of theme.
 * Any entry will generate it's corresponding css variable.
 */
export const defaultTheme = {
  main: '#4cc2ff',
  main_bg: '#f3f3f3',
  menu_bg: '#f3f3f3',
  hovering: '#f5f4f4b0',
  highlight: '#f9f9f960',
  layer_0: '#f9f9f9',
  layer_1: '#fdfdfd',
  layer_2: '#fefefe',
};

export type QTheme = typeof defaultTheme;

export const defaultDarkTheme: QTheme = {
  main: '#4cc2ff',
  main_bg: '#202020',
  menu_bg: '#202020',
  hovering: '#4b4b4bb0',
  highlight: '#4b4b4b60',
  layer_0: '#272727',
  layer_1: '#323232',
  layer_2: '#3e3e3e',
};

const staticColors = {
  passion: '#f97316',
};

const colors = Object.keys(defaultTheme) as (keyof QTheme)[];
const varColors: any = {};
for (const key of colors)
  varColors[key] = `var(--${key})`;

const safelist = [
  ...colors.map(key => `bg-${key}`),
  ...colors.map(key => `odd:bg-${key}`),
  ...colors.map(key => `even:bg-${key}`),
];

export const breakpoints = {
  sm: '674px',
  md: '1070px',
};

export default {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  safelist,
  theme: {
    screens: breakpoints,
    borderRadius: {
      none: '0',
      DEFAULT: '4px',
      WINDOW: '8px',
      full: '9999px',
    },
    extend: {
      colors: {
        ...varColors,
        ...staticColors,
      },
      fontSize: {
        '2xs': '0.625rem',
      },
      transitionProperty: {
        'position-x': 'left, right',
        'position-y': 'top, bottom',
        'shape': 'width, height',
      },
      spacing: {
        player: '118px',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
} satisfies Config;
