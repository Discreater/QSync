<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import Basic from '~/layouts/Basic.vue';
import { toggleDark } from '~/utils';
import { useMusyncStore } from '~/store';
import QSelect from '~/components/QSelect.vue';

const { t } = useI18n();
const store = useMusyncStore();

const langOptions = [
  { name: 'English', key: 'en' },
  { name: '简体中文', key: 'zh-CN' },
];

const breakPoints = useBreakpoints(breakpointsTailwind);
const current = breakPoints.current();
</script>

<template>
  <Basic :header="t('menu.home')">
    <button @click="toggleDark()">
      toggle dark
    </button>
    <p>
      {{ current }}
    </p>
    <QSelect v-model:value="store.locale" :options="langOptions" />
  </Basic>
</template>
