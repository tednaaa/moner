<script setup lang="ts">
import type { Experience, ExperienceDto } from "@/entities/experience/experience.types";
import { useDateFormat } from "@vueuse/core";
import { addYears, differenceInMonths, differenceInYears } from "date-fns";
import { computed, ref } from "vue";
import ExperienceForm from "./ExperienceForm.vue";
import { useExperienceStore } from "@/entities/experience/experience.store";

const props = defineProps<{
  isCurrentUserProfile: boolean;
  experience: Experience;
}>();

const emit = defineEmits<{
  update: [number, ExperienceDto];
  delete: [number];
}>();

const experienceStore = useExperienceStore();
const isEditMode = ref(false);

const formattedStartDate = useDateFormat(props.experience.startDate, "MMMM YYYY");
const formattedEndDate = computed(() => {
  if (props.experience.isCurrent) return "Present";

  return useDateFormat(props.experience.endDate, "MMMM YYYY").value;
});

const duration = computed(() => {
  const endDate = props.experience.isCurrent || !props.experience.endDate ? new Date() : props.experience.endDate;
  const years = differenceInYears(endDate, props.experience.startDate);
  const months = differenceInMonths(endDate, addYears(props.experience.startDate, years));

  let result = "";
  if (years > 0) result += `${years} year${years > 1 ? "s" : ""} `;
  if (months > 0) result += `${months} month${months > 1 ? "s" : ""} `;

  return result.trim();
});

async function updateExperience(experienceDto: ExperienceDto) {
  const isUpdated = await experienceStore.update(props.experience.id, experienceDto);
  if (isUpdated) {
    emit("update", props.experience.id, experienceDto);
    isEditMode.value = false;
  }
}

async function deleteExperience() {
  const isDeleted = await experienceStore.deleteExperience(props.experience.id);
  if (isDeleted) {
    emit("delete", props.experience.id);
    isEditMode.value = false;
  }
}
</script>

<template>
  <ExperienceForm
    v-if="isEditMode"
    :experience="props.experience"
    deleteable
    @submit="updateExperience"
    @cancel="isEditMode = false"
    @delete="deleteExperience"
  />
  <div v-else>
    <div>
      <div class="flex items-center justify-between text-xl text-white mb-3">
        <span>
          <i class="pi pi-briefcase mr-2"></i>
          {{ props.experience.occupation }}
        </span>
        <button
          v-if="isCurrentUserProfile"
          class="flex items-center justify-center p-2 bg-zinc-700 rounded-lg ml-4 transition-colors text-zinc-400 hover:bg-zinc-700/80"
          @click="isEditMode = true"
        >
          <i class="pi pi-pencil text-lg"></i>
        </button>
      </div>
      <div class="flex flex-wrap gap-8">
        <div class="flex items-center gap-2">
          <i class="pi pi-calendar"></i>
          <span>{{ formattedStartDate }} - {{ formattedEndDate }} ({{ duration }})</span>
        </div>
        <div class="flex items-center gap-2">
          <i class="pi pi-building"></i>
          <span>{{ props.experience.companyName }}</span>
          <span>·</span>
          <span>{{ props.experience.employmentType }}</span>
        </div>
        <div class="flex items-center gap-2">
          <i class="pi pi-map-marker"></i>
          <span v-if="props.experience.locationName">{{ props.experience.locationName }} ·</span>
          <span>{{ props.experience.locationType }}</span>
        </div>
      </div>
    </div>
    <p class="mt-5">
      {{ props.experience.description }}
    </p>
  </div>
</template>
