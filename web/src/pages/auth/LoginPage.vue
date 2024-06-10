<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { object, string } from "zod";
import Button from "primevue/button";

import { routes } from "@/shared/routes";

import AuthLayout from "@/layouts/auth-layout.vue";
import OAuthBox from "@/widgets/OAuthBox.vue";

import { useUserStore } from "@/entities/user/user.store";
import { RouterLink } from "vue-router";
import { FormInputText, FormPassword } from "@/shared/ui/form";

const userStore = useUserStore();

const formSchema = object({
  login: string().min(6, { message: "Must be valid email or username" }).describe("Email or username"),
  password: string().min(6, { message: "Must be at least 6 characters" }),
});
const { handleSubmit } = useForm({ validationSchema: toTypedSchema(formSchema) });

const onSubmit = handleSubmit((credentials) => userStore.loginUser(credentials));
</script>

<template>
  <AuthLayout status-text="I see you">
    <h1 class="font-bold text-3xl mb-6 text-center">Log In</h1>
    <OAuthBox class="mx-auto mb-4" />
    <span :class="$style.divider">or</span>

    <form class="mt-10" @submit="onSubmit">
      <div class="flex flex-col gap-2 mb-4">
        <FormInputText name="login" label="Email or username" />
        <FormPassword name="password" label="Password" />
      </div>

      <Button class="w-full" variant="default" type="submit">Log In</Button>
    </form>

    <RouterLink class="block text-right mt-2 mb-4 font-medium text-primary" :to="{ name: routes.RECOVER }">
      Forgot password?
    </RouterLink>
    <span class="flex gap-1 font-medium">
      Need to create an account?
      <RouterLink class="text-primary" :to="{ name: routes.REGISTER }">Register</RouterLink>
    </span>
  </AuthLayout>
</template>

<style module lang="scss">
.divider {
  width: 100%;
  text-align: center;
  position: relative;

  &::before,
  &::after {
    content: "";
    position: absolute;
    background-color: #898989;
    height: 1px;
    width: 40%;
    top: 50%;
    transform: translateY(-50%);
  }

  &::before {
    left: 0;
  }

  &::after {
    right: 0;
  }
}
</style>
