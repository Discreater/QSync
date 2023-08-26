/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        main_w_bg: '#f9f9f9',
        main_d_bg: '#272727',
        menu_w_bg: '#f3f3f3',
        menu_d_bg: '#202020',
        main: 'var(--main)',
        passion: '#f97316',
        highlight: '#4b4b4b60',
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
