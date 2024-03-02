import { defineStore, acceptHMRUpdate } from 'pinia'

import { apiAuthorizeUser, apiCreateUser, apiGetUser } from '@/shared/api/user/user.api'
import { statusCodes } from '@/shared/api/status-codes'
import { apiInstance } from '@/shared/api/instance'
import { routes } from '@/shared/routes'
import { router } from '@/pages/router'

export const useUserStore = defineStore({
  id: 'user',
  state: () => ({
    email: '',
    username: '',
    isLoggedIn: false,
  }),
  actions: {
    async create(email: string, username: string, password: string) {
      const { status } = await apiCreateUser(email, username, password)
      if (status !== statusCodes.CREATED) return

      this.$patch({
        email: email,
        username: username,
        isLoggedIn: true,
      })
      router.push({ name: routes.DASHBOARD })
    },

    async authorize(emailOrUsername: string, password: string) {
      const { status, data } = await apiAuthorizeUser(emailOrUsername, password)
      if (status !== statusCodes.OK) return

      this.$patch({
        email: data.email,
        username: data.username,
        isLoggedIn: true,
      })
      router.push({ name: routes.DASHBOARD })
    },
    async checkUser() {
      try {
        const { status, data } = await apiGetUser()
        if (status === statusCodes.UNAUTHORIZED) return false

        this.$patch({
          email: data.email,
          username: data.username,
          isLoggedIn: true,
        })

        return true
      } catch {
        return false
      }
    },

    logout() {
      this.$reset()
    },
  },
})

apiInstance.interceptors.response.use(response => {
  return response
}, error => {
  const store = useUserStore()

  if (error.response && error.response.status === 401) {
    store.logout()
  }
  return Promise.reject(error)
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
