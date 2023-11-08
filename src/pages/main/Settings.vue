<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useColorMode } from '@vueuse/core';
import Basic from '~/layouts/Basic.vue';
import H2 from '~/components/typo/H2.vue';
import LongButton from '~/components/LongButton.vue';

import IconPaintBrush from '~icons/fluent/paint-brush-24-regular';

const { t } = useI18n();

const { store, system } = useColorMode();
</script>

<template>
  <Basic :header="t('menu.settings')">
    <H2>{{ t("settings.theme") }}</H2>
    <LongButton :icon="IconPaintBrush" :droppable="true" :text="t('settings.application_theme')">
      <template #extra>
        {{ t(`settings.theme_${store}`) }}
      </template>
      <template #drop>
        <div class="flex flex-col justify-center">
          <label for="theme-light">
            <input id="theme-light" v-model="store" type="radio" name="theme" value="light">
            {{ t('settings.theme_light') }}
          </label>
          <label for="theme-dark">
            <input id="theme-dark" v-model="store" type="radio" name="theme" value="dark">
            {{ t('settings.theme_dark') }}
          </label>
          <label for="theme-auto">
            <input id="theme-auto" v-model="store" type="radio" name="theme" value="auto">{{ t('settings.theme_auto') }} :
            {{ t(`settings.theme_${system}`) }}
          </label>
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
