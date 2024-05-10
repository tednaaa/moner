<script setup lang='ts'>
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import { toTypedSchema } from '@vee-validate/zod';
import { object } from 'zod';

import { passwordValidation } from '@/shared/validation';

import AuthLayout, { type AuthLayoutStatus } from '@/layouts/auth-layout.vue'
import { routes } from '@/shared/routes';
import BaseButton from '@/shared/ui/base-button/base-button.vue';
import BaseInput from '@/shared/ui/base-input/base-input.vue';
import BaseLink from '@/shared/ui/base-link/base-link.vue';
import ArrowGoBackIcon from '@/shared/icons/arrow-go-back-icon.vue'

const status = ref<AuthLayoutStatus>('default')

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(
    object({
      email: passwordValidation,
    }),
  ),
});

const onSubmit = handleSubmit(values => {
  console.log(values)
});
</script>

<template>
  <AuthLayout @submit="onSubmit" :status="status" status-text="I see you">
    <h1 :class="$style.title">Recover Password</h1>
    <p :class="$style.description">Enter the email address you used to register and we'll send you the instruction</p>

    <BaseInput :class="$style.input" name="email" label="Email" type="email"></BaseInput>
    <BaseButton :class="$style.resetButton" type="submit">Reset Password</BaseButton>
    <BaseLink :class="$style.goBackLink" :to="{ name: routes.LOGIN }">
      <ArrowGoBackIcon :class="$style.goBackIcon"></ArrowGoBackIcon>
      Back to Log In
    </BaseLink>
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
  margin-bottom: 30px;
  text-align: center;
}

.input {
  width: 100%;
}

.resetButton {
  width: 100%;
  margin: 35px 0 25px;
}

.goBackLink {
  display: flex;
  align-items: center;
  gap: 10px;
}
</style>
