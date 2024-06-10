<script setup lang="ts">
const props = defineProps<{
  statusText: string;
}>();
</script>

<template>
  <div class="flex justify-center items-center gap-12 h-full w-full">
    <div class="relative">
      <span class="block py-5 px-10 mb-12 ml-24 text-xl font-medium bg-secondary text-white rounded-lg">
        {{ props.statusText }}
      </span>
      <div :class="$style.statusImageWrapper">
        <img src="@/assets/logo.svg" alt="logo" />
      </div>
    </div>
    <div
      class="flex flex-col max-w-md w-full py-6 px-8 rounded-lg text-white bg-gradient-to-br from-zinc-800 to-zinc-900"
    >
      <slot></slot>
    </div>
  </div>
</template>

<style module lang="scss">
.statusImageWrapper {
  height: 150px;
  width: 150px;
  position: relative;

  &::before,
  &::after {
    content: "";
    position: absolute;
    border-radius: 50%;
    background-color: hsl(var(--secondary));
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
</style>
