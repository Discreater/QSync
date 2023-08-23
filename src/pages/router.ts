import { createRouter, createWebHistory } from 'vue-router';
import { routes as mainRoutes } from './main/router';
import Main from './Main.vue';
import Lyric from './Lyric.vue';

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
      props: route => ({ id: route.query.id }),
    },
  ],
  history: createWebHistory(),
});
