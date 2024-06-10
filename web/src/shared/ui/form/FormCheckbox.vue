<script setup lang="ts">
import Checkbox from "primevue/checkbox";
import { useField } from "vee-validate";

const props = defineProps<{
  name: string;
  label: string;
}>();

const { value, errorMessage } = useField(props.name, undefined, { validateOnValueUpdate: false });
</script>

<template>
  <div>
    <Checkbox v-model="value" :input-id="props.name" :name="props.name" binary />
    <label class="text-sm text-muted-foreground inline-block ml-1" :for="props.name">{{ props.label }}</label>
    <span v-if="errorMessage" class="text-xs font-medium text-red-500 ml-2">{{ errorMessage }}</span>
  </div>
</template>
