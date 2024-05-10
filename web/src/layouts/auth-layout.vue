<script setup lang='ts'>
export type AuthLayoutStatus = 'default' | 'error'

const props = defineProps<{
  status: AuthLayoutStatus
  statusText: string
}>()

const emit = defineEmits<{
  submit: []
}>()
</script>

<template>
  <div :class="$style.wrapper">
    <div :class="$style.statusBar">
      <span :class="[$style.statusText, $style[`status-${props.status}`]]">{{ props.statusText }}</span>
      <div :class="$style.statusImageWrapper">
        <img :class="$style.statusImage" src="@/assets/logo.svg" alt="logo">
      </div>
    </div>
    <form :class="$style.form" @submit.prevent="emit('submit')">
      <slot></slot>
    </form>
  </div>
</template>

<style module lang='scss'>
.wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 100px;
  height: 100%;
  width: 100%;
  background: #283c86;
  background: linear-gradient(to right, #45a247, #283c86);
}

.statusBar {
  background-color: var(--color-blue);
  border-radius: 40px;
  padding: 30px;
  position: relative;
}

.statusImageWrapper {
  height: 150px;
  width: 150px;
  position: relative;

  &::before,
  &::after {
    content: '';
    position: absolute;
    border-radius: 50%;
    background-color: white;
  }

  &::before {
    height: 20px;
    width: 20px;
    top: 0;
    right: 0;
  }

  &::after {
    height: 30px;
    width: 30px;
    top: -40px;
    right: -30px;
  }
}

.statusText {
  display: block;
  padding: 20px 40px;
  margin-bottom: 50px;
  margin-left: 100px;
  font-size: 20px;
  font-weight: 500;
  background-color: white;
  color: var(--color-black);
  border-radius: 20px;
}

.status-default {
  color: var(--color-black);
}

.form {
  padding: 60px 50px;
  background-color: white;
  border-radius: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  max-width: 500px;
  width: 100%;
}
</style>
