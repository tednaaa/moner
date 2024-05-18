import { createFetch } from '@vueuse/core';
import { API_URL } from '../config';
import { useUserStore } from '@/entities/user/user.store';

export const useApiFetch = createFetch({
  baseUrl: API_URL,
  fetchOptions: {
    mode: 'cors',
    credentials: 'include',
  },
  options: {
    async onFetchError(ctx) {
      if (ctx.response?.status === 401) {
        const { logoutUser } = useUserStore()
        await logoutUser()
      }

      return ctx
    }
  }
})
