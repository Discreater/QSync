import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import './style.css';
import messages from '@intlify/unplugin-vue-i18n/messages';
import { createRouter, createWebHistory } from 'vue-router';
import App from './App.vue';
import Home from './pages/Home.vue';

const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en',
  messages,
});

const routes = [
  {
    path: '/', component: Home, name: 'home',
  },
];

const router = createRouter({
  routes,
  history: createWebHistory(),
});

const app = createApp(App);

app.use(i18n);
app.use(router);
app.mount('#app');
