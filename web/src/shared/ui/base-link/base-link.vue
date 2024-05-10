<script setup lang='ts'>
import type { RouterLinkProps } from 'vue-router';

const props = defineProps<RouterLinkProps>()
</script>

<template>
  <RouterLink :class="$style.link" v-bind="props">
    <slot></slot>
  </RouterLink>
</template>

<style module lang='scss'>
.link {
  font-weight: 500;
  color: #478FE8;

  &:hover {
    color: var(--color-blue);
  }

  &:disabled {
    color: var(--color-disabled);
  }
}
</style>
