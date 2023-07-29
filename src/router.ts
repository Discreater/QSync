import { createRouter, createWebHistory } from 'vue-router';
import { routes as mainRoutes } from './pages/main/router';
import Main from './pages/Main.vue';
import Lyric from './pages/Lyric.vue';

export const router = createRouter({
  routes: [
    {
      path: '/main', component: Main, name: 'main', children: mainRoutes,
    },
    {
      path: '/', redirect: '/main',
    },
    {
      path: '/lyric',
      component: Lyric,
      name: 'lyric',
      props: route => ({ path: route.query.path }),
    },
  ],
  history: createWebHistory(),
});
