<script setup lang='ts'>
import InputText from 'primevue/inputtext';

import GmailIcon from '@/shared/icons/gmail-icon.vue';
import LinkedinIcon from '@/shared/icons/linkedin-icon.vue';
import TelegramIcon from '@/shared/icons/telegram-icon.vue';
import GithubIcon from '@/shared/icons/github-icon.vue';
import WakatimeIcon from '@/shared/icons/wakatime-icon.vue';
import ViewsIcon from '@/shared/icons/views-icon.vue';
import StarIcon from '@/shared/icons/star-icon.vue';

const props = defineProps<{
  avatarUrl: string
  userId: number
  isFollowing: boolean
  name: string
  username: string
  occupation: string

  isCurrentUserProfile: boolean
  isEditMode: boolean
}>()

const emit = defineEmits<{
  editButtonClick: []
  saveButtonClick: []
  cancelButtonClick: []
  followButtonClick: []
  unfollowButtonClick: []
}>()

const profileLinks = [
  { id: 1, icon: GmailIcon, link: "mailto:tednaaa@gmail.com", label: "tednaaa@gmail.com" },
  { id: 2, icon: LinkedinIcon, link: "https://www.linkedin.com/in/tednaaa", label: "tednaaa" },
  { id: 3, icon: TelegramIcon, link: "https://t.me/indertale", label: "indertale" },
  { id: 4, icon: GithubIcon, link: "https://github.com/tednaaa", label: "tednaaa" },
  { id: 5, icon: WakatimeIcon, link: "https://wakatime.com/@tednaaa", label: "tednaaa" },
]
</script>

<template>
  <aside :class="$style.sidebar">
    <div :class="$style.user">
      <img
        :class="$style.avatar"
        :src="props.avatarUrl"
        alt=""
      >
      <div :class="$style.userInfo">
        <span :class="$style.name">{{ props.name }}</span>
        <span :class="$style.username">{{ props.username }}</span>
        <span :class="$style.occupation">{{ props.occupation }}</span>
      </div>
    </div>

    <div :class="$style.actions">
      <template v-if="isEditMode">
        <button
          :class="[$style.actionsButton, $style.saveButton]"
          @click="emit('saveButtonClick')"
        >
          Save
        </button>
        <button
          :class="[$style.actionsButton, $style.cancelButton]"
          @click="emit('cancelButtonClick')"
        >
          Cancel
        </button>
      </template>
      <button
        v-else-if="isCurrentUserProfile"
        :class="$style.editButton"
        @click="emit('editButtonClick')"
      >
        Edit Profile
      </button>
      <template v-else>
        <button
          v-if="props.isFollowing"
          :class="[$style.actionsButton, $style.unfollowButton]"
          @click="emit('unfollowButtonClick')"
        >
          <i class="pi pi-user-minus" />
          <span>Unfollow</span>
        </button>
        <button
          v-else
          :class="[$style.actionsButton, $style.followButton]"
          @click="emit('followButtonClick')"
        >
          <i class="pi pi-user-plus" />
          <span>Follow</span>
        </button>
        <button :class="[$style.actionsButton, $style.messageButton]">
          <i class="pi pi-envelope" />
          <span>Message</span>
        </button>
      </template>
    </div>


    <div :class="$style.links">
      <template v-if="isEditMode">
        <div :class="$style.link">
          <GmailIcon :class="$style.linkIcon" />
          <InputText
            :class="$style.linkInput"
            aria-label="Profile Gmail link"
          />
        </div>
        <div :class="$style.link">
          <LinkedinIcon :class="$style.linkIcon" />
          <InputText
            :class="$style.linkInput"
            aria-label="Profile LinkedIn link"
          />
        </div>
        <div :class="$style.link">
          <TelegramIcon :class="$style.linkIcon" />
          <InputText
            :class="$style.linkInput"
            aria-label="Profile Telegram link"
          />
        </div>
        <div :class="$style.link">
          <GithubIcon :class="$style.linkIcon" />
          <InputText
            :class="$style.linkInput"
            aria-label="Profile GitHub link"
          />
        </div>
        <div :class="$style.link">
          <WakatimeIcon :class="$style.linkIcon" />
          <InputText
            :class="$style.linkInput"
            aria-label="Profile Wakatime link"
          />
        </div>
      </template>
      <template v-else>
        <a
          v-for="{ id, icon, label, link } in profileLinks"
          :key="id"
          :class="[$style.link, $style.linkGmail]"
          :href="link"
          target="_blank"
        >
          <component
            :is="icon"
            :class="$style.linkIcon"
          />
          <span>{{ label }}</span>
        </a>
      </template>
    </div>

    <div :class="$style.stats">
      <h2 :class="$style.statsTitle">
        Community Stats
      </h2>

      <div :class="$style.statBox">
        <ViewsIcon :class="$style.statIcon" />
        <span>Views <span :class="$style.statTotal">100</span></span>
        <span>Last week <span :class="$style.statLastWeek">59</span></span>
      </div>
      <div :class="$style.statBox">
        <StarIcon :class="$style.statIcon" />
        <span>Reputation <span :class="$style.statTotal">999</span></span>
        <span>Last week <span :class="$style.statLastWeek">45</span></span>
      </div>
    </div>
  </aside>
</template>

<style module lang='scss'>
.sidebar {
  height: max-content;
  min-width: 300px;
  color: #A3A3A5;
  background-color: #282828;
  padding: 20px;
  border-radius: 20px;
}

.user {
  display: flex;
  align-items: center;
}

.avatar {
  height: 100px;
  width: 100px;
  object-fit: cover;
  border-radius: 10px;
}

.userInfo {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-left: 20px;
}

.name {
  font-size: 24px;
  line-height: 28px;
  color: white;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}

.editButton {
  background-color: #007CFF1f;
  color: #007CFF;
  width: 100%;
  padding: 10px;
  border-radius: 4px;
  transition: 0.3s;

  &:hover {
    background-color: #007bff3a;
  }
}

.actionsButton {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  gap: 10px;
  padding: 10px;
  border-radius: 4px;
}

.followButton {
  background-color: #007CFF1f;
  color: #007CFF;
  transition: 0.3s;

  &:hover {
    background-color: #007bff3a;
  }
}

.unfollowButton {
  background-color: #ff00001f;
  color: #ff0000;
  transition: 0.3s;

  &:hover {
    background-color: #ff00003a;
  }
}

.messageButton {
  background-color: #ccd60f1f;
  color: #ccd60f;
  transition: 0.3s;

  &:hover {
    background-color: #ccd60f3a;
  }
}

.saveButton {
  background-color: #2cbb5d1f;
  color: #2cbb5d;
}

.cancelButton {
  background-color: #ff00001f;
  color: #ff0000;
}

.links {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 20px;
}

.link {
  display: flex;
  align-items: center;
  gap: 10px;
}

.linkIcon {
  height: 24px;
  width: 24px;
}

.linkInput {
  border: 1px solid #494949;
  color: white;
  width: 100%;
  height: 24px;
  font-size: 14px;
  line-height: 17px;
  border-radius: 10px;
  padding: 15px 10px;
}

.stats {
  display: flex;
  flex-direction: column;
  gap: 15px;
  border-top: 1px solid #616161;
  margin-top: 20px;
  padding-top: 20px;
}

.statsTitle {
  font-size: 20px;
  line-height: 23px;
  color: white;
  margin-bottom: 10px;
}

.statBox {
  display: flex;
  flex-direction: column;
  position: relative;
  padding-left: 30px;
  gap: 5px;
}

.statIcon {
  max-width: 20px;
  height: auto;
  position: absolute;
  top: 2px;
  left: 0;
}

.statTotal {
  color: white;
}

.statLastWeek {
  color: #616161;
}
</style>
