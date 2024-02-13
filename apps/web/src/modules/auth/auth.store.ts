import { defineStore, acceptHMRUpdate } from 'pinia'

import { apiCreateUser } from '@/shared/api/user/user.api'

export const useUserStore = defineStore({
  id: 'user',
  state: () => ({
    email: '',
    username: '',
    isLoggedIn: false,
  }),

  actions: {
    async login(email: string, username: string, password: string) {
      await apiCreateUser(email, username, password)


      this.$patch({
        email: email,
        username: username,
        isLoggedIn: true,
      })
    },

    logout() {
      this.$reset()

      // we could do other stuff like redirecting the user
    },
  },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
