import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import './style.css';
import messages from '@intlify/unplugin-vue-i18n/messages';
import App from './App.vue';

const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en',
  messages,
});

const app = createApp(App);

app.use(i18n);
app.mount('#app');
