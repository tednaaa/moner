<script setup lang="ts">
import { useForm } from "vee-validate";
import { z } from "zod";
import { toTypedSchema } from "@vee-validate/zod";

import Button from "primevue/button";
import { useConfirm } from "primevue/useconfirm";

import { FormCalendarMonth, FormCheckbox, FormDropdown, FormInputText } from "@/shared/ui/form";
import type { ExperienceDto } from "@/entities/experience/experience.types";
import { useExperienceStore } from "@/entities/experience/experience.store";
import ConfirmPopup from "primevue/confirmpopup";

const props = defineProps<{
  deleteable?: boolean;
  experience?: ExperienceDto;
}>();

const emit = defineEmits<{
  submit: [ExperienceDto];
  cancel: [];
  delete: [];
}>();

const employmentTypeOptions = ["full-time", "part-time", "contract", "freelance", "internship"];
const locationTypeOptions = ["on-site", "remote", "hybrid"];

const formSchema = z.object({
  companyName: z.string().min(1).max(255),
  occupation: z.string().min(1).max(255),
  locationName: z.string().max(255).optional(),
  locationType: z.string(),
  employmentType: z.string(),
  startDate: z.date(),
  endDate: z.date().optional(),
  isCurrent: z.boolean(),
  description: z.string().min(1),
});

const { values: form, handleSubmit } = useForm({
  validationSchema: toTypedSchema(formSchema),
  initialValues: props.experience,
});
const confirm = useConfirm();
const experienceStore = useExperienceStore();

const onSubmit = handleSubmit((experience) => emit("submit", experience));

function confirmDelete(event: MouseEvent) {
  if (!event.currentTarget) return;

  confirm.require({
    target: event.currentTarget as HTMLButtonElement,
    message: "Are you sure?",
    icon: "pi pi-info-circle",
    rejectLabel: "No",
    rejectClass: "p-button-secondary",
    acceptLabel: "Delete",
    acceptClass: "p-button-danger",
    accept: () => emit("delete"),
  });
}
</script>

<template>
  <form @submit="onSubmit">
    <div class="flex gap-2 mb-2">
      <FormInputText class="flex-1" name="occupation" label="Occupation" absolute-labels />
      <FormInputText class="flex-1" name="companyName" label="Company Name" absolute-labels />
      <FormDropdown name="employmentType" label="Employment Type" :options="employmentTypeOptions" />
    </div>
    <div class="flex gap-2 mb-2">
      <FormInputText class="flex-1" name="locationName" label="Location" absolute-labels />
      <FormDropdown name="locationType" label="Location Type" :options="locationTypeOptions" />
    </div>
    <div class="flex gap-2 mb-2">
      <FormCalendarMonth class="flex-1" name="startDate" label="Start Date" />
      <FormCalendarMonth class="flex-1" name="endDate" label="End Date" :disabled="form.isCurrent" />
      <FormCheckbox name="isCurrent" label="Currently working" />
    </div>

    <FormInputText class="mb-4" name="description" label="Description" absolute-labels />

    <div class="flex justify-end gap-2">
      <Button type="submit">
        <i v-if="experienceStore.isLoading" class="pi pi-spinner pi-spin mr-2"></i>
        <i v-else class="pi pi-save mr-2"></i>
        <span>Save</span>
      </Button>
      <Button severity="secondary" @click="emit('cancel')">Cancel</Button>

      <template v-if="props.deleteable">
        <ConfirmPopup></ConfirmPopup>
        <Button severity="danger" outlined @click="confirmDelete">Delete</Button>
      </template>
    </div>
  </form>
</template>
