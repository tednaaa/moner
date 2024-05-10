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
  <button :class="[$style.button, isLoading && $style.buttonLoading]" :type="props.type"
    @click="event => emit('click', event)">
    <BaseSpinner :class="$style.spinner" :size="35" big-path-color="white" small-path-color="#13d7bd" />
    <slot></slot>
  </button>
</template>

<style module lang='scss'>
.button {
  display: flex;
  justify-content: center;
  position: relative;
  padding: 10px 25px;
  color: white;
  background-color: var(--color-blue);
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
