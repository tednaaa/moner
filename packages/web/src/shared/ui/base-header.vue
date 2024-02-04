<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { routes } from '@/shared/router/routes'

</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <nav>
        <ul>
          <li>
            <RouterLink :to="routes.home">Home</RouterLink>
          </li>
          <li>
            <RouterLink :to="routes.about">About</RouterLink>
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>
