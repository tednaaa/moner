<script setup lang="ts">
import { computed, ref } from "vue";
import { RouterLink, useRouter } from "vue-router";
import InputText from "primevue/inputtext";
import Menu from "primevue/menu";
import Avatar from "primevue/avatar";

import { routes } from "@/shared/routes";
import { useUserStore } from "@/entities/user/user.store";
import { AUTH_SESSION_REFERRER_KEY } from "@/shared/config";
import mockImage from "@/assets/mock.jpg";
import type { MenuItem } from "primevue/menuitem";

const userStore = useUserStore();
const router = useRouter();

const menuItems = computed<MenuItem[]>(() => [
  {
    label: "Profile",
    items: [
      {
        label: "Sign Out",
        icon: "pi pi-sign-out",
        command: () => userStore.logoutUser(),
      },
    ],
  },
]);

const menu = ref();
const toggle = (event: MouseEvent) => menu.value.toggle(event);

function redirectToLogin() {
  sessionStorage.setItem(AUTH_SESSION_REFERRER_KEY, router.currentRoute.value.fullPath);
  router.push({ name: routes.LOGIN });
}
</script>

<template>
  <header class="container flex items-center justify-between bg-background">
    <RouterLink class="text-3xl mt-0.5 font-bold text-white" :to="{ name: routes.ROOT }">moner</RouterLink>

    <div class="flex items-center gap-3">
      <div class="relative h-10">
        <i class="pi pi-search text-xl text-muted-foreground absolute left-3 top-[50%] translate-y-[-50%]"></i>
        <InputText class="pl-12 w-[24rem] h-full" type="text" placeholder="Search..." />
      </div>

      <template v-if="userStore.isLoggedIn">
        <button class="flex" aria-haspopup="true" @click="toggle">
          <Avatar size="large" :image="mockImage" />
        </button>
        <Menu ref="menu" :model="menuItems" :popup="true" />
      </template>
      <button v-else :class="$style.loginButton" @click="redirectToLogin">Log In</button>
    </div>
  </header>
</template>

<style module lang="scss">
.loginButton {
  background-color: #9747ff1f;
  color: #9747ff;
  padding: 10px 40px;
  border-radius: 10px;
  transition: 0.3s;

  &:hover {
    background-color: #9747ff3f;
  }
}
</style>
