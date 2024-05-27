import { defineStore, acceptHMRUpdate } from 'pinia'

import { useToast } from 'primevue/usetoast'

import { useApiFetch } from '@/shared/api/api'

export const useFollowsStore = defineStore('follows', () => {
  const toast = useToast()

  function followUser(followedId: number): Promise<boolean> {
    return new Promise(resolve => {
      const { onFetchError, onFetchResponse } = useApiFetch("/follow").post({ followedId })

      onFetchError(() => {
        toast.add({ severity: 'error', summary: 'Failed to follow', life: 5000 })
        resolve(false)
      })

      onFetchResponse(() => resolve(true))
    })
  }

  function unfollowUser(unfollowedId: number): Promise<boolean> {
    return new Promise(resolve => {
      const { onFetchError, onFetchResponse } = useApiFetch(`/unfollow`).post({ unfollowedId })

      onFetchError(() => {
        toast.add({ severity: 'error', summary: 'Failed to unfollow', life: 5000 })
        resolve(false)
      })

      onFetchResponse(() => resolve(true))
    })
  }

  return { followUser, unfollowUser }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useFollowsStore, import.meta.hot))
}
