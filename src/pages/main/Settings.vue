<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useColorMode } from '@vueuse/core';
import Basic from '~/layouts/Basic.vue';
import LongButton from '~/components/LongButton.vue';

import IconLanguage from '~icons/fluent/local-language-zi-24-regular';
import IconPaintBrush from '~icons/fluent/paint-brush-24-regular';
import QRadio from '~/components/QRadio.vue';
import { useMusyncStore } from '~/store';
import QSelect from '~/components/QSelect.vue';

const { t } = useI18n();

const { store: colorStore, system } = useColorMode();

const system_label = `${t('settings.theme_auto')} : ${t(`settings.theme_${system.value}`)}`;

const musyncStore = useMusyncStore();

const langOptions = [
  { name: 'English', key: 'en' },
  { name: '简体中文', key: 'zh-CN' },
];
</script>

<template>
  <Basic :header="t('menu.settings')">
    <LongButton :icon="IconPaintBrush" :droppable="true" :text="t('settings.application_theme')">
      <template #extra>
        {{ t(`settings.theme_${colorStore}`) }}
      </template>
      <template #drop>
        <div class="flex flex-col justify-center space-y-5">
          <QRadio v-model="colorStore" value="light" :label="t('settings.theme_light')" name="theme" />
          <QRadio v-model="colorStore" value="dark" :label="t('settings.theme_dark')" name="theme" />
          <QRadio v-model="colorStore" value="auto" :label="system_label" name="theme" />
        </div>
      </template>
    </LongButton>
    <LongButton :icon="IconLanguage" :text="t('settings.language')">
      <template #extra>
        <QSelect v-model:value="musyncStore.locale" :options="langOptions" />
      </template>
    </LongButton>
  </Basic>
</template>

<style></style>
