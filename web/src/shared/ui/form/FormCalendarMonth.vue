<script setup lang="ts">
import Calendar from "primevue/calendar";
import { useField } from "vee-validate";

const props = defineProps<{
  name: string;
  label: string;
  disabled?: boolean;
}>();

const { value, errorMessage } = useField(props.name, undefined, { validateOnValueUpdate: false });
</script>

<template>
  <div class="inline-flex flex-col">
    <label class="text-sm text-muted-foreground inline-block ml-1 mb-1" :for="props.name">{{ props.label }}</label>
    <Calendar
      v-model="value"
      :input-id="props.name"
      :disabled="props.disabled"
      :manual-input="false"
      view="month"
      date-format="yy MM"
      show-icon
    />
    <span v-if="errorMessage" class="text-xs font-medium text-red-500 ml-2">{{ errorMessage }}</span>
  </div>
</template>
