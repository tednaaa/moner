<script setup lang='ts'>
import { useField } from 'vee-validate';
import type { InputTypeHTMLAttribute } from 'vue';

const props = defineProps<{
  name: string,
  label: string
  type: InputTypeHTMLAttribute
}>()

const { value, errorMessage } = useField(props.name, undefined, { validateOnValueUpdate: false, });
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
  border: 1px solid #5F5F5F;
  border-radius: 10px;

  --input-side-offset: 15px;
}

.label {
  color: white;
  position: absolute;
  top: 50%;
  left: var(--input-side-offset);
  transform: translateY(-50%);
  border-radius: 6px;
  transition: 0.3s;
}

.input {
  padding: 10px var(--input-side-offset);
  outline: none;
  border-radius: 10px;
  color: white;

  &:-webkit-autofill,
  &:-webkit-autofill:hover,
  &:-webkit-autofill:focus,
  &:-webkit-autofill:active {
    -webkit-background-clip: text;
    -webkit-text-fill-color: white;
    box-shadow: none;
  }
}

.input:focus+.label,
.input:autofill+.label,
.input:-webkit-autofill+.label,
.labelActive {
  font-size: 13px;
  top: -2px;
  background-color: #1a1a1a;
  padding: 0 5px;
}

.errorMessage {
  font-size: 13px;
  font-weight: 500;
  line-height: 17px;
  color: var(--color-error);
  display: inline-block;
  margin-top: 4px;
  margin-left: 16px;
}
</style>
