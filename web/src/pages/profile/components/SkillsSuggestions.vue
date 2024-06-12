<script setup lang="ts">
import { useApiFetch } from "@/shared/api/api";
import Chip from "primevue/chip";
import ProgressSpinner from "primevue/progressspinner";
import { ref } from "vue";

export interface Skill {
  id: number;
  name: string;
}

const emit = defineEmits<{
  append: [Skill];
}>();

const skills = ref<Skill[]>([]);
const { onFetchResponse, isFetching } = useApiFetch("/skills/suggestions");

onFetchResponse(async (response) => {
  const data: Skill[] = await response.json();
  skills.value = data;
});

function onAppend(skill: Skill, index: number) {
  skills.value.splice(index, 1);
  emit("append", skill);
}
</script>

<template>
  <div v-if="isFetching">
    <ProgressSpinner
      stroke-width="8"
      class="size-10 fill-surface-0 dark:fill-surface-800"
      animation-duration=".5s"
      aria-label="Custom ProgressSpinner"
    />
  </div>
  <div v-else-if="skills?.length">
    <h3 class="text-lg font-bold mb-3">Suggestions</h3>
    <ul class="flex flex-wrap p-3 gap-2 bg-background rounded-lg">
      <li v-for="(skill, index) in skills" :key="skill.id">
        <button @click="onAppend(skill, index)">
          <Chip :label="skill.name" />
        </button>
      </li>
    </ul>
  </div>
</template>
