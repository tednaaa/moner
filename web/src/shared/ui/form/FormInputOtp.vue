<script setup lang="ts">
import InputOtp from "primevue/inputotp";
import { useField } from "vee-validate";
import { watch } from "vue";

const props = defineProps<{
  name: string;
}>();

const emit = defineEmits<{
  complete: [string];
}>();

const { value: otp, errorMessage } = useField<string>(props.name, undefined, { validateOnValueUpdate: false });

watch(otp, () => {
  if (otp.value.length === 6) emit("complete", otp.value);
});
</script>

<template>
  <InputOtp v-model="otp" :length="6" integer-only :invalid="Boolean(errorMessage)" />
</template>
