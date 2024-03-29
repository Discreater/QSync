import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import './style.postcss';
import messages from '@intlify/unplugin-vue-i18n/messages';
import { pinia, useMusyncStore } from './store';

import App from './App.vue';
import { router } from './pages/router';
import { attachLogger } from './utils/logger';

const app = createApp(App);

attachLogger();
app.use(pinia);

const host = window.location.hostname.split(':')[0];

const store = useMusyncStore();
store.init(host);

const i18n = createI18n({
  legacy: false,
  locale: store.locale,
  fallbackLocale: 'en',
  messages,
});

app.use(i18n);
app.use(router);

app.mount('#app');
