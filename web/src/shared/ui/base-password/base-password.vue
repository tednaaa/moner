<script setup lang='ts'>
import { ref } from 'vue';

import EyeIcon from '@/shared/icons/eye-icon.vue'

import BaseInput from '../base-input/base-input.vue';

const props = defineProps<{
  name: string
  label: string
}>()

const show = ref(false)

function toggleMask() {
  show.value = !show.value
}
</script>

<template>
  <BaseInput
    :name="props.name"
    :label="props.label"
    :type="show ? 'text' : 'password'"
  >
    <template #right>
      <button
        :class="[$style.eyeButton, show || $style.eyeButtonHidden]"
        aria-label="Toggle"
        type="button"
        @click="toggleMask"
      >
        <EyeIcon :class="$style.eyeIcon" />
      </button>
    </template>
  </BaseInput>
</template>

<style module lang='scss'>
.eyeButton {
  position: relative;
  padding: 0 10px;

  &::after {
    content: '';
    position: absolute;
    height: 2px;
    width: 0;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) rotate(45deg);
    background-color: var(--color-gray);
    transition: 0.2s;
  }

  &:focus-visible {
    outline: none;
    background-color: #2c2828;
    border-radius: 9px;
  }

  &:active {
    background-color: #424141;
  }
}

.eyeButtonHidden {
  &::after {
    width: 50%;
  }
}

.eyeIcon {
  display: block;
  color: var(--color-gray);
  width: 20px;
  height: 20px;
}
</style>
