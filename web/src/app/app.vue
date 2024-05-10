<script setup lang="ts">
import { onBeforeMount } from 'vue';
import { RouterView, useRoute, useRouter } from 'vue-router'

import { useUserStore } from '@/modules/user/user.store';
import { routes } from '@/shared/routes';

const { getUser, isLoggedIn } = useUserStore()
const router = useRouter();
const { meta } = useRoute()

onBeforeMount(() => {
  getUser()

  if (meta.requiresAuth && !isLoggedIn) router.push({ name: routes.LOGIN })
  if (meta.hideForLogged && isLoggedIn) router.push({ name: routes.DASHBOARD })
})
</script>

<template>
  <RouterView />
</template>
