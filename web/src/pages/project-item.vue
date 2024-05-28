<script setup lang='ts'>
import EyeIcon from '@/shared/icons/eye-icon.vue';
import HearthIcon from '@/shared/icons/hearth-icon.vue';
import { routes } from '@/shared/routes';

export interface ProjectItem {
  previewImage: string
  projectId: number
  views: number
  likes: number
  title: string
}

const props = defineProps<ProjectItem>()
</script>

<template>
  <RouterLink
    :class="$style.project"
    :to="{ name: routes.PROJECT, params: { projectId } }"
  >
    <div :class="$style.info">
      <div :class="$style.head">
        <div :class="$style.views">
          <EyeIcon />
          <span>{{ props.views }}</span>
        </div>
        <div :class="$style.likes">
          <HearthIcon />
          <span>{{ props.likes }}</span>
        </div>
      </div>

      <h3 :class="$style.title">
        {{ props.title }}
      </h3>
    </div>

    <img
      :src="props.previewImage"
      :class="$style.previewImage"
      alt=""
    >
  </RouterLink>
</template>

<style module lang='scss'>
.project {
  color: white;
  border-radius: 10px;
  position: relative;

  &:hover {
    .info {
      opacity: 1;
    }
  }
}

.previewImage {
  border-radius: 10px;
}

.info {
  position: absolute;
  border-radius: 10px;
  top: 0;
  left: 0;
  height: 100%;
  width: 100%;
  padding: 15px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  box-shadow: inset 0 100px 250px 50px rgba(0, 0, 0, 0.5);
  transition: 0.3s;
  opacity: 0;
}

.head {
  display: flex;
  align-items: center;
  gap: 10px;
}

.views,
.likes {
  display: flex;
  align-items: center;
  gap: 5px;
}

.title {
  margin-top: 20px;
}
</style>
