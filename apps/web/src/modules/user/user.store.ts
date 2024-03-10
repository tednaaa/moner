import { defineStore, acceptHMRUpdate } from 'pinia'

import { apiAuthorizeUser, apiCreateUser, apiGetUser, apiLogoutUser } from '@/shared/api/user/user.api'
import { useToast } from 'vue-toastification'
import { ref } from 'vue'

import type { ApiAuthorizeUserDto, ApiCreateUserDto, ApiUserResponse } from '@/shared/api/user/user.types'
import { apiInstance } from '@/shared/api/instance'
import { router } from '@/pages/router'
import { routes } from '@/shared/routes'

const toast = useToast()
const getUserDefaultState = () => ({
  email: '',
  username: ''
})
export const useUserStore = defineStore('user', () => {
  const user = ref(getUserDefaultState())
  const isLoggedIn = ref(false)

  function setAuthUser({ email, username }: ApiUserResponse) {
    user.value = { email, username }
    isLoggedIn.value = true
    router.push({ name: routes.DASHBOARD })
  }

  async function createUser(createUserDto: ApiCreateUserDto) {
    const { data } = await apiCreateUser(createUserDto)

    if ('error' in data) {
      toast.error(data.error, { timeout: 5000 });
      return
    }

    setAuthUser(data)
  }

  async function authorizeUser(authorizeUserDto: ApiAuthorizeUserDto) {
    const { data } = await apiAuthorizeUser(authorizeUserDto)

    if ('error' in data) {
      toast.error(data.error, { timeout: 5000 })
      return
    }

    setAuthUser(data)
  }
  async function checkUser() {
    const { data } = await apiGetUser()

    if ('error' in data) {
      isLoggedIn.value = false
      return false
    }

    setAuthUser(data)
    return true
  }

  async function logoutUser() {
    await apiLogoutUser()

    user.value = getUserDefaultState()
    isLoggedIn.value = false
    router.push({ name: routes.LOGIN })
  }

  return { user, isLoggedIn, createUser, authorizeUser, checkUser, logoutUser }
})

apiInstance.interceptors.response.use(response => {
  const store = useUserStore()

  if (response.status === 401) store.logoutUser()

  return response
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
