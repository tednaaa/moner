<script setup lang='ts'>
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import { object } from 'zod';
import { toTypedSchema } from '@vee-validate/zod';

import { emailValidation, usernameValidation, passwordValidation } from '@/shared/validation';

import AuthLayout, { type AuthLayoutStatus } from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';
import { useUserStore } from '@/entities/user/user.store'

import googleIcon from '@/shared/icons/google-icon.vue'
import gitlabIcon from '@/shared/icons/gitlab-icon.vue'
import githubIcon from '@/shared/icons/github-icon.vue'
import BaseButton from '@/shared/ui/base-button/base-button.vue';

const status = ref<AuthLayoutStatus>('default')

const oAuthList = [
  { iconComponent: googleIcon },
  { iconComponent: gitlabIcon },
  { iconComponent: githubIcon }
]

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(
    object({
      email: emailValidation,
      username: usernameValidation,
      password: passwordValidation,
    }),
  ),
});

const { registerUser } = useUserStore()
const onSubmit = handleSubmit((userCredentials) => {
  registerUser(userCredentials)
});
</script>

<template>
  <AuthLayout @submit="onSubmit" :status="status" status-text="I see you">
    <h1 :class="$style.title">Sign Up via</h1>
    <ul :class="$style.OAuthList">
      <li :class="$style.OAuthItem" v-for="oAuth, index in oAuthList" :key="index">
        <button :class="$style.OAuthButton">
          <component :is="oAuth.iconComponent"></component>
        </button>
      </li>
    </ul>
    <span :class="$style.divider">or</span>

    <div :class="$style.inputGroup">
      <BaseInput name="email" label="Email" type="email"></BaseInput>
      <BaseInput name="username" label="Username" type="text"></BaseInput>
      <BasePassword name="password" label="Password"></BasePassword>
    </div>
    <BaseButton :class="$style.button">Register</BaseButton>

    <span :class="$style.signUpText">
      Already have an account?

      <BaseLink :to="{ name: routes.LOGIN }">Log In</BaseLink>
    </span>
  </AuthLayout>
</template>

<style module lang='scss'>
.title {
  font-size: 32px;
  font-weight: 700;
  margin-bottom: 35px;
}

.OAuthList {
  display: flex;
  gap: 25px;
  margin-bottom: 35px;
}

.OAuthItem {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 5px;
  box-shadow: 1px 2px 5px 0px rgba(161, 161, 161, 0.25);
  background-color: white;
  transition: 0.3s;

  &:hover {
    background-color: var(--color-light-gray);
  }
}

.OAuthButton {
  padding: 12px;
}

.divider {
  width: 100%;
  text-align: center;
  position: relative;

  &::before,
  &::after {
    content: '';
    position: absolute;
    background-color: var(--color-gray);
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

.inputGroup {
  margin-top: 35px;
  margin-bottom: 20px;
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 20px;
}

.button {
  width: 100%;
  margin-bottom: 20px;
}

.signUpText {
  display: flex;
  gap: 5px;
  font-weight: 500;
}
</style>
