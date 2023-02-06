/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    screens: {
      sm: '675px',
      md: '1010px',
    },
    extend: {
      colors: {
        main_w_bg: '#f9f9f9',
        main_d_bg: '#272727',
        menu_w_bg: '#f3f3f3',
        menu_d_bg: '#202020',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
};
