import { createRouter, createWebHistory } from 'vue-router'
import { routes } from '@/shared/routes'
import { useUserStore } from '@/modules/auth/auth.store'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: routes.ROOT,
      redirect: { name: routes.LOGIN },
    },
    {
      path: '/:pathMatch(.*)*',
      name: routes.NOT_FOUND,
      component: () => import('./not-found-page.vue')
    },
    {
      path: '/auth',
      meta: { hideForLogged: true },
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
          path: 'password',
          children: [
            {
              path: 'recover',
              name: routes.RECOVER_PASSWORD,
              component: () => import('./auth/password/recover-password-page.vue'),
            },
            {
              path: 'edit',
              name: routes.EDIT_PASSWORD,
              component: () => import('./auth/password/edit-password-page.vue'),
            }
          ]
        },
      ],
    },
    {
      path: '/dashboard',
      name: routes.DASHBOARD,
      meta: { requiresAuth: true },
      component: () => import('./dashboard-page.vue'),
    },
  ]
})

router.beforeEach((to) => {
  const { isLoggedIn } = useUserStore()

  if (to.meta.requiresAuth && !isLoggedIn) return { name: routes.LOGIN }
  if (to.meta.hideForLogged && isLoggedIn) return { name: routes.DASHBOARD }
})

declare module 'vue-router' {
  interface RouteMeta {
    requiresAuth?: boolean
    hideForLogged?: boolean
  }
}
