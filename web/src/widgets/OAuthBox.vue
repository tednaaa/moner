<script setup lang="ts">
import { API_URL } from "@/shared/config";

import googleIcon from "@/shared/icons/google-icon.vue";
import gitlabIcon from "@/shared/icons/gitlab-icon.vue";
import githubIcon from "@/shared/icons/github-icon.vue";

const oAuthList = [
  { provider: "google", iconComponent: googleIcon },
  { provider: "gitlab", iconComponent: gitlabIcon },
  { provider: "github", iconComponent: githubIcon },
];
</script>

<template>
  <ul class="flex gap-6">
    <li
      v-for="(oAuth, index) in oAuthList"
      :key="index"
      class="flex items-center justify-center w-12 h-12 rounded-md bg-secondary text-secondary-foreground shadow hover:bg-secondary/50"
    >
      <a class="p-3" :href="`${API_URL}/oauth/${oAuth.provider}`">
        <component :is="oAuth.iconComponent" />
      </a>
    </li>
  </ul>
</template>
