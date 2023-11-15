<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  label: string
  modelValue?: string
  value: string
  name: string
}>();

defineEmits<{
  (e: 'update:modelValue', value: string): void
}>();

const isChecked = computed(() => props.modelValue === props.value);
</script>

<template>
  <label class="wrapper">
    {{ label }}
    <input
      class="checkbox" type="radio" :checked="isChecked" :value="value" :name="name"
      @change="$emit('update:modelValue', ($event.target as HTMLInputElement)?.value)"
    >
    <span class="checkmark" />
  </label>
</template>

<style lang="postcss" scoped>
/* Customize the label (the wrapper) */
.wrapper {
  display: block;
  position: relative;
  padding-left: 30px;
  cursor: default;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

/* Hide the browser's default radio button */
.wrapper input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

/* Create a custom radio button */
.checkmark {
  @apply bg-main_bg border-black/50 dark:border-white/50 border-[1.5px];
  position: absolute;
  top: 0;
  left: 0;
  height: 20px;
  width: 20px;
  border-radius: 50%;
}

/* On mouse-over, add a grey background color */
.wrapper:hover input~.checkmark {
  @apply bg-highlight;
}

.wrapper:hover input:checked~.checkmark {
  @apply bg-passion border-passion;
}

/* When the radio button is checked, add a blue background */
.wrapper input:checked~.checkmark {
  @apply bg-passion border-passion;
}

/* Create the indicator (the dot/circle - hidden when not checked) */
.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}

/* Show the indicator (dot/circle) when checked */
.wrapper input:checked~.checkmark:after {
  display: block;
}

/* Style the indicator (dot/circle) */
.wrapper .checkmark:after {
  @apply bg-white dark:bg-black;
  top: 4px;
  left: 4px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.wrapper:hover .checkmark:after {
  top: 3px;
  left: 3px;
  width: 12px;
  height: 12px;
}
</style>
