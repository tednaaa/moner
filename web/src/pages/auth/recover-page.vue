<script setup lang='ts'>
import { computed, ref } from 'vue';
import { useField, useForm } from 'vee-validate';
import { toTypedSchema } from '@vee-validate/zod';
import { object, string } from 'zod';

import InputOtp from 'primevue/inputotp';

import { emailValidation } from '@/shared/validation';

import AuthLayout from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseButton from '@/shared/ui/base-button/base-button.vue';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';
import ArrowGoBackIcon from '@/shared/icons/arrow-go-back-icon.vue'
import { useUserStore } from '@/entities/user/user.store';
import BasePassword from '@/shared/ui/base-password/base-password.vue';

const userStore = useUserStore()

enum FormStep {
  PrepareEmail = 0,
  VerifyIdentity = 1,
  ChangePassword = 2,
}

const schemas = [
  toTypedSchema(object({
    email: emailValidation,
  })),
  toTypedSchema(object({
    verificationCode: string().min(6).max(6),
  })),
  toTypedSchema(object({
    newPassword: string().min(6).max(20),
    repeatPassword: string().min(6).max(20)
  }).refine(({ newPassword, repeatPassword }) => newPassword === repeatPassword, {
    message: 'Passwords do not match',
    path: ['repeatPassword'],
  }))
]

const currentStep = ref<FormStep>(FormStep.PrepareEmail)
const currentSchema = computed(() => schemas[currentStep.value])

const { handleSubmit, values: form } = useForm<{
  email: string,
  verificationCode: string,
  newPassword: string
}>({ validationSchema: currentSchema, keepValuesOnUnmount: true, validateOnMount: false })
const { value: verificationCode, errorMessage: verificationError } = useField<string>("verificationCode")

const onSubmit = handleSubmit(async () => {
  if (currentStep.value > FormStep.ChangePassword) return

  if (currentStep.value === FormStep.PrepareEmail) {
    const isOk = await userStore.resetPassword(form.email)
    if (!isOk) return
  }

  if (currentStep.value === FormStep.VerifyIdentity) {
    const isOk = await userStore.verifyPassswordReset(form.email, form.verificationCode)
    if (!isOk) return
  }

  if (currentStep.value === FormStep.ChangePassword) {
    userStore.changePassword(form.newPassword)
    return
  }

  currentStep.value++
})
</script>

<template>
  <AuthLayout
    status-text="I see you"
    @submit="onSubmit"
  >
    <template v-if="FormStep.PrepareEmail === currentStep">
      <h1 :class="$style.title">
        Reset Password
      </h1>
      <p :class="$style.description">
        Enter the email address you used to register and we'll send you the instruction
      </p>

      <BaseInput
        :class="$style.input"
        name="email"
        label="Email"
        type="email"
      />
      <BaseButton
        :class="$style.actionButton"
        type="submit"
      >
        Reset Password
      </BaseButton>
      <BaseLink
        :class="$style.goBackLink"
        :to="{ name: routes.LOGIN }"
      >
        <ArrowGoBackIcon :class="$style.goBackIcon" />
        Back to Log In
      </BaseLink>
    </template>
    <template v-if="FormStep.VerifyIdentity === currentStep">
      <h1 :class="$style.title">
        Verify your identity
      </h1>
      <p :class="$style.description">
        Enter the code we just sent you
      </p>

      <InputOtp
        v-model="verificationCode"
        :length="6"
        integer-only
        :invalid="Boolean(verificationError)"
      />
      <BaseButton
        :class="$style.actionButton"
        type="submit"
      >
        Verify
      </BaseButton>
      <BaseLink
        :class="$style.goBackLink"
        :to="{ name: routes.LOGIN }"
      >
        <ArrowGoBackIcon :class="$style.goBackIcon" />
        Back to Log In
      </BaseLink>
    </template>
    <template v-if="FormStep.ChangePassword === currentStep">
      <h1 :class="$style.title">
        Change Password
      </h1>
      <p :class="$style.description">
        Create a new, strong password that you don't use for other websites
      </p>

      <div :class="$style.passwords">
        <BasePassword
          :class="$style.input"
          name="newPassword"
          label="Password"
        />
        <BasePassword
          :class="$style.input"
          name="repeatPassword"
          label="Repeat password"
        />
      </div>

      <BaseButton
        :class="$style.actionButton"
        type="submit"
      >
        Change your password
      </BaseButton>
      <BaseLink
        :class="$style.goBackLink"
        :to="{ name: routes.LOGIN }"
      >
        <ArrowGoBackIcon :class="$style.goBackIcon" />
        Back to Log In
      </BaseLink>
    </template>
  </AuthLayout>
</template>

<style module lang='scss'>
.title {
  font-size: 32px;
  line-height: 39px;
  font-weight: 700;
  margin-bottom: 25px;
}

.description {
  max-width: 320px;
  line-height: 24px;
  margin-bottom: 40px;
  text-align: center;
}

.passwords {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.input {
  width: 100%;
}

.actionButton {
  width: 100%;
  margin: 35px 0 25px;
}

.goBackLink {
  display: flex;
  align-items: center;
  gap: 10px;
}
</style>
