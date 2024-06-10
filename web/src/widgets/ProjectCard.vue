<script setup lang="ts">
import EyeIcon from "@/shared/icons/eye-icon.vue";
import HearthIcon from "@/shared/icons/hearth-icon.vue";
import { routes } from "@/shared/routes";

export interface ProjectItem {
  previewImage: string;
  projectId: number;
  views: number;
  likes: number;
  title: string;
}

const props = defineProps<ProjectItem>();
</script>

<template>
  <RouterLink
    class="group relative text-white rounded-lg"
    :to="{ name: routes.PROJECT, params: { projectId: props.projectId } }"
  >
    <div
      class="group-hover:opacity-100 absolute top-0 left-0 h-full w-full inset-0 rounded-lg p-4 flex flex-col justify-between shadow-[inset_0_100px_250px_50px_rgba(0,0,0,0.7)] transition-opacity duration-300 opacity-0"
    >
      <div class="flex items-center gap-2">
        <div class="flex items-center gap-1">
          <EyeIcon />
          <span>{{ props.views }}</span>
        </div>
        <div class="flex items-center gap-1">
          <HearthIcon />
          <span>{{ props.likes }}</span>
        </div>
      </div>

      <h3 class="mt-5">
        {{ props.title }}
      </h3>
    </div>

    <img class="rounded-lg" :src="props.previewImage" alt="" />
  </RouterLink>
</template>
