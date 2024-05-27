<script setup lang='ts'>
export interface ExperienceItemInterface {
  company: {
    name: string
    logoUrl: string
  }
  occupation: string
  location: {
    name?: string
    type: "remote" | "hybrid" | "on-site"
  }
  employmentType: "full-time" | "part-time" | "internship" | "freelance"
  date: string
  description: string
}

const props = defineProps<{
  item: ExperienceItemInterface
}>()
</script>

<template>
  <div :class="$style.experienceItem">
    <div :class="$style.experienceItemHeader">
      <img
        :class="$style.experienceItemLogo"
        :src="props.item.company.logoUrl"
        alt=""
      >
      <div :class="$style.experienceItemInfo">
        <span :class="$style.experienceItemOccupation">{{ props.item.occupation }}</span>
        <span :class="$style.experienceItemDate">{{ props.item.date }}</span>
        <span :class="$style.experienceItemName">
          {{ props.item.company.name }}
          ·
          {{ props.item.employmentType }}
        </span>
        <span :class="$style.experienceItemLocation">
          {{ props.item.location.name }}
          ·
          {{ props.item.location.type }}
        </span>
      </div>
    </div>
    <p :class="$style.experienceItemDescription">
      {{ props.item.description }}
    </p>
  </div>
</template>

<style module lang='scss'>
.experienceItemHeader {
  display: flex;
  align-items: center;
}

.experienceItemLogo {
  height: 100px;
  width: 100px;
  border-radius: 10px;
  object-fit: cover;
}

.experienceItemInfo {
  display: flex;
  flex-direction: column;
  gap: 5px;
  margin-left: 20px
}

.experienceItemOccupation {
  font-size: 18px;
  line-height: 23px;
  font-weight: 700;
  color: white;
}

.experienceItemDescription {
  margin-top: 20px;
}
</style>
