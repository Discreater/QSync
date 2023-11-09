<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useColorMode } from '@vueuse/core';
import Basic from '~/layouts/Basic.vue';
import H2 from '~/components/typo/H2.vue';
import LongButton from '~/components/LongButton.vue';

import IconPaintBrush from '~icons/fluent/paint-brush-24-regular';
import QRadio from '~/components/QRadio.vue';

const { t } = useI18n();

const { store, system } = useColorMode();

const system_label = `${t('settings.theme_auto')} : ${t(`settings.theme_${system.value}`)}`;
</script>

<template>
  <Basic :header="t('menu.settings')">
    <H2>{{ t("settings.theme") }}</H2>
    <LongButton :icon="IconPaintBrush" :droppable="true" :text="t('settings.application_theme')">
      <template #extra>
        {{ t(`settings.theme_${store}`) }}
      </template>
      <template #drop>
        <div class="flex flex-col justify-center space-y-5">
          <QRadio v-model="store" value="light" :label="t('settings.theme_light')" name="theme" />
          <QRadio v-model="store" value="dark" :label="t('settings.theme_dark')" name="theme" />
          <QRadio v-model="store" value="auto" :label="system_label" name="theme" />
        </div>
      </template>
    </LongButton>
  </Basic>
</template>

<style>
input[type="radio"] {
  margin-right: 0.5rem;
  width: 1rem;
  height: 1rem;
  background-color: transparent;
}
</style>
