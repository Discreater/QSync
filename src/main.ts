import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import './style.css';
import messages from '@intlify/unplugin-vue-i18n/messages';
import { pinia, useQSyncStore } from './store';

import App from './App.vue';
import { router } from './router';

const app = createApp(App);

app.use(pinia);

const store = useQSyncStore();

const i18n = createI18n({
  legacy: false,
  locale: store.locale,
  fallbackLocale: 'en',
  messages,
});

app.use(i18n);
app.use(router);

app.mount('#app');
