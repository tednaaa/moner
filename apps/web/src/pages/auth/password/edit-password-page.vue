<script setup lang='ts'>
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import { toTypedSchema } from '@vee-validate/zod';
import { object } from 'zod';

import { passwordValidation } from '@/shared/validation';

import AuthLayout, { type AuthLayoutStatus } from '@/layouts/auth-layout.vue'
import BasePassword from '@/shared/ui/base-password/base-password.vue';
import BaseButton from '@/shared/ui/base-button/base-button.vue';

const status = ref<AuthLayoutStatus>('default')

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(
    object({
      newPassword: passwordValidation,
      repeatPassword: passwordValidation,
    }),
  ),
});

const onSubmit = handleSubmit(values => {
  console.log(values)
});
</script>

<template>
  <AuthLayout @submit="onSubmit" :status="status" status-text="I see you">
    <h1 :class="$style.title">Change Password</h1>
    <p :class="$style.description">Create a new, strong password that you don't use for other websites</p>

    <div :class="$style.inputGroup">
      <BasePassword :class="$style.input" name="newPassword" label="Password"></BasePassword>
      <BasePassword :class="$style.input" name="repeatPassword" label="Repeat password"></BasePassword>

      <BaseButton>Change your password</BaseButton>
    </div>
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
  max-width: 250px;
  line-height: 24px;
  margin-bottom: 30px;
  text-align: center;
}

.inputGroup {
  margin-bottom: 20px;
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 20px;
}

.input {
  width: 100%;
}
</style>
