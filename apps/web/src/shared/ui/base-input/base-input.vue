<script setup lang='ts'>
import { useField } from 'vee-validate';
import type { InputTypeHTMLAttribute } from 'vue';

const props = defineProps<{
  name: string,
  label: string
  type: InputTypeHTMLAttribute
}>()

const { value, errorMessage } = useField(() => props.name);
</script>

<template>
  <div :class="$style.wrapper">
    <label :class="$style.inputWrapper">
      <slot name="left"></slot>
      <input :class="$style.input" :type="props.type" v-model="value" />
      <span :class="[$style.label, value && $style.labelActive]">{{ props.label }}</span>

      <slot name="right"></slot>
    </label>
    <span v-if="errorMessage" :class="$style.errorMessage" :aria-errormessage="errorMessage">{{ errorMessage }}</span>
  </div>
</template>

<style module lang='scss'>
.wrapper {
  position: relative;
}

.inputWrapper {
  display: flex;
  position: relative;
  border: 1px solid var(--color-gray);
  border-radius: 10px;

  --input-side-offset: 15px;
}

.label {
  color: var(--color-gray);
  position: absolute;
  top: 50%;
  left: var(--input-side-offset);
  transform: translateY(-50%);
  z-index: 0;
  transition: 0.3s;
}

.input {
  z-index: 1;
  padding: 10px var(--input-side-offset);
  outline: none;
}

.input:focus+.label,
.labelActive {
  font-size: 14px;
  top: -2px;
  background-color: white;
  padding: 0 8px;
}

.errorMessage {
  font-size: 13px;
  font-weight: 500;
  line-height: 17px;
  color: var(--color-error);
  margin-top: 2px;
}
</style>
