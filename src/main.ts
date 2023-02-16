import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import './style.css';
import messages from '@intlify/unplugin-vue-i18n/messages';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';
import App from './App.vue';
import Home from './pages/Home.vue';
import MusicLib from './pages/MusicLib.vue';
import Settings from './pages/Settings.vue';

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
  {
    path: '/music-lib', component: MusicLib, name: 'music-lib',
  },
  {
    path: '/settings', component: Settings, name: 'settings',
  },
];

const router = createRouter({
  routes,
  history: createWebHistory(),
});
const pinia = createPinia();

const app = createApp(App);

app.use(i18n);
app.use(router);
app.use(pinia);

app.mount('#app');
