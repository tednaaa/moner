<script setup lang="ts">
import Password from "primevue/password";
import { useField } from "vee-validate";

const props = defineProps<{
  name: string;
  label: string;
  feedback?: boolean;
}>();

const { value, errorMessage } = useField(props.name, undefined, { validateOnValueUpdate: false });
</script>

<template>
  <label>
    <span class="text-sm text-muted-foreground inline-block ml-1 mb-1">{{ props.label }}</span>
    <Password v-model="value" :name="props.name" :feedback="props.feedback" />
    <span v-if="errorMessage" class="text-xs font-medium text-red-500 ml-2">{{ errorMessage }}</span>
  </label>
</template>
