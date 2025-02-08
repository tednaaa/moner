<script setup lang="ts">
import IconMessage from '@/assets/icons/IconMessage.vue'
import Button from 'primevue/button'
</script>

<template>
  <main>
    <h1>This is profile page</h1>

    <Button>
      <template #icon>
        <IconMessage />
      </template>
      Send message
    </Button>
  </main>
</template>
