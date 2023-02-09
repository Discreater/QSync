import path from 'path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite';
import Icons from 'unplugin-icons/vite';
import webfontDownload from 'vite-plugin-webfont-dl';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '~/': `${path.resolve(__dirname, 'src')}/`,
    },
  },
  plugins: [
    vue(),
    VueI18nPlugin({
      runtimeOnly: true,
      compositionOnly: true,
      include: [path.resolve(__dirname, './src/locales/**')],
    }),
    Icons({
      compiler: 'vue3',
      customCollections: {
        qsync: {
          // modified fluent:subtract-16-regular
          minimize: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><rect width="10" height="0.5" x="3" y="7.5" fill="currentColor" rx=".5" /></svg>',
        },
      },
    }),
    webfontDownload([
      'https://fonts.googleapis.com/css2?family=Bitter&family=Noto+Serif+SC&display=swap',
    ]),
  ],
});
