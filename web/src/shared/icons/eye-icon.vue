<script setup lang="ts">
const props = defineProps<{
  class?: string,
}>()
</script>

<template>
  <svg :class="props.class" width="15.000000" height="15.000000" viewBox="0 0 15 15" fill="none"
    xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <defs>
      <clipPath id="clip19_1228">
        <rect id="eye" width="15.000000" height="15.000000" fill="white" fill-opacity="0" />
      </clipPath>
    </defs>
    <rect id="eye" width="15.000000" height="15.000000" fill="white" fill-opacity="0" />
    <g clip-path="url(#clip19_1228)">
      <path id="Vector"
        d="M13.75 7.5C13.75 7.6972 13.655 7.83044 13.465 8.09686C12.6112 9.29413 10.4308 11.875 7.5 11.875C4.56924 11.875 2.38882 9.29413 1.53503 8.09686C1.345 7.83044 1.25 7.6972 1.25 7.5C1.25 7.3028 1.345 7.16956 1.53503 6.90314C2.38882 5.70587 4.56924 3.125 7.5 3.125C10.4308 3.125 12.6112 5.70587 13.465 6.90314C13.655 7.16956 13.75 7.3028 13.75 7.5Z"
        stroke="currentColor" stroke-opacity="1.000000" stroke-width="1.500000" />
      <path id="Vector"
        d="M7.5 5.625C6.46445 5.625 5.625 6.46442 5.625 7.5C5.625 8.53558 6.46445 9.375 7.5 9.375C8.53555 9.375 9.375 8.53558 9.375 7.5C9.375 6.46442 8.53555 5.625 7.5 5.625Z"
        stroke="currentColor" stroke-opacity="1.000000" stroke-width="1.500000" />
    </g>
  </svg>
</template>
