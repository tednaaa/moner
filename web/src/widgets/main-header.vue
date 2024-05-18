<script setup lang='ts'>
import { RouterLink } from 'vue-router';
import { routes } from '@/shared/routes';

import BaseContainer from '@/shared/ui/base-container/base-container.vue';
import EnvelopeIcon from '@/shared/icons/envelope-icon.vue';
import BellIcon from '@/shared/icons/bell-icon.vue';
import SearchIcon from '@/shared/icons/search-icon.vue';
import Menu from 'primevue/menu';
import type { MenuItem } from 'primevue/menuitem';
import { computed, ref } from 'vue';
import { PrimeIcons } from 'primevue/api'
import { useUserStore } from '@/entities/user/user.store';

const props = defineProps<{
  user: {
    avatarUrl: string
  }
}>()

const emit = defineEmits<{
  logout: []
}>()

const menuItems = computed<MenuItem[]>(() => ([{ icon: PrimeIcons.SIGN_OUT, label: "Logout", command: () => emit('logout') }]))
const menu = ref()

function toggleMenu(event: MouseEvent) {
  menu.value.toggle(event)
}

const { isLoggedIn } = useUserStore()
</script>

<template>
  <header :class="$style.header">
    <BaseContainer :class="$style.container">
      <RouterLink :class="$style.logo" :to="{ name: routes.PROFILE }">moner</RouterLink>

      <div :class="$style.rightSide">
        <label :class="$style.search">
          <SearchIcon :class="$style.searchIcon"></SearchIcon>
          <input :class="$style.searchInput" name="search" placeholder="Search..." type="text" />
        </label>

        <template v-if="isLoggedIn">
          <button :class="$style.messagesButton">
            <EnvelopeIcon></EnvelopeIcon>
          </button>
          <button :class="$style.notificationsButton">
            <BellIcon></BellIcon>
          </button>
          <button :class="$style.userButton" @click="toggleMenu" aria-haspopup="true" aria-controls="overlay_menu">
            <img :class="$style.userAvatar" :src="props.user.avatarUrl" alt="">
          </button>
          <Menu :class="$style.menu" id="overlay_menu" ref="menu" :model="menuItems" :popup="true" />
        </template>
        <RouterLink v-else :class="$style.loginButton" :to="{ name: routes.LOGIN }">Log In</RouterLink>
      </div>
    </BaseContainer>
  </header>
</template>

<style module lang='scss'>
.header {
  height: 50px;
}

.container {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  font-size: 32px;
  font-weight: 700;
  color: white;
}

.rightSide {
  display: flex;
  align-items: center;
  gap: 20px;
}

.search {
  position: relative;
}

.searchIcon {
  position: absolute;
  left: 15px;
  top: 50%;
  transform: translateY(-50%);
}

.searchInput {
  background-color: #282828;
  color: white;
  border-radius: 25px;
  height: 40px;
  width: 450px;
  padding-left: 50px;
  padding-right: 20px;
}

.messagesButton {
  width: 24px;
  height: 18px;
}

.notificationsButton {
  width: 18px;
  height: 21px;
}

.userButton {
  width: 35px;
  height: 35px;
  border: 1px solid #2d2c2c;
  border-radius: 50%;
}

.userAvatar {
  height: 100%;
  object-fit: cover;
  border-radius: 50%;
}

.menu {
  ul[data-pc-section="menu"] {
    background-color: #282828;
    color: white;
    padding: 12px 24px;
    border-radius: 4px;
  }

  a[data-pc-section="action"] {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
  }
}

.loginButton {
  background-color: #9747FF1f;
  color: #9747FF;
  padding: 10px 40px;
  border-radius: 10px;
  transition: 0.3s;

  &:hover {
    background-color: #9747ff3f;
  }
}
</style>
