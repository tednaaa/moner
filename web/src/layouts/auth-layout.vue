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
  gap: 50px;
  height: 100%;
  width: 100%;
}

.statusBar {
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
  background: linear-gradient(135deg, rgba(38, 38, 38, 0.5) 0%, rgba(0, 0, 0, 0.5) 100%);
  border: 1px solid #616161;
  box-shadow: 0 4px 4px 0 rgba(0, 0, 0, 0.25);
  color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
  max-width: 500px;
  width: 100%;
  padding: 60px 50px;
  border-radius: 20px;
}
</style>
