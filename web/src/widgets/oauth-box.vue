<script setup lang='ts'>
import { API_URL } from '@/shared/config';

import googleIcon from '@/shared/icons/google-icon.vue'
import gitlabIcon from '@/shared/icons/gitlab-icon.vue'
import githubIcon from '@/shared/icons/github-icon.vue'

const oAuthList = [
  { provider: 'google', iconComponent: googleIcon },
  { provider: 'gitlab', iconComponent: gitlabIcon },
  { provider: 'github', iconComponent: githubIcon }
]
</script>

<template>
  <ul :class="$style.OAuthList">
    <li :class="$style.OAuthItem" v-for="oAuth, index in oAuthList" :key="index">
      <a :class="$style.OAuthLink" :href="`${API_URL}/oauth/${oAuth.provider}`">
        <component :is="oAuth.iconComponent"></component>
      </a>
    </li>
  </ul>
</template>

<style module lang='scss'>
.OAuthList {
  display: flex;
  gap: 25px;
  margin-bottom: 35px;
}

.OAuthItem {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 5px;
  box-shadow: 1px 2px 5px 0 rgba(161, 161, 161, 0.25);
  background-color: #2a2a2a;
  transition: 0.2s;

  &:hover {
    background-color: #414040;
  }
}

.OAuthLink {
  padding: 12px;
}
</style>
