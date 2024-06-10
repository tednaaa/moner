import { createRouter, createWebHistory } from "vue-router";
import { routes } from "@/shared/routes";

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: routes.ROOT,
      redirect: { name: routes.LOGIN },
    },
    {
      path: "/:username/:projectId",
      name: routes.PROJECT,
      component: () => import("./project/ProjectPage.vue"),
    },
    {
      path: "/:username",
      name: routes.PROFILE,
      component: () => import("./profile/ProfilePage.vue"),
    },
    {
      path: "/auth",
      children: [
        {
          path: "login",
          name: routes.LOGIN,
          component: () => import("./auth/LoginPage.vue"),
        },
        {
          path: "register",
          name: routes.REGISTER,
          component: () => import("./auth/RegisterPage.vue"),
        },
        {
          path: "recover",
          name: routes.RECOVER,
          component: () => import("./auth/RecoverPage.vue"),
        },
      ],
    },
  ],
});
