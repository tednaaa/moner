<script setup lang="ts">
import { computed, ref } from "vue";
import { RouterLink } from "vue-router";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { object, string } from "zod";
import Button from "primevue/button";

import { FormInputOtp, FormInputText, FormPassword } from "@/shared/ui/form";
import AuthLayout from "@/layouts/auth-layout.vue";
import { routes } from "@/shared/routes";
import ArrowGoBackIcon from "@/shared/icons/arrow-go-back-icon.vue";
import { useUserStore } from "@/entities/user/user.store";

const userStore = useUserStore();

enum FormStep {
  PrepareEmail = 0,
  VerifyIdentity = 1,
  ChangePassword = 2,
}

const formSchemas = [
  toTypedSchema(object({ email: string().email().min(6) })),
  toTypedSchema(object({ verificationCode: string().min(6).max(6) })),
  toTypedSchema(
    object({
      newPassword: string().min(6),
      repeatPassword: string().min(6),
    }).refine(({ newPassword, repeatPassword }) => newPassword === repeatPassword, {
      message: "Passwords do not match",
      path: ["repeatPassword"],
    }),
  ),
];
const currentStep = ref<FormStep>(FormStep.PrepareEmail);
const currentSchema = computed(() => formSchemas[currentStep.value]);

const {
  handleSubmit,
  values: form,
  setFieldError,
} = useForm<{
  email: string;
  verificationCode: string;
  newPassword: string;
}>({ validationSchema: currentSchema, keepValuesOnUnmount: true });

const onSubmit = handleSubmit(async () => {
  if (currentStep.value > FormStep.ChangePassword) return;

  if (currentStep.value === FormStep.PrepareEmail) {
    const isOk = await userStore.resetPassword(form.email);
    if (!isOk) return;
  }

  if (currentStep.value === FormStep.ChangePassword) {
    userStore.changePassword(form.newPassword);
    return;
  }

  currentStep.value++;
});

async function verifyIdentity(otp: string) {
  const isOk = await userStore.verifyPassswordReset(form.email, otp);
  if (isOk) {
    currentStep.value++;
  } else setFieldError("verificationCode", "Invalid code");
}
</script>

<template>
  <AuthLayout status-text="I see you">
    <form @submit="onSubmit">
      <template v-if="FormStep.PrepareEmail === currentStep">
        <h1 class="text-3xl text-center font-bold mb-4">Reset Password</h1>
        <p class="text-sm font-medium max-w-[19rem] mb-6 text-center mx-auto">
          Enter the email address you used to register and we will send you the instruction
        </p>

        <div class="w-full flex flex-col gap-4">
          <FormInputText name="email" label="Email" />
          <Button class="w-full" type="submit">Reset</Button>
        </div>
      </template>
      <template v-if="FormStep.VerifyIdentity === currentStep">
        <h1 class="text-3xl text-center font-bold mb-4">Verify your identity</h1>
        <div class="text-sm font-medium text-center mb-6">
          We sent to an email
          <span class="text-primary">{{ userStore.currentUser.email }}</span> <br />
          verification code, please enter it below
        </div>

        <FormInputOtp class="justify-center" name="verificationCode" @complete="verifyIdentity" />
      </template>
      <template v-if="FormStep.ChangePassword === currentStep">
        <h1 class="text-3xl text-center font-bold mb-4">Change Password</h1>
        <p class="text-sm max-w-64 mx-auto text-center mb-6">
          Create a new, strong password that you don't use for other websites
        </p>

        <div class="w-full flex flex-col gap-4 mb-4">
          <FormPassword name="newPassword" label="New password" />
          <FormPassword name="repeatPassword" label="Repeat password" :feedback="false" />
        </div>
        <Button class="w-full" type="submit">Change</Button>
      </template>
      <RouterLink class="flex justify-center items-center text-primary gap-2 mt-6" :to="{ name: routes.LOGIN }">
        <ArrowGoBackIcon />
        <span>Back to Log In</span>
      </RouterLink>
    </form>
  </AuthLayout>
</template>
