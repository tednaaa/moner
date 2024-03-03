<script setup lang='ts'>
import { useUserStore } from '@/modules/user/user.store';
import BaseButton from '@/shared/ui/base-button/base-button.vue';

const { logoutUser } = useUserStore()
</script>

<template>
  <BaseButton @click="logoutUser">Logout</BaseButton>
</template>

<style module lang='scss'></style>
