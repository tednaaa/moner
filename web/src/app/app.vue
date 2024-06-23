<script setup lang="ts">
import { RouterView } from "vue-router";
import Toast from "primevue/toast";
import { useColorMode } from "@vueuse/core";

import { useUserStore } from "@/entities/user/user.store";

useColorMode();
const userStore = useUserStore();

userStore.getCurrentUser();
</script>

<template>
  <Toast />
  <RouterView v-if="!userStore.isLoading" />
</template>
