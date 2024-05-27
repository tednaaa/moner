<script setup lang='ts'>
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import { object } from 'zod';

import { passwordValidation, requiredString } from '@/shared/validation';
import { routes } from '@/shared/routes';

import AuthLayout from '@/layouts/auth-layout.vue'
import OauthBox from '@/widgets/oauth-box.vue';

import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';

import BaseButton from '@/shared/ui/base-button/base-button.vue';
import { useUserStore } from '@/entities/user/user.store';

const userStore = useUserStore()

const validationSchema = toTypedSchema(object({
  login: requiredString,
  password: passwordValidation,
}));

const { handleSubmit } = useForm({
  validationSchema,
});

const handleLogin = handleSubmit((userCredentials) => {
  userStore.loginUser(userCredentials)
});

</script>

<template>
  <AuthLayout
    status-text="I see you"
    @submit="handleLogin"
  >
    <h1 :class="$style.title">
      Log in via
    </h1>
    <OauthBox />
    <span :class="$style.divider">or</span>

    <div :class="$style.inputGroup">
      <BaseInput
        name="login"
        label="Login"
        type="text"
      />
      <BasePassword
        name="password"
        label="Password"
      />
    </div>
    <BaseButton :class="$style.button">
      Log In
    </BaseButton>
    <BaseLink
      :class="$style.forgotPasswordLink"
      :to="{ name: routes.RECOVER }"
    >
      Forgot password?
    </BaseLink>

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
