<script setup lang='ts'>
import { ref } from 'vue';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import { object } from 'zod';

import { passwordValidation, requiredString } from '@/shared/validation';

import AuthLayout, { type AuthLayoutStatus } from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';

import googleIcon from '@/shared/icons/google-icon.vue'
import gitlabIcon from '@/shared/icons/gitlab-icon.vue'
import githubIcon from '@/shared/icons/github-icon.vue'
import BaseButton from '@/shared/ui/base-button/base-button.vue';
import { useUserStore } from '@/modules/user/user.store';
import { API_URL } from '@/shared/config';

const status = ref<AuthLayoutStatus>('default')

const oAuthList = [
  { provider: 'google', iconComponent: googleIcon },
  { provider: 'gitlab', iconComponent: gitlabIcon },
  { provider: 'github', iconComponent: githubIcon }
]

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(
    object({
      login: requiredString,
      password: passwordValidation,
    }),
  ),
});

const { loginUser } = useUserStore()
const onSubmit = handleSubmit((userCredentials) => {
  loginUser(userCredentials)
});
</script>

<template>
  <AuthLayout @submit="onSubmit" :status="status" status-text="I see you">
    <h1 :class="$style.title">Log in via</h1>
    <ul :class="$style.OAuthList">
      <li :class="$style.OAuthItem" v-for="oAuth, index in oAuthList" :key="index">
        <a :class="$style.OAuthLink" :href="`${API_URL}/oauth/${oAuth.provider}`">
          <component :is="oAuth.iconComponent"></component>
        </a>
      </li>
    </ul>
    <span :class="$style.divider">or</span>

    <div :class="$style.inputGroup">
      <BaseInput name="login" label="Login" type="text"></BaseInput>
      <BasePassword name="password" label="Password"></BasePassword>
    </div>
    <BaseButton :class="$style.button">Log In</BaseButton>
    <BaseLink :class="$style.forgotPasswordLink" :to="{ name: routes.RECOVER_PASSWORD }">Forgot password?</BaseLink>

    <span :class="$style.signUpText">
      Need to create an account?

      <BaseLink :to="{ name: routes.SIGN_UP }">Sign Up</BaseLink>
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

.OAuthLink {
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
  margin-bottom: 10px;
}

.forgotPasswordLink {
  margin-left: auto;
  margin-bottom: 20px;
}

.signUpText {
  display: flex;
  gap: 5px;
  font-weight: 500;
}
</style>
