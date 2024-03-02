<script setup lang='ts'>
import { useUserStore } from '@/modules/auth/auth.store';
import BaseButton from '@/shared/ui/base-button/base-button.vue';

const user = useUserStore()
</script>

<template>
  <BaseButton @click="user.logout">Logout</BaseButton>
</template>

<style module lang='scss'></style>
