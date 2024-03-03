<script setup lang="ts">
import { onBeforeMount } from 'vue';
import { RouterView, useRoute, useRouter } from 'vue-router'

import { useUserStore } from '@/modules/user/user.store';
import { routes } from '@/shared/routes';

const { checkUser } = useUserStore()
const router = useRouter();
const { meta } = useRoute()

onBeforeMount(async () => {
  const loggedIn = await checkUser()

  if (meta.requiresAuth && !loggedIn) router.push({ name: routes.LOGIN })
  if (meta.hideForLogged && loggedIn) router.push({ name: routes.DASHBOARD })
})
</script>

<template>
  <RouterView />
</template>
