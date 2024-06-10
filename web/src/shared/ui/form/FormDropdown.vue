<script setup lang="ts">
import { useField } from "vee-validate";
import Dropdown from "primevue/dropdown";

const props = defineProps<{
  name: string;
  label: string;

  optionLabel?: string;
  options: string[] | { label: string; value: string }[];
}>();

const { value } = useField(props.name, undefined, { validateOnValueUpdate: false });
</script>

<template>
  <div class="flex flex-col gap-1">
    <span class="text-sm text-muted-foreground inline-block ml-1">{{ props.label }}</span>
    <Dropdown
      v-model="value"
      class="w-full md:w-[14rem]"
      placeholder="Please select"
      :options="options"
      :option-label="optionLabel"
    />
  </div>
</template>
