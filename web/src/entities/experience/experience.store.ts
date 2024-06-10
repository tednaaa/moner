import { defineStore, acceptHMRUpdate } from "pinia";

import { useApiFetch } from "@/shared/api/api";
import type { ExperienceDto } from "./experience.types";
import { ref } from "vue";
import { useToast } from "primevue/usetoast";

export const useExperienceStore = defineStore("experience", () => {
  const toast = useToast();
  const isLoading = ref(false);

  function create(experienceDto: ExperienceDto): Promise<number | null> {
    return new Promise((resolve) => {
      isLoading.value = true;
      const { onFetchError, onFetchResponse, onFetchFinally } = useApiFetch("/experience").post(experienceDto);

      onFetchError(() => {
        toast.add({
          severity: "error",
          summary: "Failed to create",
          detail: "Please try again later",
        });
        resolve(null);
      });

      onFetchResponse((response) => response.json().then((experienceId: number) => resolve(experienceId)));
      onFetchFinally(() => (isLoading.value = false));
    });
  }

  function update(id: number, experienceDto: ExperienceDto): Promise<boolean> {
    return new Promise((resolve) => {
      isLoading.value = true;
      const { onFetchError, onFetchResponse, onFetchFinally } = useApiFetch(`/experience/${id}`).put(experienceDto);

      onFetchError(() => {
        toast.add({
          severity: "error",
          summary: "Failed to update",
          detail: "Please try again",
        });
        resolve(false);
      });

      onFetchResponse(() => resolve(true));
      onFetchFinally(() => (isLoading.value = false));
    });
  }

  function deleteExperience(id: number): Promise<boolean> {
    return new Promise((resolve) => {
      isLoading.value = true;
      const { onFetchError, onFetchResponse, onFetchFinally } = useApiFetch(`/experience/${id}`).delete();

      onFetchError(() => {
        toast.add({
          severity: "error",
          summary: "Failed to delete",
          detail: "Please try again",
        });
        resolve(false);
      });

      onFetchResponse(() => resolve(true));
      onFetchFinally(() => (isLoading.value = false));
    });
  }

  return { isLoading, create, update, deleteExperience };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useExperienceStore, import.meta.hot));
}
