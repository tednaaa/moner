<script setup lang='ts'>
import type { ButtonHTMLAttributes } from 'vue';
import BaseSpinner from '../base-spinner/base-spinner.vue';

const props = defineProps<{
  isLoading?: boolean
  type?: ButtonHTMLAttributes['type']
}>()
const emit = defineEmits<{
  click: [event: MouseEvent]
}>()
</script>

<template>
  <button
    :class="[$style.button, props.isLoading && $style.buttonLoading]"
    :type="props.type"
    @click="event => emit('click', event)"
  >
    <BaseSpinner
      :class="$style.spinner"
      :size="35"
      big-path-color="white"
      small-path-color="#13d7bd"
    />
    <slot />
  </button>
</template>

<style module lang='scss'>
.button {
  background-color: var(--color-blue);
  color: white;
  transition: 0.3s;
  padding: 10px 25px;
  border-radius: 10px;
  display: flex;
  justify-content: center;
  position: relative;

  &:hover {
    background-color: #368be7;
  }
}

.buttonLoading {
  color: var(--color-blue);
}

.spinner {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  opacity: 0;

  .buttonLoading & {
    opacity: 1;
  }
}
</style>
