<script setup lang='ts'>
import { useRoute } from 'vue-router';
import { computed, ref } from 'vue';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';
import Chips from 'primevue/chips';
import Chip from 'primevue/chip';

import FolderIcon from '@/shared/icons/folder-icon.vue';
import CvIcon from '@/shared/icons/cv-icon.vue';

import { useUserStore } from '@/entities/user/user.store';

import BaseContainer from '@/shared/ui/base-container/base-container.vue';
import ProfileSidebar from './profile-sidebar.vue';
import MainHeader from '@/widgets/main-header.vue';
import ExperienceItem, { type ExperienceItemInterface } from './experience-item.vue';

import mockImage from '@/assets/mock.jpg';
import type { PublicUserResponse } from '@/entities/user/user.types';
import { useApiFetch } from '@/shared/api/api';
import { routes } from '@/shared/routes';

const userStore = useUserStore()
const route = useRoute()

const isCurrentUserProfile = computed(() => route.params.username === userStore.currentUser.username)

const skills = ref<string[]>(["React", "Vue", "Angular", "Node", "Solidity"]);
const initialSkills = ref(skills.value);

const isEditMode = ref(false)

const { data: visitedUser } = useApiFetch(`/users/${route.params.username}`).json<PublicUserResponse>()

function updateProfile() {
  updateSkills()
  disableEditMode()
}

function updateSkills() {
  initialSkills.value = skills.value
  // TODO: API request
  console.log("update_skills", skills.value)
}

function cancelEdit() {
  skills.value = initialSkills.value
  disableEditMode()
}

function enableEditMode() {
  if (!isCurrentUserProfile.value) return

  isEditMode.value = true
}

function disableEditMode() {
  isEditMode.value = false
}


const experience: ExperienceItemInterface[] = [{
  company: {
    name: "Solicy",
    logoUrl: mockImage
  },
  occupation: "Software Engineer",
  location: {
    name: "Yerevan, Armenia",
    type: "on-site"
  },
  employmentType: "full-time",
  date: "June 2020 - October 2021 (1 year 6 months)",
  description: "Project ethermail.io is essentially like Gmail, but with web3 features"
},
{
  company: {
    name: "PMP Tech",
    logoUrl: mockImage
  },
  occupation: "Frontend Engineer",
  location: {
    type: "remote"
  },
  employmentType: "part-time",
  date: "March 2019 - June 2020 (1 year 1 month)",
  description: "lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum"
}]
</script>

<template>
  <div v-if="visitedUser" :class="$style.wrapper">
    <MainHeader :class="$style.header" :user="{ avatarUrl: mockImage }" :is-current-user-profile="isCurrentUserProfile"
      @logout="userStore.logoutUser" />
    <BaseContainer :class="$style.container">
      <ProfileSidebar :class="$style.sidebar" :avatar-url="mockImage" occupation="Developer" name="Andranik"
        :username="visitedUser.username" :is-edit-mode="isEditMode" :is-current-user-profile="isCurrentUserProfile"
        @edit-button-click="enableEditMode" @save-button-click="updateProfile" @cancel-button-click="cancelEdit" />
      <div :class="$style.mainContainer">
        <TabView :class="$style.tabView" :active-index="1">
          <TabPanel>
            <template #header>
              <span :class="$style.tabPanelHeader">
                <span>Projects</span>
                <FolderIcon></FolderIcon>
              </span>
            </template>
            <div :class="$style.projects">
              <RouterLink :class="$style.project" v-for="i in 25" :key="i"
                :to="{ name: routes.PROJECT, params: { projectId: i } }">
                <img :src="mockImage" alt="">
              </RouterLink>
            </div>
          </TabPanel>
          <TabPanel>
            <template #header>
              <span :class="$style.tabPanelHeader">
                <span>CV</span>
                <CvIcon></CvIcon>
              </span>
            </template>
            <div :class="$style.contentWrapper">
              <div :class="$style.content">
                <h2 :class="$style.contentTitle">Skills</h2>

                <div :class="$style.skills">
                  <Chips v-if="isEditMode" placeholder="🐙  Tell me about your skills" :max="30" v-model="skills" />
                  <Chip v-else v-for="skill in skills" :key="skill" :class="$style.skill">{{ skill }}</Chip>
                </div>
              </div>
              <div :class="$style.content">
                <h2 :class="$style.contentTitle">Experience</h2>
                <ul :class="$style.experience">
                  <li :class="$style.experienceItem" v-for="item in experience" :key="item.company.name">
                    <ExperienceItem :item="item" />
                  </li>
                </ul>
              </div>
            </div>
          </TabPanel>
        </TabView>
      </div>
    </BaseContainer>
  </div>
  <div v-else>User not found!</div>
</template>

<style module lang='scss'>
.wrapper {
  min-height: 100%;
  background-color: #1A1A1A;
}

.header {
  background-color: #1A1A1A;
  position: sticky;
  top: 0;
  padding-top: 20px;
  padding-bottom: 60px;
}

.container {
  display: flex;
  gap: 20px;
  padding-bottom: 50px;
}

.sidebar {
  position: sticky;
  top: 80px;
}

.mainContainer {
  width: 100%;
}

.tabView {
  div[data-pc-section="navcontainer"] {
    background-color: #1a1a1a;
    position: sticky;
    top: 80px;
    padding-bottom: 20px;
  }

  ul[data-pc-section="nav"] {
    display: flex;
    gap: 2px;
  }

  li[data-pc-name="tabpanel"] {
    cursor: pointer;
  }

  li[data-p-active="true"] {
    border-radius: 5px;
    background-color: #282828;
    color: white;
  }

  li[data-pc-section="inkbar"] {
    height: 0;
  }
}

.tabPanelHeader {
  display: flex;
  align-items: center;
  gap: 10px;
  color: white;
  padding: 10px 15px;
  font-weight: 700;
  font-size: 20px;
}

.contentWrapper {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.content {
  padding: 20px;
  border-radius: 20px;
  color: #A3A3A5;
  background-color: #282828;
}

.contentTitle {
  font-size: 24px;
  line-height: 28px;
  font-weight: 700;
  color: white;
  margin-bottom: 30px;
}

.projects {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;

  @media (max-width: 1200px) {
    grid-template-columns: repeat(2, 1fr);
  }
}

.project {
  border-radius: 10px;
}

.skills {

  &,
  ul[data-pc-section="container"] {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }

  div[data-pc-name="chips"] {
    width: 100%;
  }

  input[data-pc-section="input"] {
    background-color: #1a1a1a;
    color: white;
    padding: 8px 16px;
    border-radius: 10px;
    width: 100%;
  }

  li[data-pc-section="inputtoken"] {
    flex: 1 1 auto;
    display: inline-flex;

    &:not(:only-child) {
      margin-left: 8px;
    }
  }

  li[data-pc-section="token"],
  div[data-pc-name="chip"] {
    position: relative;
    display: inline-flex;
    align-items: center;
    background-color: #494949;
    color: #A3A3A5;
    padding: 5px 15px;
    border-radius: 20px;
  }

  svg[data-pc-section="removetokenicon"] {
    position: absolute;
    height: 20px;
    width: 20px;
    top: -8px;
    right: -8px;
    cursor: pointer;

    &:hover {
      color: #cc2424;
    }
  }
}

.experienceItem {
  &:not(:last-child) {
    margin-bottom: 20px;
    padding-bottom: 20px;
    border-bottom: 1px solid #616161;
  }
}
</style>
