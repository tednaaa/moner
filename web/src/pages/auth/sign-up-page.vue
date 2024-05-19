<script setup lang='ts'>
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import { object } from 'zod';
import { toTypedSchema } from '@vee-validate/zod';
import InputOtp from 'primevue/inputotp';

import { emailValidation, usernameValidation, passwordValidation } from '@/shared/validation';

import AuthLayout, { type AuthLayoutStatus } from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';
import { useUserStore } from '@/entities/user/user.store'

import BaseButton from '@/shared/ui/base-button/base-button.vue';
import OauthBox from '@/widgets/oauth-box.vue';

const status = ref<AuthLayoutStatus>('default')
const isRegistered = ref(false)
const verificationCode = ref("")

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(
    object({
      email: emailValidation,
      username: usernameValidation,
      password: passwordValidation,
    }),
  ),
});

const userStore = useUserStore()
const onSubmit = handleSubmit(async (userCredentials) => {
  isRegistered.value = await userStore.registerUser(userCredentials)
});
</script>

<template>
  <AuthLayout @submit="onSubmit" :status="status" status-text="I see you">
    <template v-if="isRegistered">
      <h1 :class="$style.title">Verify your account</h1>
      <span :class="$style.verificationDescription">
        We sent to an email
        <span :class="$style.verificationEmail">{{ userStore.currentUser.email }}</span> <br>
        verification code, please enter it below
      </span>

      <InputOtp :class="$style.verificationInput" v-model="verificationCode" :length="6" integer-only />
      <BaseButton @click.prevent="userStore.verifyUser(verificationCode)" :class="$style.button">
        Verify
      </BaseButton>

      <div :class="$style.resendContainer">
        <span>Didn't receive the code?</span>
        <button @click.prevent="userStore.resendVerification" :class="$style.resendButton">Resend</button>
      </div>
    </template>
    <template v-else>
      <h1 :class="$style.title">Sign Up via</h1>
      <OauthBox />
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
    </template>
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
}

.signUpText {
  display: flex;
  gap: 5px;
  font-weight: 500;
  margin-top: 20px;
}

.verificationDescription {
  font-size: 13px;
  font-weight: 500;
  line-height: 17px;
  text-align: center;
  margin-bottom: 30px;
}

.verificationEmail {
  color: #478FE8;
}

.verificationInput {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 40px;

  input[data-pc-name="input"] {
    color: #334155;
    background: white;
    padding: 0.5rem 0.75rem;
    border: 1px solid #cbd5e1;
    appearance: none;
    border-radius: 6px;
    outline-color: transparent;
    text-align: center;
    box-shadow: 0 0 #0000, 0 0 #0000, 0 1px 2px 0 rgba(18, 18, 23, 0.05);
    width: 2.5rem;
    transition: background-color 0.2s, color 0.2s, border-color 0.2s, box-shadow 0.2s, outline-color 0.2s;
  }
}

.resendContainer {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  margin-top: 20px;
}

.resendButton {
  background-color: #478fe813;
  color: #478FE8;
  padding: 6px 12px;
  border-radius: 16px;
  transition: 0.2s;

  &:hover {
    background-color: #54a0f634;
  }
}
</style>
