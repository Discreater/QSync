<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { toggleDark } from '~/utils';

const { locale } = useI18n();
</script>

<template>
  <h1 class="text-center text-3xl font-bold underline">
    Hello World!
  </h1>
  <button @click="toggleDark()">
    toggle dark
  </button>
  <p class="block sm:hidden md:hidden">
    small
  </p>
  <p class="hidden sm:block md:hidden">
    middle
  </p>
  <p class="hidden sm:hidden md:block">
    large
  </p>
  <select v-model="locale">
    <option value="en">
      English
    </option>
    <option value="zh-CN">
      简体中文
    </option>
  </select>
</template>
