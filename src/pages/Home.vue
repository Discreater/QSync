<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';
import { toggleDark } from '~/utils';
import H1 from '~/components/typo/H1.vue';

const { t, locale } = useI18n();
</script>

<template>
  <Basic>
    <H1>{{ t('menu.home') }}</H1>
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
  </Basic>
</template>
