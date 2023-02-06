<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import MenuItem from './MenuItem.vue';

const { t } = useI18n();
</script>

<template>
  <div class="flex flex-col bg-menu_w_bg dark:bg-menu_d_bg">
    <MenuItem icon="fluent:home-24-regular" :name="t('menu.home')" />
    <MenuItem icon="fluent:music-note-2-24-regular" :name="t('menu.musiclib')" />
    <div class="flex-1 w-full">
      &nbsp;
    </div>
    <MenuItem icon="fluent:settings-24-regular" :name="t('menu.settings')" />
  </div>
</template>
