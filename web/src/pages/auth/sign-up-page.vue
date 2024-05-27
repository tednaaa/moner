<script setup lang='ts'>
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import { object } from 'zod';
import { toTypedSchema } from '@vee-validate/zod';
import InputOtp from 'primevue/inputotp';

import { emailValidation, usernameValidation, passwordValidation } from '@/shared/validation';

import AuthLayout from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';
import { useUserStore } from '@/entities/user/user.store'

import BaseButton from '@/shared/ui/base-button/base-button.vue';
import OauthBox from '@/widgets/oauth-box.vue';

const isRegistered = ref(false)
const verificationCode = ref("")

const userStore = useUserStore()

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(object({
    email: emailValidation,
    username: usernameValidation,
    password: passwordValidation,
  })),
});

const registerUser = handleSubmit(async (userCredentials) => {
  isRegistered.value = await userStore.registerUser(userCredentials)
});

function verify() {
  if (verificationCode.value.length !== 6) return;

  userStore.verifyUser(verificationCode.value)
}
</script>

<template>
  <AuthLayout
    status-text="I see you"
    @submit="registerUser"
  >
    <template v-if="isRegistered">
      <h1 :class="$style.title">
        Verify your account
      </h1>
      <span :class="$style.verificationDescription">
        We sent to an email
        <span :class="$style.verificationEmail">{{ userStore.currentUser.email }}</span> <br>
        verification code, please enter it below
      </span>

      <InputOtp
        v-model="verificationCode"
        :class="$style.verificationInput"
        :length="6"
        integer-only
      />
      <BaseButton
        :class="$style.button"
        @click.prevent="verify"
      >
        Verify
      </BaseButton>

      <div :class="$style.resendContainer">
        <span>Didn't receive the code?</span>
        <button
          :class="$style.resendButton"
          @click.prevent="userStore.resendVerification"
        >
          Resend
        </button>
      </div>
    </template>
    <template v-else>
      <h1 :class="$style.title">
        Sign Up via
      </h1>
      <OauthBox />
      <span :class="$style.divider">or</span>

      <div :class="$style.inputGroup">
        <BaseInput
          name="email"
          label="Email"
          type="email"
        />
        <BaseInput
          name="username"
          label="Username"
          type="text"
        />
        <BasePassword
          name="password"
          label="Password"
        />
      </div>
      <BaseButton :class="$style.button">
        Register
      </BaseButton>

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
  margin-bottom: 40px;
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
