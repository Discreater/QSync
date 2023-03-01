import { createRouter, createWebHistory } from 'vue-router';
import Account from './pages/Account.vue';
import Home from './pages/Home.vue';
import MusicLib from './pages/MusicLib.vue';
import Settings from './pages/Settings.vue';

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
  {
    path: '/account', component: Account, name: 'account',
  },
];

export const router = createRouter({
  routes,
  history: createWebHistory(),
});
