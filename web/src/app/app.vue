<script setup lang="ts">
import { RouterView } from 'vue-router'
import Toast from 'primevue/toast';

import { useUserStore } from '@/entities/user/user.store';

const userStore = useUserStore()

userStore.getCurrentUser()
</script>

<template>
  <Toast />
  <RouterView v-if="!userStore.isLoading" />
</template>
