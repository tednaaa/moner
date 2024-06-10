<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { computed, ref } from "vue";
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";
import Chips from "primevue/chips";
import Chip from "primevue/chip";

import FolderIcon from "@/shared/icons/folder-icon.vue";
import CvIcon from "@/shared/icons/cv-icon.vue";

import { useUserStore } from "@/entities/user/user.store";

import mockImage from "@/assets/mock.jpg";
import type { PublicUserResponse } from "@/entities/user/user.types";
import { useApiFetch } from "@/shared/api/api";
import { useFollowsStore } from "@/entities/follows/follows.store";
import { routes } from "@/shared/routes";
import { AUTH_SESSION_REFERRER_KEY } from "@/shared/config";
import type { Experience, ExperienceDto } from "@/entities/experience/experience.types";
import Header from "@/widgets/Header.vue";
import ProjectCard from "@/widgets/ProjectCard.vue";

import ProfileSidebar from "./components/ProfileSidebar.vue";
import ExperienceItem from "./components/ExperienceItem.vue";
import ExperienceForm from "./components/ExperienceForm.vue";
import { useExperienceStore } from "@/entities/experience/experience.store";

const userStore = useUserStore();
const followsStore = useFollowsStore();
const experienceStore = useExperienceStore();
const router = useRouter();
const route = useRoute();

const isCurrentUserProfile = computed(() => route.params.username === userStore.currentUser.username);

const skills = ref(["React", "Vue", "Angular", "Node", "Solidity"]);
const initialSkills = ref(skills.value);

const isEditMode = ref(false);
const showExperienceForm = ref(false);
const isFollowing = ref(false);

const { data: visitedUser, onFetchResponse } = useApiFetch(
  `/users/${route.params.username}`,
).json<PublicUserResponse>();
const experienceList = ref<Experience[]>([]);

onFetchResponse(async () => {
  if (!visitedUser.value) return;

  isFollowing.value = visitedUser.value.isFollowing;

  const { data } = await useApiFetch(`/${visitedUser.value.id}/experience`).json<Experience[]>();
  if (data.value)
    experienceList.value = data.value.map((experience) => ({
      ...experience,
      startDate: new Date(experience.startDate),
      endDate: experience.endDate ? new Date(experience.endDate) : undefined,
    }));
});

function updateProfile() {
  updateSkills();
  disableEditMode();
}

function updateSkills() {
  initialSkills.value = skills.value;
  // TODO: API request
  console.log("update_skills", skills.value);
}

function cancelEdit() {
  skills.value = initialSkills.value;
  disableEditMode();
}

function enableEditMode() {
  if (!isCurrentUserProfile.value) return;

  isEditMode.value = true;
}

function disableEditMode() {
  isEditMode.value = false;
}

async function followUser() {
  if (!visitedUser.value) return;
  if (!userStore.isLoggedIn) {
    sessionStorage.setItem(AUTH_SESSION_REFERRER_KEY, router.currentRoute.value.fullPath);
    router.push({ name: routes.LOGIN });
    return;
  }

  const isFollowed = await followsStore.followUser(visitedUser.value.id);
  if (isFollowed) {
    isFollowing.value = true;
    visitedUser.value.followersCount += 1;
  }
}

async function unfollowUser() {
  if (!visitedUser.value) return;

  const isUnfollowed = await followsStore.unfollowUser(visitedUser.value.id);
  if (isUnfollowed) {
    visitedUser.value.followersCount -= 1;
    isFollowing.value = false;
  }
}

async function createExperience(experience: ExperienceDto) {
  showExperienceForm.value = false;

  const createdExperienceId = await experienceStore.create(experience);

  if (createdExperienceId)
    experienceList.value.unshift({
      id: createdExperienceId,
      userId: userStore.currentUser.id,
      ...experience,
    });
}

function updateExperienceInList(id: number, experienceDto: ExperienceDto) {
  const index = experienceList.value.findIndex((experience) => experience.id === id);
  if (index !== -1) experienceList.value[index] = { ...experienceList.value[index], ...experienceDto };
}

function deleteExperienceInList(id: number) {
  const index = experienceList.value.findIndex((experience) => experience.id === id);
  if (index !== -1) experienceList.value.splice(index, 1);
}
</script>

<template>
  <div v-if="visitedUser" class="min-h-full bg-background">
    <Header class="sticky top-0 z-[1] pt-4 pb-6" />
    <div class="container flex gap-5 pb-12">
      <ProfileSidebar
        class="sticky top-[5.25rem]"
        :user-id="visitedUser.id"
        :avatar-url="mockImage"
        occupation="Developer"
        name="Andranik"
        :username="visitedUser.username"
        :is-following="userStore.isLoggedIn && isFollowing"
        :followers-count="visitedUser.followersCount"
        :following-count="visitedUser.followingCount"
        :is-edit-mode="isEditMode"
        :is-current-user-profile="isCurrentUserProfile"
        @edit-button-click="enableEditMode"
        @save-button-click="updateProfile"
        @cancel-button-click="cancelEdit"
        @follow-button-click="followUser"
        @unfollow-button-click="unfollowUser"
      />
      <div class="w-full">
        <TabView :active-index="1" :pt="{ navContainer: 'sticky top-[5.25rem] z-[1] pb-5' }">
          <TabPanel :pt="{ headerAction: 'flex gap-2 text-lg' }">
            <template #header>
              <span>Projects</span>
              <FolderIcon />
            </template>
            <ul class="grid gap-5 grid-cols-2 lg:grid-cols-3">
              <li v-for="i in 25" :key="i">
                <ProjectCard
                  :project-id="i"
                  :preview-image="mockImage"
                  :title="`Project ${i}`"
                  :views="10"
                  :likes="24"
                />
              </li>
            </ul>
          </TabPanel>
          <TabPanel :pt="{ headerAction: 'flex gap-2 text-lg' }">
            <template #header>
              <span>CV</span>
              <CvIcon />
            </template>
            <div class="flex flex-col gap-5">
              <div class="p-5 rounded-lg shadow bg-card text-card-foreground">
                <h2 class="text-2xl font-bold text-white">Skills</h2>

                <div class="flex flex-wrap gap-2 mt-5">
                  <Chips
                    v-if="isEditMode"
                    v-model="skills"
                    placeholder="🐙  Tell me about your skills"
                    :max="30"
                    :pt="{ inputToken: 'min-w-60' }"
                    :pt-options="{ mergeProps: true }"
                  />
                  <Chip v-for="skill in skills" v-else :key="skill">
                    {{ skill }}
                  </Chip>
                </div>
              </div>
              <div class="p-5 rounded-lg shadow bg-card text-card-foreground">
                <div class="flex items-center justify-between gap-2 mb-8">
                  <h2 class="text-2xl font-bold text-white">Experience</h2>
                  <button
                    v-if="isCurrentUserProfile && !showExperienceForm"
                    class="flex items-center justify-center p-2 bg-zinc-700 text-neutral-400 rounded-lg transition-colors hover:bg-zinc-700/80"
                    @click="showExperienceForm = true"
                  >
                    <i class="pi pi-plus text-lg"></i>
                  </button>
                </div>
                <ExperienceForm
                  v-if="isCurrentUserProfile && showExperienceForm"
                  class="mb-8"
                  @submit="createExperience"
                  @cancel="showExperienceForm = false"
                />
                <ul v-if="experienceList.length">
                  <li
                    v-for="experience in experienceList"
                    :key="experience.id"
                    class="not-last:mb-5 not-last:pb-5 not-last:border-b border-zinc-600"
                  >
                    <ExperienceItem
                      :is-current-user-profile="isCurrentUserProfile"
                      :experience="experience"
                      @update="updateExperienceInList"
                      @delete="deleteExperienceInList"
                    />
                  </li>
                </ul>
              </div>
            </div>
          </TabPanel>
        </TabView>
      </div>
    </div>
  </div>
  <div v-else>User not found!</div>
</template>
