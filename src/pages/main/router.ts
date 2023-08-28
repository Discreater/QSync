import type { RouteRecordRaw } from 'vue-router';
import Home from './Home.vue';
import MusicLib from './MusicLib.vue';
import Settings from './Settings.vue';
import Source from './Source.vue';
import PlayQueue from './PlayQueue.vue';
import Account from './Account.vue';
import SearchResult from './SearchResult.vue';
import Track from './Track.vue';

export const routes = [
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
  {
    path: '/source', component: Source, name: 'source',
  },
  {
    path: '/play-queue', component: PlayQueue, name: 'play-queue',
  },
  {
    path: '/search-result', component: SearchResult, name: 'search-result', props: route => ({ query: route.query.q }),
  },
  {
    path: '/track', component: Track, name: 'track', props: route => ({ id: route.query.id }),
  },
] as RouteRecordRaw[];
