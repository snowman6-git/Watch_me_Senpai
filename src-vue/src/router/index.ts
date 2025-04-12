import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/newgame',
      name: 'newgame',
      component: () => import('../views/NewGameView.vue'),
    },
    {
      path: '/savelist',
      name: 'savelist',
      component: () => import('../views/SaveView.vue'),
    },
  ],
})

export default router
