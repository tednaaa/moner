import { createRouter, createWebHistory } from 'vue-router'
import { routes } from './routes'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: routes.home,
      component: () => import('@/pages/home-page.vue'),
    },
    {
      path: routes.about,
      component: () => import('@/pages/about-page.vue'),
    }
  ]
})
