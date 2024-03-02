import { useUserStore } from '@/modules/auth/auth.store'
import { routes } from '@/shared/routes'
import { createRouter, createWebHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
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

router.beforeEach(async (to) => {
  const { checkUser } = useUserStore()
  const isLoggedIn = await checkUser()
  const isRootRoute = to.path === '/'

  if (isRootRoute || to.meta.requiresAuth && !isLoggedIn) return { name: routes.LOGIN }
  if (isRootRoute || to.meta.hideForLogged && isLoggedIn) return { name: routes.DASHBOARD }
})

declare module 'vue-router' {
  interface RouteMeta {
    requiresAuth?: boolean
    hideForLogged?: boolean
  }
}
