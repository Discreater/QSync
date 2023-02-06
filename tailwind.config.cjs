/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        black_bg: '#272727',
        white_bg: '#f9f9f9',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
};
