/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    borderRadius: {
      none: '0',
      DEFAULT: '4px',
      WINDOW: '8px',
      full: '9999px',
    },
    extend: {
      colors: {
        main_bg: 'var(--main_bg)',
        menu_bg: 'var(--menu_bg)',
        hovering: 'var(--hovering)',
        main: 'var(--main)',
        passion: '#f97316',
        highlight: 'var(--highlight)',
      },
      transitionProperty: {
        position: 'top, bottom',
      },
      spacing: {
        title: '32px',
        player: '118px',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
};
