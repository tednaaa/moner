import { defineStore, acceptHMRUpdate } from 'pinia'

import { apiLoginUser, apiRegisterUser, apiGetUser, apiLogoutUser } from '@/shared/api/user/user.api'
import { useToast } from 'vue-toastification'
import { ref } from 'vue'

import type { ApiLoginUserDto, ApiRegisterUserDto, ApiUserResponse } from '@/shared/api/user/user.types'
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

  function setCurrentUser({ email, username }: ApiUserResponse) {
    user.value = { email, username }
    isLoggedIn.value = true
    router.push({ name: routes.DASHBOARD })
  }

  function registerUser(registerUserDto: ApiRegisterUserDto) {
    apiRegisterUser(registerUserDto)
      .then(({ data: user }) => setCurrentUser(user))
      .catch(error => toast.error(error.response.data, { timeout: 5000 }))
  }

  function loginUser(loginUserDto: ApiLoginUserDto) {
    apiLoginUser(loginUserDto)
      .then(({ data: user }) => setCurrentUser(user))
      .catch(error => toast.error(error.response.data, { timeout: 5000 }))
  }

  function getUser() {
    apiGetUser()
      .then(({ data: user }) => setCurrentUser(user))
  }

  async function logoutUser() {
    await apiLogoutUser()

    user.value = getUserDefaultState()
    isLoggedIn.value = false
    router.push({ name: routes.LOGIN })
  }

  return { user, isLoggedIn, registerUser, loginUser, getUser, logoutUser }
})

apiInstance.interceptors.response.use(response => {
  const store = useUserStore()

  if (response.status === 401) store.logoutUser()

  return response
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
