import { defineStore, acceptHMRUpdate } from 'pinia'

import { ref } from 'vue'
import { useToast } from 'primevue/usetoast'

import { useApiFetch } from '@/shared/api/api'
import { router } from '@/pages/router'
import { routes } from '@/shared/routes'
import type { LoginUserDto, RegisterUserDto, UserResponse } from './user.types'

const getUserInitialState = (): Partial<UserResponse> => ({})

export const useUserStore = defineStore('user', () => {
  const currentUser = ref(getUserInitialState())
  const isLoggedIn = ref(false)
  const isLoading = ref(true)
  const toast = useToast()

  function setLoggedUser(userResponse: UserResponse) {
    currentUser.value = userResponse
    isLoggedIn.value = true
  }

  function registerUser(registerUserDto: RegisterUserDto): Promise<boolean> {
    return new Promise(resolve => {
      const { onFetchError, onFetchResponse } = useApiFetch("/users/register").post(registerUserDto)

      onFetchError(() => {
        toast.add({ severity: 'error', summary: 'Failed to register', detail: 'User already exists', life: 5000 })
        resolve(false)
      })

      onFetchResponse(async (response) => {
        const { id, email }: UserResponse = await response.json()
        currentUser.value.id = id
        currentUser.value.email = email
        resolve(true)
      })
    })
  }

  function verifyUser(code: string) {
    const { onFetchError, onFetchResponse } = useApiFetch(`/users/verify`).patch({ userId: currentUser.value.id, code })

    onFetchError(() => toast.add({ severity: 'error', summary: 'Failed to verify', detail: 'Invalid verification code', life: 5000 }))
    onFetchResponse(async (response) => {
      const user: UserResponse = await response.json()
      setLoggedUser(user)
      await router.push({ name: routes.PROFILE, params: { username: user.username } })
    })
  }

  function resendVerification() {
    const { onFetchError, onFetchResponse } = useApiFetch(`/users/resend-verification`).post({ userId: currentUser.value.id })

    onFetchError(() => toast.add({ severity: 'error', summary: 'Failed to resend verification', life: 5000 }))
    onFetchResponse(() => toast.add({ severity: 'success', summary: 'Verification email sent', detail: 'Check your email', life: 5000 }))
  }

  function loginUser(loginUserDto: LoginUserDto) {
    const { onFetchError, onFetchResponse } = useApiFetch("/users/login").post(loginUserDto)

    onFetchError(() => toast.add({ severity: 'error', summary: 'Failed to login', detail: 'Wrong email or password', life: 5000 }))

    onFetchResponse(async (response) => {
      const user: UserResponse = await response.json()
      setLoggedUser(user)
      await router.push({ name: routes.PROFILE, params: { username: user.username } })
    })
  }

  function getCurrentUser() {
    const { onFetchResponse, onFetchFinally } = useApiFetch("/users/me").get()

    onFetchFinally(() => isLoading.value = false)
    onFetchResponse(async (response) => {
      const user: UserResponse = await response.json()
      setLoggedUser(user)
    })
  }

  async function logoutUser() {
    await useApiFetch("/users/logout").get()
    currentUser.value = getUserInitialState()
    isLoggedIn.value = false
    await router.push({ name: routes.LOGIN })
  }

  return { currentUser, isLoggedIn, isLoading, registerUser, verifyUser, resendVerification, loginUser, getCurrentUser, logoutUser }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
