import { createRouter, createWebHistory } from 'vue-router'
import { routes } from '@/shared/routes'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: routes.ROOT,
      redirect: { name: routes.LOGIN }
    },
    {
      path: '/:username/:projectId',
      name: routes.PROJECT,
      component: () => import('./project-page.vue'),
    },
    {
      path: '/:username',
      name: routes.PROFILE,
      component: () => import('./profile-page.vue'),
    },
    {
      path: '/auth',
      children: [
        {
          path: 'login',
          name: routes.LOGIN,
          component: () => import('./auth/login-page.vue'),
        },
        {
          path: 'sign-up',
          name: routes.SIGN_UP,
          component: () => import('./auth/sign-up-page.vue'),
        },
        {
          path: 'recover',
          name: routes.RECOVER,
          component: () => import('./auth/recover-page.vue'),
        },
      ],
    },
  ]
})
