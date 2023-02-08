import path from 'path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite';
import Icons from 'unplugin-icons/vite';
import webfontDownload from 'vite-plugin-webfont-dl';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    VueI18nPlugin({
      runtimeOnly: true,
      compositionOnly: true,
      include: [path.resolve(__dirname, './src/locales/**')],
    }),
    Icons({ compiler: 'vue3' }),
    webfontDownload([
      'https://fonts.googleapis.com/css2?family=Bitter&family=Noto+Serif+SC&display=swap',
    ]),
  ],
});
