<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';
import { toggleDark } from '~/utils';
import H1 from '~/components/typo/H1.vue';
import { useQSyncStore } from '~/store';
import QSelect from '~/components/QSelect.vue';

const { t } = useI18n();
const store = useQSyncStore();

const langOptions = [
  { label: 'English', value: 'en' },
  { label: '简体中文', value: 'zh-CN' },
];
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
    <QSelect v-model:value="store.locale" :options="langOptions" />
  </Basic>
</template>
