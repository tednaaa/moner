<script setup lang="ts">
import { ref, watch } from "vue";
import { RouterLink } from "vue-router";
import { useForm } from "vee-validate";
import { object, string } from "zod";
import { toTypedSchema } from "@vee-validate/zod";
import InputOtp from "primevue/inputotp";
import Button from "primevue/button";

import OAuthBox from "@/widgets/OAuthBox.vue";

import AuthLayout from "@/layouts/auth-layout.vue";
import { routes } from "@/shared/routes";
import { useUserStore } from "@/entities/user/user.store";
import { FormInputText, FormPassword } from "@/shared/ui/form";

const formSchema = object({
  email: string().min(6).email().min(6),
  username: string().min(6),
  password: string().min(6),
});
const { handleSubmit } = useForm({ validationSchema: toTypedSchema(formSchema) });
const userStore = useUserStore();

const isRegistered = ref(false);
const onSubmit = handleSubmit(async (credentials) => {
  isRegistered.value = await userStore.registerUser(credentials);
});

const otp = ref("");
const isInvalidOtp = ref(false);
watch(otp, () => {
  isInvalidOtp.value = false;
  if (otp.value.length !== 6) return;

  const isOk = userStore.verifyUser(otp.value);
  if (!isOk) isInvalidOtp.value = true;
});
</script>

<template>
  <AuthLayout status-text="I see you">
    <template v-if="isRegistered">
      <h1 class="font-bold text-center text-3xl mb-6">Verify your account</h1>

      <div class="text-sm font-medium text-center mb-8">
        We sent to an email
        <span class="text-primary">{{ userStore.currentUser.email }}</span> <br />
        verification code, please enter it below
      </div>

      <InputOtp v-model="otp" class="justify-center" :length="6" integer-only :invalid="Boolean(isInvalidOtp)" />

      <div class="w-full flex items-center justify-center gap-2 mt-5">
        <span>Didn't receive the code?</span>

        <button
          class="bg-blue-600 bg-opacity-20 text-blue-600 px-4 py-2 rounded-3xl transition-colors hover:bg-blue-600 hover:bg-opacity-30"
          @click.prevent="userStore.resendVerification"
        >
          Resend
        </button>
      </div>
    </template>
    <template v-else>
      <h1 class="font-bold text-3xl text-center mb-6">Register</h1>
      <OAuthBox class="mx-auto mb-4" />
      <span :class="$style.divider">or</span>

      <form class="mt-5" @submit="onSubmit">
        <div class="flex flex-col gap-2 mb-4">
          <FormInputText name="email" label="Email" />
          <FormInputText name="username" label="Username" />
          <FormPassword name="password" label="Password" />
        </div>

        <Button class="w-full" type="submit">Register</Button>
      </form>

      <span class="flex justify-center gap-1 font-medium mt-5">
        Already have an account?

        <RouterLink class="text-primary" :to="{ name: routes.LOGIN }">Log In</RouterLink>
      </span>
    </template>
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
