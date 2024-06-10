<script setup lang="ts">
import InputText from "primevue/inputtext";
import { useField } from "vee-validate";

const props = defineProps<{
  name: string;
  label: string;
}>();

const { value, errorMessage } = useField(props.name, undefined, { validateOnValueUpdate: false });
</script>

<template>
  <label class="flex flex-col gap-1">
    <span class="text-sm text-muted-foreground inline-block ml-1">{{ props.label }}</span>
    <InputText v-model="value" :name="props.name" />
    <span v-if="errorMessage" class="text-xs font-medium text-red-500 ml-2">{{ errorMessage }}</span>
  </label>
</template>
