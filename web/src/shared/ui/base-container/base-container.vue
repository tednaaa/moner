<script setup lang='ts'></script>

<template>
  <div :class="$style.container">
    <slot></slot>
  </div>
</template>

<style module lang='scss'>
.container {
  max-width: 1420px;
  width: 100%;
  padding: 0 20px;
  margin: 0 auto;
}
</style>
